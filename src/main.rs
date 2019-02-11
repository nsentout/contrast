extern crate luminance;
extern crate luminance_glfw;
extern crate contrast;

use luminance::framebuffer::Framebuffer;
use luminance::shader::program::Program;
use luminance::tess::{Mode, Tess};
use luminance::render_state::RenderState;
use luminance_glfw::event::{Action, Key, WindowEvent};
use luminance_glfw::surface::{GlfwSurface, Surface, WindowDim, WindowOpt};
use luminance::context::GraphicsContext;

use contrast::properties::Contrast;
use contrast::pointmark::Shape;
use contrast::pointmark::VertexPoint;
use contrast::linemark::LineMode;


const VSPOINT: &'static str = include_str!("shaders/point.vertex");
const FSPOINT: &'static str = include_str!("shaders/point.fragment");
//const GSPOINT: &'static str = include_str!("shaders/mark-gs.glsl");

const VERTEXPOINTTAB: [VertexPoint; 3] = [
    ([-0.5, 0.5, 1.0], [0.01, 0.01], [0.0, 0.0, 1.0, 1.0], 0.0, 0,  0.0, 0.0),
    ([ 0.0, 0.5, 1.0], [0.01, 0.01], [0.0, 1.0, 0.0, 1.0], 0.0, 0,  0.0, 0.0),
    ([ 0.5, 0.5, 1.0], [0.01, 0.01], [1.0, 0.0, 0.0, 1.0], 0.0, 0,  0.0, 0.0),
];


fn main()
{
    // Initialize contrast
    let mut contrast = Contrast::init();

    // Build some marks
         // TODO : faire en sorte que les marks soient modifiables à postériori de leur création
    let mark_1 = {
        contrast.add_point_mark().set_position(1.0, 3.0, 5.0).set_shape(Shape::Triangle);
    };
    dbg!(&mark_1);

    let mark_2 = {
        contrast.add_line_mark().set_thickness(5.0).set_mode(LineMode::Dotted);
    };
    dbg!(&mark_2);


    let mut surface = GlfwSurface::new(WindowDim::Windowed(800, 800), "Hello, world!", WindowOpt::default()).expect("GLFW surface creation");

    let (program, _) = Program::<VertexPoint, (), ()>::from_strings(None, VSPOINT, None, FSPOINT).expect("program creation");

    let tess = Tess::new(&mut surface, Mode::Point, &VERTEXPOINTTAB[..], None);

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
