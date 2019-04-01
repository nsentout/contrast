//! Triangles are not enough. We need more shapes to fill the window.
//! This program shows all existing shapes. 
//!
//! You can press <escape> to close the window.

use contrast_renderer::LumiRenderer;
use contrast::properties::Position;
use contrast::properties::Size;
use contrast::properties::Color;
use contrast::marks::pointmark::Shape;

const WINDOW_WIDTH : u32 = 900;
const WINDOW_HEIGHT : u32 = 900;

fn main()
{
    let mut renderer = LumiRenderer::init(WINDOW_WIDTH, WINDOW_HEIGHT, "More shapes");
    let contrast = renderer.get_contrast_mut();

    // Initialize some properties our marks will need.
    let start_x = 150.0;
    let mut pos = Position { x : start_x, y : 90.0, z : 0.0 };
    let mut color = Color { r : 0.8, g : 0.0, b : 0.0, a : 1.0 };
    let size = Size { width : 150.0, height : 150.0 };

    // Loop through all type of shapes and add them into contrast.
    for i in 1..20
    {
        contrast.add_point_mark().set_position(pos)
            .set_size(size)
            .set_color(color)
            .set_shape(Shape::from_integer(i));

        color.g += 0.05;

        pos.x += size.width + 60.0;
        if i % 4 == 0 {
            pos.x = start_x;
            pos.y += size.height + 40.0;
            color.b += 0.5;
        }
    }

    contrast.mark_dirty_all();
    renderer.run();
}