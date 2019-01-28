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

use contrast::MarkTrait;
use contrast::Mark;
use contrast::MarkProperty;
use contrast::MarksManager;
use contrast::Shape;

// we get the shaders at compile time 
const VS: &'static str = include_str!("shaders/mark-vs.glsl");
const FS: &'static str = include_str!("shaders/mark-fs.glsl");
const GS: &'static str = include_str!("shaders/mark-gs.glsl");

fn main()
{
    // Create some rectangles
    let center = [-0.6, 0.0];
    let size = [0.3, 0.5];
    let color = [0., 0., 1.];
    let rect1 = Mark::new(center, size, color, Shape::Rectangle);
    let rect2 = Mark::new([0.0, 0.0], size, [1., 1., 1.], Shape::Triangle);
    let rect3 = Mark::new([0.6, 0.0], size, [1., 0., 0.], Shape::Rectangle);

    // Add them to the marks manager to render them
    let mut marksmanager = MarksManager::create_marksmanager();
    marksmanager.add_mark(rect1);
    marksmanager.add_mark(rect2);
    marksmanager.add_mark(rect3);

    let mut surface = GlfwSurface::new(WindowDim::Windowed(800, 800), "Hello, world!", WindowOpt::default()).expect("GLFW surface creation");

    let (program, _) = Program::<MarkProperty, (), ()>::from_strings(None, VS, GS, FS).expect("program creation");

    //let tess = Tess::new(&mut surface, Mode::Point, &TRI_VERTICES[..], None);
    let tess = Tess::new(&mut surface, Mode::Point, &marksmanager.get_marks_properties()[..], None);

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

                WindowEvent::Key(Key::Space, _, Action::Release, _) =>
                {
                    
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