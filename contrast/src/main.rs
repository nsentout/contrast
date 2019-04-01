use contrast::markscontainer::Contrast;
use contrast::marks::mark::Mark;
use contrast::marks::pointmark::Shape;
use contrast::marks::linemark::LineMode;
use contrast::MarkMacro;
use properties::position::Position;
use properties::size::Size;


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
    // Initialize contrast
    let mut contrast = Contrast::new();
    contrast.init();

    // Set the current layer in which marks will be added by default
    contrast.set_current_layer(1);

    // Build some marks
    let pos = Position { x : 150.0, y : 400.0, z : 0.0 };
    let size = Size { width : 200.0, height : 200.0 };

    let _mark_line = contrast.add_line_mark().add_point(pos)
        .add_point((pos.x + 100.0, pos.y, pos.z))
        .add_point((pos.x + 100.0, pos.y + 100.0, pos.z))
        .add_point((pos.x, pos.y, pos.z))
        .set_thickness(20.0)
        .set_color((1.0, 0.0, 0.0, 1.0))
        .set_mode(LineMode::Linear)
        .get_id();

    let mark_poly = contrast.add_polygon_mark().add_point((pos.x + 70.0, pos.y - 150.0, pos.z))
        .add_point((pos.x + 70.0, pos.y - 50.0, pos.z))
        .add_point((pos.x + 90.0, pos.y - 20.0, pos.z))
        .add_point((pos.x + 170.0, pos.y - 50.0, pos.z))
        .add_point((pos.x + 170.0, pos.y - 150.0, pos.z))
        .add_point((pos.x + 120.0, pos.y - 190.0, pos.z))
        .set_color((1.0, 0.7, 0.0, 1.0))
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

    let layer_0 = contrast.get_layer_mut(0).unwrap();
    layer_0.add_mark(&mut m1);
    layer_0.add_mark(&mut _mark_triangle);
    layer_0.add_mark(&mut _mark_clover);
    layer_0.apply_to_marks(rotate_marks);

    let layer_1 = contrast.get_layer_mut(1).unwrap();
    layer_1.add_mark(&mut m2);
    layer_1.apply_to_marks(color_marks);

    let layer_2 = contrast.get_layer_mut(2).unwrap();
    layer_2.add_mark(&mut m3);
    layer_2.add_mark(&mut _mark_cross);
    layer_2.apply_to_marks(enlarge_marks);
}
