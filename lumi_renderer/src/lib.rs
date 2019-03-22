#[macro_use]
extern crate luminance;

use luminance_glfw::surface::{GlfwSurface, Surface, WindowDim, WindowOpt};
use luminance_glfw::event::{Action, Key, WindowEvent};
use luminance::tess::{Mode, Tess, TessSlice};
use luminance::context::GraphicsContext;
use luminance::render_state::RenderState;
use luminance::framebuffer::Framebuffer;
use luminance::shader::program::Program;
use luminance::pipeline::BoundTexture;
use luminance::texture::{Dim2, Flat};
use luminance::vertex::Vertex;
use luminance::linear::M44;
use luminance::pixel::R32F;

use contrast::camera::Camera;
use contrast::markscontainer::Contrast;
use contrast::marks::pointmark::VertexPoint;
use contrast::marks::linemark::VertexSubLine;
use contrast::marks::textmark::VertexText;
use contrast::marks::mark::MarkTy;
use properties::markid::MarkId;
use std::collections::HashMap;
use std::iter;

const VSPOINT: &'static str = include_str!("../../src/shaders/point.vert");
const FSPOINT: &'static str = include_str!("../../src/shaders/point.frag");
const GSPOINT: &'static str = include_str!("../../src/shaders/point.geom");

const VSLINE: &'static str = include_str!("../../src/shaders/line.vert");
const FSLINE: &'static str = include_str!("../../src/shaders/line.frag");
const GSLINE: &'static str = include_str!("../../src/shaders/line.geom");

const VSTEXT: &'static str = include_str!("../../src/shaders/text.vert");
const FSTEXT: &'static str = include_str!("../../src/shaders/text.frag");

uniform_interface!
{
    pub struct ShaderInterface
    {
        projection: M44
    }
}

uniform_interface!
{
    pub struct ShaderTextInterface
    {
        atlas: &'static BoundTexture<'static, Flat, Dim2, R32F>,
        projection: M44
    }
}

const DUMMY_POINT: &'static VertexPoint = &([0.0, 0.0, -10.0], [0.0, 0.0], [0.0, 0.0, 0.0, 0.0], 0.0, 0u32, 0.0, 0.0);
const DUMMY_LINE: &'static VertexSubLine = &([0.0, 0.0], [0.0, 0.0, 0.0, 0.0], 0.0, [0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0, 0.0], 0.0, 0u32);
const DUMMY_TEXT: &'static VertexText = &([0.0, 0.0, 0.0, 0.0]);

pub struct TessPool<V>
{
    pub(crate) tess: Tess<V>,
    pub(crate) size: usize
}

impl<V> TessPool<V> where V: Vertex, V: std::marker::Copy
{
    pub fn new(ctx: &mut GlfwSurface, mode: Mode, dummy: V) -> TessPool<V>
    {
        let vertices: Vec<V> = iter::repeat(dummy).take(1000).collect();
        TessPool{tess: Tess::new(ctx, mode, &vertices[..], None), size: 0}
    }

    pub fn data(&self) -> TessSlice<V> { TessSlice::one_sub(&self.tess, self.size) }

    pub fn range(&self, from: usize, to: usize) -> TessSlice<V> { TessSlice::one_slice(&self.tess, from, to) }

    pub fn update(&mut self, vertices: Vec<V>)
    {
        self.size = vertices.len();
        let mut buffer = self.tess.as_slice_mut::<GlfwSurface>().unwrap();
        if buffer.len() < self.size { panic!("Tess is full") }
        buffer[..self.size].copy_from_slice(vertices.as_slice());
    }
}

pub struct RenderPass<V, U>
{
    pub(crate) pool: TessPool<V>,
    pub(crate) program: Program<V, (), U>
}

impl<V, U> RenderPass<V, U> where V: Vertex, V: std::marker::Copy
{
    pub fn shader(&self) -> &Program<V, (), U> { &self.program }
    pub fn vertices(&self) -> TessSlice<V> { self.pool.data() }
}

pub type RPoint = RenderPass<VertexPoint, ShaderInterface>;
pub type RLine = RenderPass<VertexSubLine, ShaderInterface>;
pub type RText = RenderPass<VertexText, ShaderTextInterface>;
pub type Frame = Framebuffer<Flat, Dim2, (), ()>;


enum Callback<'a> {
    NoArgument(fn()),
    ArgumentMark(fn(&mut Contrast, markid : &'a MarkId), &'a MarkId),
    ArgumentMarkList(fn(&mut Contrast, markids : &'a Vec<MarkId>), &'a Vec<MarkId>)
}

pub struct LumiRenderer<'a>
{
    contrast: Contrast,
    surface: GlfwSurface,
    frame: Frame,
    point: RPoint,
    line: RLine,
    text: RText,
    cam: Camera,
    callbacks : HashMap<Key, Callback<'a>>
}

impl<'a> LumiRenderer<'a>
{
    pub fn init(w: u32, h: u32, title: &str) -> LumiRenderer
    {
        let mut surface = GlfwSurface::new(WindowDim::Windowed(w, h), title, WindowOpt::default()).expect("GLFW ERROR");
        let frame = Framebuffer::back_buffer(surface.size());

        let shd = Program::<VertexPoint, (), ShaderInterface>::from_strings(None, VSPOINT, GSPOINT, FSPOINT).expect("program creation");
        let tss = TessPool::new(&mut surface, Mode::Point, DUMMY_POINT.clone());
        let point = RPoint{pool: tss, program: shd.0};

        let shd = Program::<VertexSubLine, (), ShaderInterface>::from_strings(None, VSLINE, GSLINE, FSLINE).expect("program creation");
        let tss = TessPool::new(&mut surface, Mode::Point, DUMMY_LINE.clone());
        let line = RLine{pool: tss, program: shd.0};

        let shd = Program::<VertexText, (), ShaderTextInterface>::from_strings(None, VSTEXT, None, FSTEXT).expect("program creation");
        let tss = TessPool::new(&mut surface, Mode::Triangle, DUMMY_TEXT.clone());
        let text = RText{pool: tss, program: shd.0};

        let mut contrast = Contrast::new();
        contrast.init();

        let cam = Camera::init(w, h);
        let callbacks = HashMap::new();

        LumiRenderer{contrast, surface, frame, point, line, text, cam, callbacks }
    }

    pub fn get_contrast_mut(&mut self) -> &mut Contrast
    {
        &mut self.contrast
    }

    pub fn add_action_on_press(&mut self, key : Key, f: fn()) {
        self.callbacks.insert(key, Callback::NoArgument(f));
    }

    pub fn add_mark_action_on_press(&mut self, key : Key, f: fn(&mut Contrast, &'a MarkId), markid : &'a MarkId) {
        self.callbacks.insert(key, Callback::ArgumentMark(f, markid));
    }

    pub fn add_mark_list_action_on_press(&mut self, key : Key, f: fn(&mut Contrast, &'a Vec<MarkId>), markids : &'a Vec<MarkId>) {
        self.callbacks.insert(key, Callback::ArgumentMarkList(f, markids));
    }

    pub fn run(&mut self)
    {
        'app: loop
        {
            for event in self.surface.poll_events()
            {
                match event
                {
                    WindowEvent::Close | WindowEvent::Key(Key::Escape, _, Action::Release, _) =>
                    {
                        break 'app
                    }

                    WindowEvent::FramebufferSize(width, height) =>
                    {
                        self.frame = Framebuffer::back_buffer([width as u32, height as u32]);
                        self.cam.resize(width, height);
                    }

                    WindowEvent::Key(k, _, action, _) if action == Action::Press || action == Action::Repeat =>
                    {
                        for (key, callback) in self.callbacks.iter_mut() {
                            if *key == k {
                                match &callback {
                                    Callback::NoArgument(f) => { f() }
                                    Callback::ArgumentMark(f, markid) => { f(&mut self.contrast, *markid) }
                                    Callback::ArgumentMarkList(f, markids) => { f(&mut self.contrast, markids) }
                                }
                            }
                        }

                    }

                    _ => ()
                }
            }

            for ty in self.contrast.fetch_update()
            {
                match ty
                {
                    MarkTy::Point => self.point.pool.update(self.contrast.get_pointmarks_properties()),
                    MarkTy::Line => self.line.pool.update(self.contrast.get_linemarks_properties()),
                    _ => println!("TessPool not implemented yet")
                }
            }

            let p = &self.point;
            let l = &self.line;

            let mat = self.cam.data();
            let ctx = &mut self.surface;
            let back_buffer = &self.frame;

            ctx.pipeline_builder().pipeline(back_buffer, [0., 0., 0., 0.], |_, shd_gate|
            {
                shd_gate.shade(p.shader(), |rdr_gate, iface|
                {
                    iface.projection.update(mat);
                    rdr_gate.render(RenderState::default(), |tess_gate|
                    {
                        tess_gate.render(ctx, p.vertices());
                    });
                });
                shd_gate.shade(l.shader(), |rdr_gate, iface|
                {
                    iface.projection.update(mat);
                    rdr_gate.render(RenderState::default(), |tess_gate|
                    {
                        tess_gate.render(ctx, l.vertices());
                    });
                });
            });

            self.surface.swap_buffers();
        }
    }
}
