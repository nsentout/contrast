#[macro_use]
extern crate luminance;

use luminance_glfw::surface::{GlfwSurface, Surface, WindowDim, WindowOpt};
use luminance_glfw::event::{Action, WindowEvent};
use luminance::tess::{Mode, Tess, TessSlice};
use luminance::context::GraphicsContext;
use luminance::render_state::RenderState;
use luminance::framebuffer::Framebuffer;
use luminance::shader::program::Program;
use luminance::pipeline::BoundTexture;
use luminance::texture::{Dim2, Flat, Sampler, Texture};
use luminance::blending::{Equation, Factor};
use luminance::vertex::Vertex;
use luminance::linear::M44;
use luminance::pixel::R32F;

use contrast::elapsed_time_float;
use contrast::camera::Camera;
use contrast::markscontainer::Contrast;
use contrast::marks::pointmark::VertexPoint;
use contrast::marks::linemark::VertexSubLine;
use contrast::marks::textmark::VertexText;
use contrast::marks::textmark::TextMarkCmd;
use contrast::marks::textmark::Glyph;
use contrast::marks::mark::MarkTy;
use contrast_properties::markid::MarkId;

use std::collections::LinkedList;
use std::collections::HashMap;
use std::iter;
use std::time::Instant;

pub use luminance_glfw::event::Key;

const VSPOINT: &'static str = include_str!("../../contrast/src/shaders/point/point.vert");
const FSPOINT: &'static str = include_str!("../../contrast/src/shaders/point/point.frag");
const GSPOINT: &'static str = include_str!("../../contrast/src/shaders/point/point.geom");

const VSLINE: &'static str = include_str!("../../contrast/src/shaders/line/line.vert");
const FSLINE: &'static str = include_str!("../../contrast/src/shaders/line/line.frag");
const GSLINE: &'static str = include_str!("../../contrast/src/shaders/line/line.geom");

const VSTEXT: &'static str = include_str!("../../contrast/src/shaders/text/text.vert");
const FSTEXT: &'static str = include_str!("../../contrast/src/shaders/text/text.frag");

uniform_interface!
{
    pub struct ShaderInterface
    {
        projection: M44
    }
}

uniform_interface!
{
    pub struct ShaderPointInterface
    {
        #[as("t")]
        time: f32,
        projection: M44
    }
}

uniform_interface!
{
    pub struct ShaderTextInterface
    {
        atlas: &'static BoundTexture<'static, Flat, Dim2, R32F>,
        projection: M44,
        color: [f32; 4]
    }
}

const DUMMY_POINT: &'static VertexPoint = &([0.0, 0.0, -10.0], [0.0, 0.0, -10.0], 0.0, [0.0, 0.0], [0.0, 0.0], 0.0,
                                            [0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0], 0.0, 0.0, 0.0, 0.0, 0u32, 0u32, 0.0);
const DUMMY_LINE: &'static VertexSubLine = &([0.0, 0.0], [0.0, 0.0, 0.0, 0.0], 0.0, [0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0, 0.0], 0.0, 0u32);
const DUMMY_TEXT: &'static VertexText = &([0.0, 0.0, 0.0], [0.0, 0.0]);

pub struct TessPool<V>
{
    pub(crate) tess: Tess<V>,
    pub(crate) size: usize,
    pub(crate) mode: Mode,
    pub(crate) zero: V
}

impl<V> TessPool<V> where V: Vertex, V: std::marker::Copy
{
    pub fn new(ctx: &mut GlfwSurface, mode: Mode, dummy: V) -> TessPool<V>
    {
        let vertices: Vec<V> = iter::repeat(dummy).take(1000).collect();
        TessPool{tess: Tess::new(ctx, mode, &vertices[..], None), size: 0, mode, zero: dummy}
    }

    pub fn resize(&mut self, ctx: &mut GlfwSurface, reserve: usize)
    {
        let vertices: Vec<V> = iter::repeat(self.zero).take(reserve).collect();
        self.tess = Tess::new(ctx, self.mode, &vertices[..], None);
    }

    pub fn data(&self) -> TessSlice<V> { TessSlice::one_sub(&self.tess, self.size) }

    pub fn range(&self, from: usize, to: usize) -> TessSlice<V> { TessSlice::one_slice(&self.tess, from, to) }

    pub fn update(&mut self, ctx: &mut GlfwSurface, vertices: Vec<V>)
    {

        self.size = vertices.len();
        let len = { self.tess.as_slice().unwrap().len() };
        if len < self.size
        {
            let vertices: Vec<V> = iter::repeat(self.zero).take(self.size+1000).collect();
            self.tess = Tess::new(ctx, self.mode, &vertices[..], None);
        }
        let mut buffer = self.tess.as_slice_mut::<GlfwSurface>().unwrap();
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
    pub fn vertices_range(&self, from: usize, to: usize) -> TessSlice<V> { self.pool.range(from, to) }
}

pub type RPoint = RenderPass<VertexPoint,ShaderPointInterface>;
pub type RLine = RenderPass<VertexSubLine,ShaderInterface>;
pub type RText = RenderPass<VertexText,ShaderTextInterface>;
pub type Frame = Framebuffer<Flat,Dim2,(),()>;
pub type Atlas = Texture<Flat,Dim2,R32F>;

enum Callback<'a> {
    NoArgument(fn(&mut Contrast)),
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
    callbacks : HashMap<Key, Callback<'a>>,
    font_atlas: HashMap<String,Atlas>,
    font_cmmds: LinkedList<TextMarkCmd>
}

impl<'a> LumiRenderer<'a>
{
    pub fn init(w: u32, h: u32, title: &str) -> LumiRenderer
    {
        let mut surface = GlfwSurface::new(WindowDim::Windowed(w, h), title, WindowOpt::default()).expect("GLFW ERROR");
        let frame = Framebuffer::back_buffer(surface.size());

        let shd = Program::<VertexPoint, (), ShaderPointInterface>::from_strings(None, VSPOINT, GSPOINT, FSPOINT).expect("program creation");
        let tss = TessPool::new(&mut surface, Mode::Point, DUMMY_POINT.clone());
        let point = RPoint{pool: tss, program: shd.0};

        let shd = Program::<VertexSubLine, (), ShaderInterface>::from_strings(None, VSLINE, GSLINE, FSLINE).expect("program creation");
        let tss = TessPool::new(&mut surface, Mode::Point, DUMMY_LINE.clone());
        let line = RLine{pool: tss, program: shd.0};

        let shd = Program::<VertexText, (), ShaderTextInterface>::from_strings(None, VSTEXT, None, FSTEXT).expect("program creation");
        let tss = TessPool::new(&mut surface, Mode::Triangle, DUMMY_TEXT.clone());
        let text = RText{pool: tss, program: shd.0};

        let contrast = Contrast::new();

        let cam = Camera::init(w, h);
        let callbacks = HashMap::new();
        let font_atlas = HashMap::new();
        let font_cmmds = LinkedList::new();

        LumiRenderer{contrast, surface, frame, point, line, text, cam, callbacks, font_atlas, font_cmmds}
    }

    fn update_font_atlas(&mut self, glyphs: LinkedList<Glyph>)
    {
        for glyph in glyphs
        {
            if !self.font_atlas.contains_key(&glyph.name)
            {
                let tex = Texture::new(&mut self.surface, [1024, 1024], 0, &Sampler::default()).expect("luminance texture creation");
                self.font_atlas.insert(glyph.name.clone(), tex);
            }

            let atlas = self.font_atlas.get_mut(&glyph.name).unwrap();

            let x = glyph.rect.x as u32;
            let y = glyph.rect.y as u32;
            let w = glyph.rect.width as u32;
            let h = glyph.rect.height as u32;

            atlas.upload_part(false, [x, y], [w, h], glyph.bitmap.as_slice());
        }
    }

    fn build_text_marks(&mut self, bundle: (Vec<VertexText>,LinkedList<TextMarkCmd>,LinkedList<Glyph>))
    {
        self.text.pool.update(&mut self.surface, bundle.0);
        self.font_cmmds.clear();
        self.font_cmmds.extend(bundle.1);
        self.update_font_atlas(bundle.2);
    }

    pub fn get_contrast_mut(&mut self) -> &mut Contrast
    {
        self.contrast.init();
        &mut self.contrast
    }

    pub fn add_action_on_press(&mut self, key : Key, f: fn(&mut Contrast)) {
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
        let mut time = Instant::now();
        let mut frames = 0;
        let mut elapsed;

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
                                    Callback::NoArgument(f) => { f(&mut self.contrast) }
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
                    MarkTy::Point => self.point.pool.update(&mut self.surface, self.contrast.get_pointmarks_properties()),
                    MarkTy::Line => self.line.pool.update(&mut self.surface, self.contrast.get_linemarks_properties()),
                    MarkTy::Text => { let b = self.contrast.get_textmarks_properties(); self.build_text_marks(b); }
                }
            }

            let p = &self.point;
            let l = &self.line;
            let t = &self.text;

            let mat = self.cam.data();
            let ctx = &mut self.surface;
            let back_buffer = &self.frame;

            let commands = &self.font_cmmds;
            let textures = &self.font_atlas;
            let blending = Some((Equation::Additive, Factor::SrcAlpha, Factor::SrcAlphaComplement));

            elapsed = time.elapsed();
            frames += 1;

            if elapsed.as_secs() >= 1 {
                println!("FPS : {}", frames);
                time = Instant::now();
                frames = 0;
            }

            ctx.pipeline_builder().pipeline(back_buffer, [0., 0., 0., 0.], |pipeline, shd_gate|
            {
                shd_gate.shade(p.shader(), |rdr_gate, iface|
                {
                    iface.time.update(elapsed_time_float());
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
                for cmd in commands
                {
                    let tex = textures.get(&cmd.name).unwrap();
                    let bound_tex = pipeline.bind_texture(tex);
                    shd_gate.shade(t.shader(), |rdr_gate, iface|
                    {
                        iface.projection.update(mat);
                        iface.atlas.update(&bound_tex);
                        iface.color.update(cmd.color.to_array().clone());
                        rdr_gate.render(RenderState::default().set_blending(blending), |tess_gate|
                        {
                            tess_gate.render(ctx, t.vertices_range(cmd.start, cmd.end));
                        });
                    });
                }                
            });

            self.surface.swap_buffers();
        }
    }
}
