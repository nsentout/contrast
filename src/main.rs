#[macro_use]
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
use luminance::linear::M44;

use contrast::markscontainer::Contrast;
use contrast::pointmark::Shape;
use contrast::pointmark::VertexPoint;
use contrast::linemark::LineMode;
use contrast::camera::Camera;

const VSPOINT: &'static str = include_str!("shaders/point.vert");
const FSPOINT: &'static str = include_str!("shaders/point.frag");
const GSPOINT: &'static str = include_str!("shaders/point.geom");

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

    let mut cam = Camera::init(800, 800);

    // Build some marks
         // TODO : faire en sorte que les marks soient modifiables à postériori de leur création
    let mark_1 = {
        contrast.add_point_mark().set_position(100.0, 100.0, 0.0).set_size(0.1, 0.1).set_color(0.0, 0.0, 1.0, 1.0).set_shape(Shape::Rectangle);
    };

    let mark_2 = {
        contrast.add_point_mark().set_position(200.0, 100.0, 0.0).set_size(0.1, 0.1).set_color(0.0, 1.0, 0.0, 1.0).set_shape(Shape::Rectangle);
    };

    let mark_3 = {
        contrast.add_point_mark().set_position(400.0, 400.0, 1.0).set_size(0.1, 0.1).set_color(1.0, 0.0, 0.0, 1.0).set_shape(Shape::Triangle);
    };

    let mark_4 = {
        contrast.add_point_mark().set_position(400.0, 400.0, 0.0).set_size(0.5, 0.2).set_color(0.0, 1.0, 1.0, 1.0).set_shape(Shape::Triangle);
    };

    let mark_5 = {  // not displayed, lines are not handled at the moment
        contrast.add_line_mark().set_thickness(5.0).set_mode(LineMode::Linear);
    };


    let mut surface = GlfwSurface::new(WindowDim::Windowed(800, 800), "Hello, world!", WindowOpt::default()).expect("GLFW surface creation");

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