//! The simplest program possible with contrast. It just renders a red triangle.
//!
//! You can press <escape> to close the window.

use contrast_renderer::LumiRenderer;
use contrast::marks::pointmark::Shape;

const WINDOW_WIDTH : u32 = 800;
const WINDOW_HEIGHT : u32 = 800;

fn main()
{
    // Initialize the renderer, opening a window.
    let mut renderer = LumiRenderer::init(WINDOW_WIDTH, WINDOW_HEIGHT, "Hello, world!");

    // Initialize contrast, allowing us to handle the marks.
    let contrast = renderer.get_contrast_mut();

    // Add a mark into contrast
    contrast.add_point_mark().set_position((400.0, 400.0, 0.0))
            .set_size((300.0, 300.0))
            .set_color((1.0, 0.0, 0.0, 1.0))
            .set_shape(Shape::Triangle);

    // Indicate to contrast that a new mark was added and that it needs to refresh.
    contrast.mark_dirty_all();

    // Run the renderer, which makes it display the marks and listen to devices events.
    renderer.run();
}