//! This program illustrates what we can do with layers.
//! In a first time, we add every mark in the layer 1, meaning
//! it will be displayed behind the layer 0.
//! After adding all our marks, we will split them between
//! different layers and apply different modifications to
//! each layer. 
//! You should observe that each mark in the layer 0 will be slightly rotated (the blue marks).
//! Each mark in the layer 1 will be pink and each mark in the layer 2 will be larger than the other
//! marks (the red marks).
//! Furthermore, the 3 rectangles should be displayed in that order : the blue rotated one in front,
//! the pink one behind him and the big red one behind him.
//! 
//! You can press <escape> to close the window.

use contrast_renderer::LumiRenderer;
use contrast::properties::Position;
use contrast::properties::Rotation;
use contrast::properties::Size;
use contrast::marks::linemark::LineMode;
use contrast::marks::pointmark::Shape;
use contrast::marks::mark::Mark;
use contrast::MarkMacro;

const WINDOW_WIDTH : u32 = 800;
const WINDOW_HEIGHT : u32 = 800;

// Functions that will be applied to each mark of a layer //

fn rotate_marks(mark : &mut Mark) {
    mark.set_rotation(mark.get_rotation() + Rotation::from_degrees(30.0));
}

fn color_marks(mark : &mut Mark) {
    mark.set_color((1.0, 0.5, 1.0, 1.0));
}

fn enlarge_marks(mark : &mut Mark) {
    let size = mark.get_size();
    mark.set_size((size.width * 2.0, size.height * 2.0));
}


fn main()
{
    let mut renderer = LumiRenderer::init(WINDOW_WIDTH, WINDOW_HEIGHT, "Hello layers");
    let contrast = renderer.get_contrast_mut();

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

    let mut _mark_triangle = contrast.add_point_mark().set_position(pos)
        .set_size(size)
        .set_color((0.0, 0.0, 1.0, 1.0))
        .set_shape(Shape::Triangle)
        .get_id();

    let mut _mark_infinity = contrast.add_point_mark().set_position((pos.x + 250.0, pos.y, pos.z))
        .set_size(size)
        .set_color((1.0, 0.0, 0.0, 1.0))
        .set_shape(Shape::Infinity)
        .get_id();

    let mut _mark_point = contrast.add_point_mark().set_position((pos.x + 500.0, pos.y, pos.z))
        .set_size(size)
        .set_color((1.0, 0.0, 0.0, 1.0))
        .set_shape(Shape::Point)
        .get_id();

    let mut _mark_spade = contrast.add_point_mark().set_position((pos.x + 500.0, pos.y+250.0, pos.z))
        .set_size(size)
        .set_color((0.0, 0.0, 1.0, 1.0))
        .set_shape(Shape::Spade)
        .get_id();

    let mut _mark_clover = contrast.add_point_mark().set_position((pos.x + 250.0, pos.y+250.0, pos.z))
        .set_size(size)
        .set_color((1.0, 0.0, 0.0, 1.0))
        .set_shape(Shape::Clover)
        .get_id();

    let mut _mark_ring = contrast.add_point_mark().set_position((pos.x , pos.y+250.0, pos.z))
        .set_size(size)
        .set_color((1.0, 0.0, 0.0, 1.0))
        .set_shape(Shape::Ring)
        .get_id();

    let mut _mark_tag = contrast.add_point_mark().set_position((pos.x, pos.y -250.0, pos.z))
        .set_size(size)
        .set_color((1.0, 0.0, 0.0, 1.0))
        .set_shape(Shape::Tag)
        .get_id();

    let mut _mark_cross = contrast.add_point_mark().set_position((pos.x + 250.0, pos.y -250.0, pos.z))
        .set_size(size)
        .set_color((1.0, 0.0, 0.0, 1.0))
        .set_shape(Shape::Cross)
        .get_id();

    let mut _mark_asterisk = contrast.add_point_mark().set_position((pos.x + 500.0, pos.y -250.0, pos.z))
        .set_size(size)
        .set_color((1.0, 0.0, 0.0, 1.0))
        .set_shape(Shape::Asterisk)
        .get_id();

    let mut rect_1 = contrast.add_point_mark().set_position((pos.x + 230.0, pos.y, 0.0))
        .set_size((100.0, 100.0))
        .set_color((0.0, 0.0, 1.0, 1.0))
        .set_shape(Shape::Rectangle)
        .get_id();

    let mut _rect_2 = contrast.add_point_mark().set_position((pos.x + 250.0, pos.y + 10.0, 0.0))
        .set_size((100.0, 100.0))
        .set_color((1.0, 0.0, 0.0, 1.0))
        .set_shape(Shape::Rectangle)
        .get_id();

    let mut rect_3 = contrast.add_point_mark().set_position((pos.x + 270.0, pos.y + 20.0, 0.0))
        .set_size((100.0, 100.0))
        .set_color((1.0, 0.0, 0.0, 1.0))
        .set_shape(Shape::Rectangle)
        .get_id();

    // Add manually 2 layers
    contrast.add_layers(2);

    // Retrieve the layer 0 and add some marks to it
    let layer_0 = contrast.get_layer_mut(0).unwrap();
    layer_0.add_mark(&mut rect_1);
    layer_0.add_mark(&mut _mark_triangle);
    layer_0.add_mark(&mut _mark_spade);

    // Apply a rotation to each mark of the layer 0
    layer_0.apply_to_marks(rotate_marks);

    // Retrieve the layer 2 and add some marks to it
    let layer_2 = contrast.get_layer_mut(2).unwrap();
    layer_2.add_mark(&mut rect_3);
    layer_2.add_mark(&mut _mark_cross);

    // Apply an enlargement to each mark of the layer 2
    layer_2.apply_to_marks(enlarge_marks);

    // Retrieve the layer 1 and make each of its marks pink
    let layer_1 = contrast.get_layer_mut(1).unwrap();
    layer_1.apply_to_marks(color_marks);

    contrast.mark_dirty_all();

    renderer.run();
}