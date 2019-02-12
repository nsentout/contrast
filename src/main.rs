#[macro_use]
extern crate luminance;
extern crate luminance_glfw;
extern crate contrast;
extern crate rand;

use luminance::framebuffer::Framebuffer;
use luminance::shader::program::Program;
use luminance::tess::{Mode, Tess};
use luminance::render_state::RenderState;
use luminance_glfw::event::{Action, Key, WindowEvent};
use luminance_glfw::surface::{GlfwSurface, Surface, WindowDim, WindowOpt};
use luminance::context::GraphicsContext;
use luminance::linear::M44;

use contrast::markscontainer::Contrast;
use contrast::pointmark::Shape;
use contrast::pointmark::VertexPoint;
use contrast::linemark::LineMode;
use contrast::camera::Camera;

use rand::Rng;

const VSPOINT: &'static str = include_str!("shaders/point.vert");
const FSPOINT: &'static str = include_str!("shaders/point.frag");
const GSPOINT: &'static str = include_str!("shaders/point.geom");
const WINDOW_WIDTH : u32 = 800;
const WINDOW_HEIGHT : u32 = 800;

uniform_interface!
{
    struct ShaderInterface
    {
        projection: M44
    }
}

fn main()
{
    // Initialize contrast
    let mut contrast = Contrast::init();

    // Initialize the camera
    let mut cam = Camera::init(WINDOW_WIDTH, WINDOW_HEIGHT);

    // Build some marks
         // TODO : faire en sorte que les marks soient modifiables à postériori de leur création
    let mut rng = rand::thread_rng();

    println!("Building marks ...");
    for _ in 0..100_000 {
        contrast.add_point_mark().set_position(rng.gen_range::<f32>(0.0, WINDOW_WIDTH as f32), rng.gen_range::<f32>(0.0, WINDOW_HEIGHT as f32), 0.0)
            .set_size(0.01, 0.01)
            .set_color(rng.gen_range::<f32>(0.0, 1.0), rng.gen_range::<f32>(0.0, 1.0), rng.gen_range::<f32>(0.0, 1.0), 1.0)
            .set_shape(Shape::Triangle);
    }

    println!("Building finished!");

    let mark_line = {  // not displayed, lines are not handled at the moment
        contrast.add_line_mark().set_thickness(5.0).set_mode(LineMode::Linear).get_id()
    };

    contrast.remove_line_mark(mark_line);

    println!("Rendering ...");

    let mut surface = GlfwSurface::new(WindowDim::Windowed(WINDOW_WIDTH, WINDOW_HEIGHT), "contrast playground", WindowOpt::default()).expect("GLFW surface creation");

    let (program, _) = Program::<VertexPoint, (), ShaderInterface>::from_strings(None, VSPOINT, GSPOINT, FSPOINT).expect("program creation");

    let tess = Tess::new(&mut surface, Mode::Point, &contrast.get_pointmarks_properties()[..], None);

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

                WindowEvent::Size(width, height) =>
                {
                    cam.resize(width, height);
                }

                _ => ()
            }
        }

        surface.pipeline_builder().pipeline(&back_buffer, [0., 0., 0., 0.], |_, shd_gate|
        {
            shd_gate.shade(&program, |rdr_gate, iface|
            {
                iface.projection.update(cam.data());
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
