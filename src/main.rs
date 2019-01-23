extern crate luminance;
extern crate luminance_glfw;

use luminance::framebuffer::Framebuffer;
use luminance::shader::program::Program;
use luminance::tess::{Mode, Tess};
use luminance::render_state::RenderState;
use luminance_glfw::event::{Action, Key, WindowEvent};
use luminance_glfw::surface::{GlfwSurface, Surface, WindowDim, WindowOpt};
use luminance::context::GraphicsContext;

// we get the shaders at compile time 
const VS: &'static str = include_str!("shaders/mark-vs.glsl");
const FS: &'static str = include_str!("shaders/mark-fs.glsl");
const GS: &'static str = include_str!("shaders/mark-gs.glsl");
//const VS: &'static str = include_str!("shaders/prototype/marks.vert");
//const FS: &'static str = include_str!("shaders/prototype/marks.frag");
//const GS: &'static str = include_str!("shaders/prototype/marks.geom");

// 2D position ([f32; 2]), a RGB color ([f32; 3]) and a size ([f32; 2])
type Vertex = ([f32; 2], [f32; 2], [f32; 3], u32);

const WIDTH_RECT: f32 = 0.1;
const HEIGHT_RECT: f32 = 0.2;

const TRI_VERTICES: [Vertex; 9] = [
    ([-0.5, 0.5], [WIDTH_RECT, HEIGHT_RECT], [1., 0., 0.], 1),
    ([ 0.0, 0.5], [WIDTH_RECT, HEIGHT_RECT], [0., 1., 0.], 2),
    ([ 0.5, 0.5], [WIDTH_RECT, HEIGHT_RECT], [0., 0., 1.], 3),
    ([-0.5, 0.0], [WIDTH_RECT, HEIGHT_RECT], [1., 0., 1.], 4),
    ([ 0.0, 0.0], [WIDTH_RECT, HEIGHT_RECT], [1., 1., 0.], 5),
    ([ 0.5, 0.0], [WIDTH_RECT, HEIGHT_RECT], [0., 1., 1.], 6),
    ([-0.5,-0.5], [WIDTH_RECT, HEIGHT_RECT], [1., 0., 0.], 7),
    ([ 0.0,-0.5], [WIDTH_RECT, HEIGHT_RECT], [0., 1., 0.], 8),
    ([ 0.5,-0.5], [WIDTH_RECT, HEIGHT_RECT], [0., 0., 1.], 9)
];

fn main()
{
    let mut surface = GlfwSurface::new(WindowDim::Windowed(400, 400), "Hello, world!", WindowOpt::default()).expect("GLFW surface creation");

    let (program, _) = Program::<Vertex, (), ()>::from_strings(None, VS, GS, FS).expect("program creation");

    let tess = Tess::new(&mut surface, Mode::Point, &TRI_VERTICES[..], None);

    let mut back_buffer = Framebuffer::back_buffer(surface.size());

    'app: loop
    {
        // for all the events on the surface
        for event in surface.poll_events()
        {
            match event
            {
                WindowEvent::Close | WindowEvent::Key(Key::Escape, _, Action::Release, _) =>
                {
                    break 'app
                }

                WindowEvent::Key(Key::Space, _, Action::Release, _) => {}

                WindowEvent::FramebufferSize(width, height) =>
                {
                    back_buffer = Framebuffer::back_buffer([width as u32, height as u32]);
                }

                _ => ()
            }
        }

        surface.pipeline_builder().pipeline(&back_buffer, [0., 0., 0., 0.], |_, shd_gate|
        {
            shd_gate.shade(&program, |rdr_gate, _|
            {
                rdr_gate.render(RenderState::default(), |tess_gate|
                {
                    let tess = &tess;
                    tess_gate.render(&mut surface, tess.into());
                });
            });
        });

        surface.swap_buffers();
    }
}