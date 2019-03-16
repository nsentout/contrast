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
use contrast::marks::linemark::LineMode;
use contrast::marks::linemark::SubLine;
use contrast::camera::Camera;
use contrast::MarkMacro;
use properties::position::Position;
use properties::size::Size;

use rand::Rng;
use std::time::Instant;

// Path to the shaders
const VSPOINT: &'static str = include_str!("shaders/point.vert");
const FSPOINT: &'static str = include_str!("shaders/point.frag");
const GSPOINT: &'static str = include_str!("shaders/point.geom");

const VSLINE: &'static str = include_str!("shaders/line.vert");
const FSLINE: &'static str = include_str!("shaders/line.frag");
const GSLINE: &'static str = include_str!("shaders/line.geom");

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

fn rotate_marks(mark : &mut Mark) {
    mark.set_rotation(45.0);
}

fn color_marks(mark : &mut Mark) {
    mark.set_color((1.0, 0.5, 0.5, 1.0));
}

fn main()
{
    // Initialize contrast
    let mut contrast = Contrast::new();
    contrast.init();

    // Initialize the camera
    let mut cam = Camera::init(WINDOW_WIDTH, WINDOW_HEIGHT);

    // Build 100 000 point marks with random positions and random colors
    let mut rng = rand::thread_rng();

    println!("Building marks ...");
    /*
    for _ in 0..100_000 {
        contrast.add_point_mark().set_position((rng.gen_range::<f32>(0.0, WINDOW_WIDTH as f32), rng.gen_range::<f32>(0.0, WINDOW_HEIGHT as f32), 0.0))
            .set_size((8.0, 8.0))
            .set_color((rng.gen_range::<f32>(0.0, 1.0), rng.gen_range::<f32>(0.0, 1.0), rng.gen_range::<f32>(0.0, 1.0), 1.0))
            .set_shape(Shape::Triangle);
    }
    */
    let pos = Position { x : 150.0, y : WINDOW_HEIGHT as f32 / 2.0, z : 0.0 };
    let size = Size { width : 200.0, height : 200.0 };


    let mark_line = contrast.add_line_mark().add_point(pos)
        .add_point((pos.x+100.0, pos.y, pos.z))
        .add_point((pos.x +100.0, pos.y + 100.0, pos.z))
        .add_point((pos.x, pos.y, pos.z))
        //.add_point((pos.x+10.0, pos.y, pos.z))
        //.add_point((pos.x+100.0, pos.y+300.0, pos.z))
        //.add_point((pos.x+100.0, pos.y+100.0, pos.z))
        //.add_point((pos.x-100.0, pos.y-100.0, pos.z))
        //.add_point((pos.x+100.0, pos.y+50.0, pos.z))
        .set_thickness(20.0)
        .set_color((1.0, 0.0, 0.0, 1.0))
        .set_mode(LineMode::Linear)
        .get_id();


    let mut _mark_triangle = contrast.add_point_mark().set_position(pos)
        .set_size(size)
        .set_color((1.0, 0.0, 0.0, 1.0))
        .set_shape(Shape::Triangle)
        .get_id();

    let mut _mark_triangle = contrast.add_point_mark().set_position(pos)
        .set_size(size)
        .set_color((1.0, 0.0, 0.0, 1.0))
        .set_shape(Shape::Triangle)
        .get_id();

    let mut _mark_infinity = contrast.add_point_mark().set_position((pos.x + 250.0, pos.y, pos.z))
        .set_size(size)
        .set_color((0.0, 1.0, 0.0, 1.0))
        .set_shape(Shape::Infinity)
        .get_id();

    let mut _mark_point = contrast.add_point_mark().set_position((pos.x + 500.0, pos.y, pos.z))
        .set_size(size)
        .set_color((0.0, 0.0, 1.0, 1.0))
        .set_shape(Shape::Point)
        .get_id();

    let mut _mark_spade = contrast.add_point_mark().set_position((pos.x + 500.0, pos.y+250.0, pos.z))
        .set_size(size)
        .set_color((1.0, 0.0, 1.0, 1.0))
        .set_shape(Shape::Spade)
        .get_id();

    let mut _mark_clover = contrast.add_point_mark().set_position((pos.x + 250.0, pos.y+250.0, pos.z))
        .set_size(size)
        .set_color((1.0, 1.0, 1.0, 1.0))
        .set_shape(Shape::Clover)
        .get_id();

    let mut _mark_ring = contrast.add_point_mark().set_position((pos.x , pos.y+250.0, pos.z))
        .set_size(size)
        .set_color((1.0, 1.0, 0.0, 1.0))
        .set_shape(Shape::Ring)
        .get_id();

    let mut _mark_tag = contrast.add_point_mark().set_position((pos.x, pos.y -250.0, pos.z))
        .set_size(size)
        .set_color((1.0, 0.5, 0.0, 1.0))
        .set_shape(Shape::Tag)
        .get_id();

    let mut _mark_cross = contrast.add_point_mark().set_position((pos.x + 250.0, pos.y -250.0, pos.z))
        .set_size(size)
        .set_color((0.0, 1.0, 1.0, 1.0))
        .set_shape(Shape::Cross)
        .get_id();

    let mut _mark_asterisk = contrast.add_point_mark().set_position((pos.x + 500.0, pos.y -250.0, pos.z))
        .set_size(size)
        .set_color((0.0, 0.5, 1.0, 1.0))
        .set_shape(Shape::Asterisk)
        .get_id();


    let mut m1 = contrast.add_point_mark().set_position((pos.x + 230.0, pos.y, 0.0))
        .set_size((100.0, 100.0))
        .set_color((1.0, 0.0, 0.0, 1.0))
        .set_shape(Shape::Rectangle)
        .get_id();

    let mut m2 = contrast.add_point_mark().set_position((pos.x + 250.0, pos.y + 10.0, 0.0))
        .set_size((100.0, 100.0))
        .set_color((1.0, 0.5, 0.0, 1.0))
        .set_shape(Shape::Rectangle)
        .get_id();

    let mut m3 = contrast.add_point_mark().set_position((pos.x + 270.0, pos.y + 20.0, 0.0))
        .set_size((100.0, 100.0))
        .set_color((1.0, 1.0, 0.0, 1.0))
        .set_shape(Shape::Rectangle)
        .get_id();


    contrast.add_layers(2);

    let layer_1 = contrast.get_layer_mut(1).unwrap();
    layer_1.add_mark(&mut m2);
    layer_1.apply_to_marks(color_marks);

    let layer_2 = contrast.get_layer_mut(2).unwrap();
    layer_2.add_mark(&mut m3);
    layer_2.apply_to_marks(color_marks);

    let layer_0 = contrast.get_layer_mut(0).unwrap();
    layer_0.add_mark(&mut m1);
    layer_0.apply_to_marks(rotate_marks);


    println!("Building finished!");
    println!("Rendering ...");

    // Create a new surface to render to and get events from
    let mut surface = GlfwSurface::new(WindowDim::Windowed(WINDOW_WIDTH, WINDOW_HEIGHT), "contrast playground", WindowOpt::default()).expect("GLFW surface creation");

    // We need a program to “shade” our triangles and to tell luminance which is the input vertex type
    let (program, _) = Program::<VertexPoint, (), ShaderInterface>::from_strings(None, VSPOINT, GSPOINT, FSPOINT).expect("program creation");
    let (programLine, _) = Program::<SubLine, (), ShaderInterface>::from_strings(None, VSLINE, GSLINE, FSLINE).expect("program creation");

    // Create tessellation for direct geometry; that is, tessellation that will render vertices by taking one after another in the provided slice
    let tess = Tess::new(&mut surface, Mode::Point, &contrast.get_pointmarks_properties()[..], None);
    let tessLine = Tess::new(&mut surface, Mode::Point, &contrast.get_linemarks_properties()[..], None);
    // The back buffer, which we will make our render into (we make it mutable so that we can change it whenever the window dimensions change)
    let mut back_buffer = Framebuffer::back_buffer(surface.size());

    // Initialize timer
    let mut time = Instant::now();
    let mut frames = 0;
    let mut elapsed;

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
                    /*let mark = contrast.get_mark_mut(&m1).unwrap();
                    let mut color = mark.get_color();
                    println!("Mark color before : ({:.2}, {:.2}, {:.2}, {:.2})", color.r, color.g, color.b, color.a);
                    mark.set_color((rng.gen_range::<f32>(0.0, 1.0), rng.gen_range::<f32>(0.0, 1.0), rng.gen_range::<f32>(0.0, 1.0), 1.0));
                    color = mark.get_color();
                    println!("Mark color after : ({:.2}, {:.2}, {:.2}, {:.2})", color.r, color.g, color.b, color.a)*/
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

        elapsed = time.elapsed();
        frames += 1;

        if elapsed.as_secs() >= 1 {
            println!("FPS : {}", frames);
            time = Instant::now();
            frames = 0;
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
            shd_gate.shade(&programLine, |rdr_gate, iface|
            {
                iface.projection.update(cam.data());
                // Start rendering things with the default render state provided by luminance
                rdr_gate.render(RenderState::default(), |tess_gate|
                {
                    let tessLine = &tessLine;
                    // Render the tessellation to the surface
                    tess_gate.render(&mut surface, tessLine.into());
                });
            });

        });

        // Finally, swap the backbuffer with the frontbuffer in order to render our marks onto the screen
        surface.swap_buffers();
    }
}
