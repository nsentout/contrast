//! This program displays some lines and show
//! that we can change their thickness.
//! 
//! You can press <escape> to close the window.

use contrast_renderer::LumiRenderer;
use contrast::properties::Position;
use contrast::marks::linemark::LineMode;
use contrast::MarkMacro;

const WINDOW_WIDTH : u32 = 800;
const WINDOW_HEIGHT : u32 = 800;

fn main()
{
    let mut renderer = LumiRenderer::init(WINDOW_WIDTH, WINDOW_HEIGHT, "We need lines");
    let contrast = renderer.get_contrast_mut();

    let pos1 = Position { x : 200.0, y : 200.0, z : 0.0 };

    // Add a first line to contrast, with a thickness of 1 pixel.
    contrast.add_line_mark()
        .add_point(pos1)
        .add_point((pos1.x + 100.0, pos1.y, pos1.z))
        .add_point((pos1.x + 100.0, pos1.y + 100.0, pos1.z))
        .add_point((pos1.x, pos1.y + 200.0, pos1.z))
        .set_thickness(1.0)
        .set_color((1.0, 0.0, 0.0, 1.0))
        .set_mode(LineMode::Linear)
        .get_id();

    let pos2 = Position { x : 400.0, y : 250.0, z : 0.0 };

    // Add a second line to contrast, with a thickness of 10 pixels.
    contrast.add_line_mark()
        .add_point(pos2)
        .add_point((pos2.x + 100.0, pos2.y + 50.0, pos2.z))
        .add_point((pos2.x + 20.0, pos2.y - 100.0, pos2.z))
        .add_point((pos2.x, pos2.y + 200.0, pos2.z))
        .set_thickness(10.0)
        .set_color((1.0, 1.0, 0.0, 1.0))
        .set_mode(LineMode::Linear)
        .get_id();

    contrast.mark_dirty_all();
    renderer.run();
}