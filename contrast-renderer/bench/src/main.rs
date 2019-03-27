use contrast_renderer_bench::LumiRenderer;
use contrast_renderer_bench::Key;
use contrast::MarkMacro;
use contrast::marks::mark::Mark;
use contrast::marks::pointmark::Shape;
use contrast::markscontainer::Contrast;
use contrast::elapsed_time_float;
use contrast_properties::markid::MarkId;
use rand::Rng;

const WINDOW_WIDTH : u32 = 800;
const WINDOW_HEIGHT : u32 = 800;

fn move_marks(contrast : &mut Contrast, markids : &Vec<MarkId>) {
    let mut rng = rand::thread_rng();
    for m in markids {
        contrast.get_mark_mut(&m).unwrap().as_point_mark_mut_unchecked().set_position((rng.gen_range::<f32>(0.0, WINDOW_WIDTH as f32), rng.gen_range::<f32>(0.0, WINDOW_HEIGHT as f32), 0.0));
    }
    contrast.mark_dirty_all();
}

fn resize_marks(contrast : &mut Contrast, markids : &Vec<MarkId>) {
    let mut rng = rand::thread_rng();
    for m in markids {
        contrast.get_mark_mut(&m).unwrap().set_size((rng.gen_range::<f32>(6.0, 10.0), rng.gen_range::<f32>(6.0, 10.0)));
    }
    contrast.mark_dirty_all();
}

fn color_marks(contrast : &mut Contrast, markids : &Vec<MarkId>) {
    let mut rng = rand::thread_rng();
    for m in markids {
        contrast.get_mark_mut(&m).unwrap().set_color((rng.gen_range::<f32>(0.0, 1.0), rng.gen_range::<f32>(0.0, 1.0), rng.gen_range::<f32>(0.0, 1.0), 1.0));
    }
    contrast.mark_dirty_all();
}

fn rotate_marks(contrast : &mut Contrast, markids : &Vec<MarkId>) {
    let mut rng = rand::thread_rng();
    for m in markids {
        contrast.get_mark_mut(&m).unwrap().set_rotation(rng.gen_range::<f32>(3.14, 6.28));
    }
    contrast.mark_dirty_all();
}

fn reshape_marks(contrast : &mut Contrast, markids : &Vec<MarkId>) {
    let mut rng = rand::thread_rng();
    for m in markids {
        contrast.get_mark_mut(&m).unwrap().as_point_mark_mut_unchecked().set_shape(Shape::rand(&mut rng));
    }
    contrast.mark_dirty_all();
}

fn main()
{
    let mut luminance = LumiRenderer::init(WINDOW_WIDTH, WINDOW_HEIGHT, "Contrast");
    let contrast = luminance.get_contrast_mut();
    let mut rng = rand::thread_rng();

    let mut marks = Vec::<MarkId>::new();

    for _ in 0..200_000 {
        let m = contrast.add_point_mark().set_position((rng.gen_range::<f32>(0.0, WINDOW_WIDTH as f32), rng.gen_range::<f32>(0.0, WINDOW_HEIGHT as f32), 0.0))
            .set_size((8.0, 8.0))
            .set_color((rng.gen_range::<f32>(0.0, 1.0), rng.gen_range::<f32>(0.0, 1.0), rng.gen_range::<f32>(0.0, 1.0), 1.0))
            .set_shape(Shape::Triangle);
        marks.push(m.get_id());
    }

    contrast.mark_dirty_all();

    luminance.add_mark_list_action_on_press(Key::Q, reshape_marks, &marks);
    luminance.add_mark_list_action_on_press(Key::W, move_marks, &marks);
    luminance.add_mark_list_action_on_press(Key::E, color_marks, &marks);
    luminance.add_mark_list_action_on_press(Key::R, resize_marks, &marks);
    luminance.add_mark_list_action_on_press(Key::F, rotate_marks, &marks);

    luminance.run();
}