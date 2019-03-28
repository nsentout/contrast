//! This program use the previously shown feature allowing us
//! to bind actions to our keys to show how we can animate
//! our marks. 
//!
//! List of commands to animate the marks : 
//! - <A> to change their shape
//! - <Z> to change their position
//! - <E> to change their color
//! - <R> to change their size
//! - <F> to change their rotation
//! 
//! You can also press <escape> to close the window.

use contrast_renderer::LumiRenderer;
use contrast_renderer::Key;
use contrast::MarkMacro;
use contrast::marks::pointmark::Shape;
use contrast::markscontainer::Contrast;
use contrast_properties::markid::MarkId;
use rand::Rng;

const WINDOW_WIDTH : u32 = 800;
const WINDOW_HEIGHT : u32 = 800;

fn move_marks(contrast : &mut Contrast, markids : &Vec<MarkId>) {
    let mut rng = rand::thread_rng();

    // Move our marks to a random position.
    for m in markids {
        contrast.get_mark_mut(&m).unwrap().as_point_mark_mut_unchecked().set_position((rng.gen_range::<f32>(0.0, WINDOW_WIDTH as f32), rng.gen_range::<f32>(0.0, WINDOW_HEIGHT as f32), 0.0));
    }
    contrast.mark_dirty_all();
}

fn resize_marks(contrast : &mut Contrast, markids : &Vec<MarkId>) {
    let mut rng = rand::thread_rng();

    // Give our marks a random size.
    for m in markids {
        contrast.get_mark_mut(&m).unwrap().set_size((rng.gen_range::<f32>(100.0, 200.0), rng.gen_range::<f32>(100.0, 200.0)));
    }
    contrast.mark_dirty_all();
}

fn color_marks(contrast : &mut Contrast, markids : &Vec<MarkId>) {
    let mut rng = rand::thread_rng();

    // Give our marks a random color.
    for m in markids {
        contrast.get_mark_mut(&m).unwrap().set_color((rng.gen_range::<f32>(0.0, 1.0), rng.gen_range::<f32>(0.0, 1.0), rng.gen_range::<f32>(0.0, 1.0), 1.0));
    }
    contrast.mark_dirty_all();
}

fn rotate_marks(contrast : &mut Contrast, markids : &Vec<MarkId>) {
    let mut rng = rand::thread_rng();

    // Give our marks a random rotation.
    for m in markids {
        contrast.get_mark_mut(&m).unwrap().set_rotation(rng.gen_range::<f32>(3.14, 6.28));
    }
    contrast.mark_dirty_all();
}

fn reshape_marks(contrast : &mut Contrast, markids : &Vec<MarkId>) {
    let mut rng = rand::thread_rng();

    // Give our marks a random shape.
    for m in markids {
        contrast.get_mark_mut(&m).unwrap().as_point_mark_mut_unchecked().set_shape(Shape::rand(&mut rng));
    }
    contrast.mark_dirty_all();
}


fn main()
{
    let mut renderer = LumiRenderer::init(WINDOW_WIDTH, WINDOW_HEIGHT, "A lot of marks");
    let contrast = renderer.get_contrast_mut();
    let mut rng = rand::thread_rng();

    contrast.register_font("fatty", "../../crimson-b.ttf", 40);

    let mut marks = Vec::new();

    // Create ... a lot of marks
    for _ in 0..200_000 {
        marks.push(contrast.add_point_mark().set_position((rng.gen_range::<f32>(0.0, WINDOW_WIDTH as f32), rng.gen_range::<f32>(0.0, WINDOW_HEIGHT as f32), 0.0))
            .set_size((8.0, 8.0))
            .set_color((rng.gen_range::<f32>(0.0, 1.0), rng.gen_range::<f32>(0.0, 1.0), rng.gen_range::<f32>(0.0, 1.0), 1.0))
            .set_shape(Shape::Triangle)
            .get_id());
    }

    contrast.mark_dirty_all();

    renderer.add_mark_list_action_on_press(Key::Q, reshape_marks, &marks);
    renderer.add_mark_list_action_on_press(Key::W, move_marks, &marks);
    renderer.add_mark_list_action_on_press(Key::E, color_marks, &marks);
    renderer.add_mark_list_action_on_press(Key::R, resize_marks, &marks);
    renderer.add_mark_list_action_on_press(Key::F, rotate_marks, &marks);

    renderer.run();
}