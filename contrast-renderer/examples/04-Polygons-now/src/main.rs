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


    let pos0 = Position { x : 200.0, y : 50.0, z : 0.0 };

    contrast.add_polygon_mark()
        .add_point(pos0)
        .add_point((pos0.x + 100.0, pos0.y, pos0.z))
        .add_point((pos0.x + 100.0, pos0.y + 100.0, pos0.z))
        .add_point((pos0.x, pos0.y + 200.0, pos0.z))
        .set_stroke_width(15.0)
        .set_color((1.0, 0.0, 1.0, 1.0));


    let pos1 = Position { x : 50.0, y : 50.0, z : 0.0 };

    contrast.add_polygon_mark()
        .add_point(pos1)
        .add_point((pos1.x + 100.0, pos1.y, pos1.z))
        .add_point((pos1.x + 100.0, pos1.y + 100.0, pos1.z))
        .add_point((pos1.x, pos1.y + 200.0, pos1.z))
        .set_fill()
        .set_color((1.0, 0.0, 0.0, 1.0));


    let pos2 = Position { x : 400.0, y : 250.0, z : 0.0 };

    contrast.add_polygon_mark()
        .add_point(pos2)
        .add_point((pos2.x + 50.0, pos2.y, pos2.z))
        .add_point((pos2.x + 50.0, pos2.y - 50.0, pos2.z))
        .add_point((pos2.x, pos2.y - 50.0, pos2.z))
        .add_point((pos2.x + 20.0, pos2.y - 25.0, pos2.z))
        .set_stroke_width(5.0)
        .set_color((1.0, 1.0, 0.0, 1.0));

    let pos3 = Position { x : 500.0, y : 250.0, z : 0.0 };

    contrast.add_polygon_mark()
        .add_point(pos3)
        .add_point((pos3.x + 50.0, pos3.y, pos3.z))
        .add_point((pos3.x + 50.0, pos3.y - 50.0, pos3.z))
        .add_point((pos3.x, pos3.y - 50.0, pos3.z))
        .add_point((pos3.x + 20.0, pos3.y - 25.0, pos3.z))
        .set_fill()
        .set_color((1.0, 1.0, 0.0, 1.0));


    let pos4 = Position { x : 100.0, y : 550.0, z : 0.0 };

    contrast.add_polygon_mark()
        .add_point(pos4)
        .add_point((pos4.x + 100.0, pos4.y, pos4.z))
        .add_point((pos4.x + 100.0, pos4.y + 200.0, pos4.z))
        .add_point((pos4.x, pos4.y + 200.0, pos4.z))
        .set_fill()
        .set_color((1.0, 1.0, 0.0, 1.0));

    let pos5 = Position { x : 110.0, y : 560.0, z : 1.0 };

    contrast.add_polygon_mark()
        .add_point(pos5)
        .add_point((pos5.x + 100.0, pos5.y, pos5.z))
        .add_point((pos5.x + 100.0, pos5.y + 200.0, pos5.z))
        .add_point((pos5.x, pos5.y + 200.0, pos5.z))
        .set_stroke_width(10.0)
        .set_color((1.0, 0.7, 0.3, 1.0));


    contrast.mark_dirty_all();
    renderer.run();
}
