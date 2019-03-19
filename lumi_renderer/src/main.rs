use lumi_renderer::LumiRenderer;

use contrast::MarkMacro;
use contrast::marks::mark::Mark;
use contrast::marks::pointmark::Shape;
use contrast::marks::linemark::LineMode;
use properties::position::Position;
//use properties::markid::MarkId;
use properties::size::Size;
use luminance_glfw::event::Key;

const WINDOW_WIDTH : u32 = 800;
const WINDOW_HEIGHT : u32 = 800;

fn rotate_marks(mark : &mut Mark) {
    mark.set_rotation(mark.get_rotation() + 45.0);
}

fn color_marks(mark : &mut Mark) {
    mark.set_color((1.0, 0.5, 0.5, 1.0));
}

fn enlarge_marks(mark : &mut Mark) {
    let size = mark.get_size();
    mark.set_size((size.width * 1.5, size.height * 1.5));
}

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

    contrast.add_layers(2);

    let layer_0 = contrast.get_layer_mut(0).unwrap();
    layer_0.add_mark(&mut _mark_spade);
    layer_0.apply_to_marks(rotate_marks);

    let layer_1 = contrast.get_layer_mut(1).unwrap();
    layer_1.add_mark(&mut _mark_clover);
    layer_1.apply_to_marks(color_marks);

    let layer_2 = contrast.get_layer_mut(2).unwrap();
    layer_2.add_mark(&mut _mark_ring);
    layer_2.apply_to_marks(enlarge_marks);

    contrast.mark_dirty_all();

    luminance.add_listener(Key::Up, || {
        println!("UP");
        //&contrast.get_mark_mut(&mut _mark_ring).unwrap().move_of((10.0, 0.0, 0.0));
    });

    luminance.add_listener(Key::Left, || {
        println!("LEFT");
    });

    luminance.add_listener(Key::Down, || {
        println!("DOWN");
    });

    luminance.add_listener(Key::Right, || {
        println!("RIGHT");
    });

    luminance.run();
}