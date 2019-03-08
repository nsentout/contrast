#[macro_use]
extern crate luminance;

use luminance::framebuffer::Framebuffer;
use luminance::shader::program::Program;
use luminance::tess::{Mode, Tess};
use luminance::render_state::RenderState;
use luminance_glfw::event::{Action, Key, WindowEvent};
use luminance_glfw::surface::{GlfwSurface, Surface, WindowDim, WindowOpt};
use luminance::context::GraphicsContext;
use luminance::linear::M44;

use contrast::markscontainer::Contrast;
use contrast::marks::mark::Mark;
use contrast::marks::pointmark::Shape;
use contrast::marks::pointmark::VertexPoint;
use contrast::camera::Camera;
use contrast::MarkMacro;
use properties::position::Position;
use properties::size::Size;

use rand::Rng;

// Path to the shaders
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
    let mut rng = rand::thread_rng();

    println!("Building marks ...");
    /*for _ in 0..100_000 {
        contrast.add_point_mark().set_position((rng.gen_range::<f32>(0.0, WINDOW_WIDTH as f32), rng.gen_range::<f32>(0.0, WINDOW_HEIGHT as f32), 0.0))
            .set_size((8.0, 8.0))
            .set_color((rng.gen_range::<f32>(0.0, 1.0), rng.gen_range::<f32>(0.0, 1.0), rng.gen_range::<f32>(0.0, 1.0), 1.0))
            .set_shape(Shape::Triangle);
    }*/

    let pos = Position { x : 150.0, y : WINDOW_HEIGHT as f32 / 2.0, z : 0.0 };
    let size = Size { width : 200.0, height : 200.0 };

    let mark_triangle = contrast.add_point_mark().set_position(pos)
        .set_size(size)
        .set_color((1.0, 0.0, 0.0, 1.0))
        .set_shape(Shape::Triangle)
        .get_id();

    let mark_infinity = contrast.add_point_mark().set_position((pos.x + 250.0, pos.y, pos.z))
        .set_size(size)
        .set_color((0.0, 1.0, 0.0, 1.0))
        .set_shape(Shape::Infinity)
        .get_id();

    let mark_point = contrast.add_point_mark().set_position((pos.x + 500.0, pos.y, pos.z))
        .set_size(size)
        .set_color((0.0, 0.0, 1.0, 1.0))
        .set_shape(Shape::Point)
        .get_id();

    let mark_spade = contrast.add_point_mark().set_position((pos.x + 500.0, pos.y+250.0, pos.z))
        .set_size(size)
        .set_color((1.0, 0.0, 1.0, 1.0))
        .set_shape(Shape::Spade)
        .get_id();

    let mark_clover = contrast.add_point_mark().set_position((pos.x + 250.0, pos.y+250.0, pos.z))
        .set_size(size)
        .set_color((1.0, 1.0, 1.0, 1.0))
        .set_shape(Shape::Clover)
        .get_id();

    let mark_ring = contrast.add_point_mark().set_position((pos.x , pos.y+250.0, pos.z))
        .set_size(size)
        .set_color((1.0, 1.0, 0.0, 1.0))
        .set_shape(Shape::Ring)
        .get_id();

    let mark_tag = contrast.add_point_mark().set_position((pos.x, pos.y -250.0, pos.z))
        .set_size(size)
        .set_color((1.0, 0.5, 0.0, 1.0))
        .set_shape(Shape::Tag)
        .get_id();

    let mark_cross = contrast.add_point_mark().set_position((pos.x + 250.0, pos.y -250.0, pos.z))
        .set_size(size)
        .set_color((0.0, 1.0, 1.0, 1.0))
        .set_shape(Shape::Cross)
        .get_id();

    let mark_asterisk = contrast.add_point_mark().set_position((pos.x + 500.0, pos.y -250.0, pos.z))
        .set_size(size)
        .set_color((0.0, 0.5, 1.0, 1.0))
        .set_shape(Shape::Asterisk)
        .get_id();


    contrast.add_layer();
    contrast.add_layer();

    let m1 = contrast.add_point_mark().set_position((pos.x + 230.0, pos.y, pos.z))
        .set_size((100.0, 100.0))
        .set_color((1.0, 0.0, 0.0, 1.0))
        .set_shape(Shape::Rectangle)
        .get_id();

    let m2 = contrast.add_point_mark().set_position((pos.x + 250.0, pos.y + 10.0, pos.z))
        .set_size((100.0, 100.0))
        .set_color((1.0, 0.5, 0.0, 1.0))
        .set_shape(Shape::Rectangle)
        .get_id();

    let m3 = contrast.add_point_mark().set_position((pos.x + 280.0, pos.y + 20.0, pos.z))
        .set_size((100.0, 100.0))
        .set_color((1.0, 1.0, 0.0, 1.0))
        .set_shape(Shape::Rectangle)
        .get_id();

    contrast.set_mark_layer(&m1, 0);
    contrast.set_mark_layer(&m2, 1);
    //contrast.set_mark_layer(&m3, 2);    // TODO: m3 est invalide, corriger

    println!("Building finished!");
    println!("Rendering ...");

    dbg!(&contrast.get_mark_mut(&mark_infinity).unwrap());
    contrast.remove_mark(&mark_infinity);
    dbg!(&contrast.get_mark_mut(&mark_infinity).unwrap());      // TODO: devrait renvoyer None

    let layer_0 = contrast.get_layer_mut(0).unwrap();
    layer_0.set_position((50.0, 0.0, 0.0));

    let mark_ring_ptr : *mut Mark = contrast.get_mark_mut(&mark_ring).unwrap();

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

                WindowEvent::Key(Key::Space, _, Action::Release, _) =>
                {
                    unsafe {
                        // Change the color of the rectangle mark   //TODO: update la fenetre avec la nouvelle couleur
                        //dbg!(&(*mark_ring_ptr));
                        (*mark_ring_ptr).set_color((rng.gen_range::<f32>(0.0, 1.0), rng.gen_range::<f32>(0.0, 1.0), rng.gen_range::<f32>(0.0, 1.0), 1.0));
                        let color = (*mark_ring_ptr).get_color();
                        println!("Mark ring color : ({:.2}, {:.2}, {:.2}, {:.2})", color.r, color.g, color.b, color.a);
                    }
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
