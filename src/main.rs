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

// Path of the shaders
const VSPOINT: &'static str = include_str!("shaders/point.vert");
const FSPOINT: &'static str = include_str!("shaders/point.frag");
const GSPOINT: &'static str = include_str!("shaders/point.geom");

// Window dimensions
const WINDOW_WIDTH : u32 = 800;
const WINDOW_HEIGHT : u32 = 800;


// Creates a uniform interface. This is a type that will be used to customize the shader.
// For the moment, we just pass the projection of the camera
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
    let mut contrast = Contrast::new();

    // Initialize the camera
    let mut cam = Camera::init(WINDOW_WIDTH, WINDOW_HEIGHT);

    // Build 100 000 point marks with random positions and random colors
    let mut rng = rand::thread_rng();    // TODO : faire en sorte que les marks soient modifiables à postériori de leur création

    println!("Building marks ...");
    for _ in 0..100_000 {
        contrast.add_point_mark().set_position(rng.gen_range::<f32>(0.0, WINDOW_WIDTH as f32), rng.gen_range::<f32>(0.0, WINDOW_HEIGHT as f32), 0.0)
            .set_size(0.01, 0.01)
            .set_color(rng.gen_range::<f32>(0.0, 1.0), rng.gen_range::<f32>(0.0, 1.0), rng.gen_range::<f32>(0.0, 1.0), 1.0)
            .set_shape(Shape::Triangle);
    }

    println!("Building finished!");

    // Add a line mark for testing
    let mark_line = {  // not displayed, lines are not handled at the moment
        contrast.add_line_mark().set_thickness(5.0).set_mode(LineMode::Linear).get_id()
    };

    // Remove a line mark for testing
    contrast.remove_line_mark(mark_line);


    println!("Rendering ...");

    // Create a new surface to render to and get events from
    let mut surface = GlfwSurface::new(WindowDim::Windowed(WINDOW_WIDTH, WINDOW_HEIGHT), "contrast playground", WindowOpt::default()).expect("GLFW surface creation");

    // We need a program to “shade” our triangles and to tell luminance which is the input vertex type
    let (program, _) = Program::<VertexPoint, (), ShaderInterface>::from_strings(None, VSPOINT, GSPOINT, FSPOINT).expect("program creation");

    // Create tessellation for direct geometry; that is, tessellation that will render vertices by taking one after another in the provided slice
    let tess = Tess::new(&mut surface, Mode::Point, &contrast.get_pointmarks_properties()[..], None);

    // The back buffer, which we will make our render into (we make it mutable so that we can change it whenever the window dimensions change)
    let mut back_buffer = Framebuffer::back_buffer(surface.size());

    'app: loop
    {
        // For all the events on the surface
        for event in surface.poll_events()
        {
            match event
            {
                // If we close the window or press escape, quit the main loop (i.e. quit the application)
                WindowEvent::Close | WindowEvent::Key(Key::Escape, _, Action::Release, _) =>
                {
                    break 'app
                }

                // Handle window resizing
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

        // Create a new dynamic pipeline that will render to the back buffer and must clear it with
        // pitch black prior to do any render to it
        surface.pipeline_builder().pipeline(&back_buffer, [0., 0., 0., 0.], |_, shd_gate|
        {
            // Start shading with our program
            shd_gate.shade(&program, |rdr_gate, iface|
            {
                iface.projection.update(cam.data());
                // Start rendering things with the default render state provided by luminance
                rdr_gate.render(RenderState::default(), |tess_gate|
                {
                    let tess = &tess;
                    // Render the tessellation to the surface
                    tess_gate.render(&mut surface, tess.into());
                });
            });
        });

        // Finally, swap the backbuffer with the frontbuffer in order to render our marks onto the screen
        surface.swap_buffers();
    }
}
