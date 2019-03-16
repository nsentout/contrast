use lumi_renderer::LumiRenderer;

use contrast::MarkMacro;
use contrast::marks::pointmark::Shape;
use contrast::marks::linemark::LineMode;
use properties::position::Position;
//use properties::markid::MarkId;
use properties::size::Size;

const WINDOW_WIDTH : u32 = 800;
const WINDOW_HEIGHT : u32 = 800;

fn main()
{
    let mut luminance = LumiRenderer::init(WINDOW_WIDTH, WINDOW_HEIGHT, "Contrast");

    let contrast = luminance.get_contrast_mut();

    let pos = Position { x : 150.0, y : WINDOW_HEIGHT as f32 / 2.0, z : 0.0 };
    let size = Size { width : 200.0, height : 200.0 };

    let _mark_line = contrast.add_line_mark().add_point(pos)
        .add_point((pos.x+100.0, pos.y, pos.z))
        .add_point((pos.x +100.0, pos.y + 100.0, pos.z))
        .add_point((pos.x, pos.y, pos.z))
        .set_thickness(20.0)
        .set_color((1.0, 0.0, 0.0, 1.0))
        .set_mode(LineMode::Linear)
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

    contrast.mark_dirty_all();

    luminance.run();
}