//! This program displays some polygons and show
//! that we can change the width of their stroke.
//! 
//! You can press <escape> to close the window.

use contrast_renderer::LumiRenderer;
use contrast::properties::Position;
use contrast::MarkMacro;

const WINDOW_WIDTH : u32 = 800;
const WINDOW_HEIGHT : u32 = 800;

fn main()
{
    let mut renderer = LumiRenderer::init(WINDOW_WIDTH, WINDOW_HEIGHT, "Polygons now");
    let contrast = renderer.get_contrast_mut();

    let pos1 = Position { x : 100.0, y : 200.0, z : 0.0 };

    contrast.add_polygon_mark()
        .add_point(pos1)
        .add_point((pos1.x + 200.0, pos1.y, pos1.z))
        .add_point((pos1.x + 200.0, pos1.y + 200.0, pos1.z))
        .add_point((pos1.x, pos1.y + 400.0, pos1.z))
        .set_stroke_width(10.0)
        .set_color((1.0, 0.0, 0.0, 1.0));

    let pos2 = Position { x : 400.0, y : 250.0, z : 0.0 };

    contrast.add_polygon_mark()
        .add_point(pos2)
        .add_point((pos2.x + 150.0, pos2.y - 45.0, pos2.z))
        .add_point((pos2.x + 300.0, pos2.y, pos2.z))
        .add_point((pos2.x + 300.0, pos2.y - 90.0, pos2.z))
        .add_point((pos2.x + 150.0, pos2.y - 45.0, pos2.z))
        .add_point((pos2.x, pos2.y - 90.0, pos2.z))
        .set_stroke_width(1.0)
        .set_color((1.0, 1.0, 0.0, 1.0));

    let pos3 = Position { x : 350.0, y : 500.0, z : 0.0 };

    contrast.add_polygon_mark()
        .add_point(pos3)
        .add_point((pos3.x + 100.0, pos3.y - 60.0, pos3.z))
        .add_point((pos3.x + 120.0, pos3.y, pos3.z))
        .add_point((pos3.x + 200.0, pos3.y - 80.0, pos3.z))
        .add_point((pos3.x + 200.0, pos3.y, pos3.z))
        .add_point((pos3.x + 120.0, pos3.y + 80.0, pos3.z))
        .add_point((pos3.x + 80.0, pos3.y, pos3.z))
        .add_point((pos3.x + 20.0, pos3.y + 60.0, pos3.z))
        .set_stroke_width(3.0)
        .set_color((1.0, 1.0, 1.0, 1.0));

    contrast.mark_dirty_all();
    renderer.run();
}