use lumi_renderer::LumiRenderer;
use contrast::MarkMacro;
use contrast::marks::pointmark::Shape;
use contrast::markscontainer::Contrast;
use properties::markid::MarkId;
use luminance_glfw::event::Key;
use rand::Rng;

const WINDOW_WIDTH : u32 = 800;
const WINDOW_HEIGHT : u32 = 800;

fn move_marks(contrast : &mut Contrast, markids : &Vec<MarkId>) {
    let mut rng = rand::thread_rng();
    for m in markids {
        contrast.get_mark_mut(&m).unwrap().as_point_mark_mut_unchecked().set_position((rng.gen_range::<f32>(0.0, WINDOW_WIDTH as f32), rng.gen_range::<f32>(0.0, WINDOW_HEIGHT as f32), 0.0));
        contrast.mark_dirty(*m);
    }
}

fn resize_marks(contrast : &mut Contrast, markids : &Vec<MarkId>) {
    let mut rng = rand::thread_rng();
    for m in markids {
        contrast.get_mark_mut(&m).unwrap().set_size((rng.gen_range::<f32>(100.0, 200.0), rng.gen_range::<f32>(100.0, 200.0)));
        contrast.mark_dirty(*m);
    }
}

fn color_marks(contrast : &mut Contrast, markids : &Vec<MarkId>) {
    let mut rng = rand::thread_rng();
    for m in markids {
        contrast.get_mark_mut(&m).unwrap().set_color((rng.gen_range::<f32>(0.0, 1.0), rng.gen_range::<f32>(0.0, 1.0), rng.gen_range::<f32>(0.0, 1.0), 1.0));
        contrast.mark_dirty(*m);
    }
}

fn rotate_marks(contrast : &mut Contrast, markids : &Vec<MarkId>) {
    let mut rng = rand::thread_rng();
    for m in markids {
        contrast.get_mark_mut(&m).unwrap().set_rotation(rng.gen_range::<f32>(3.14, 6.28));
        contrast.mark_dirty(*m);
    }
}

fn reshape_marks(contrast : &mut Contrast, markids : &Vec<MarkId>) {
    let mut rng = rand::thread_rng();
    for m in markids {
        contrast.get_mark_mut(&m).unwrap().as_point_mark_mut_unchecked().set_shape(Shape::rand(&mut rng));
        contrast.mark_dirty(*m);
    }
}


fn main()
{
    let mut luminance = LumiRenderer::init(WINDOW_WIDTH, WINDOW_HEIGHT, "Contrast");
    let contrast = luminance.get_contrast_mut();

    //contrast.register_font("helvetica", "uhvr8a.pfb", 32);
    contrast.register_font("fatty", "crimson-b.ttf", 40);

    contrast.add_text_mark()
        .set_position(30, 50)
        .set_font("fatty")
        .set_text("Commandes pour animer les marques :")
        .set_color((1.0, 1.0, 1.0, 1.0))
        .get_id();

    contrast.add_text_mark()
        .set_position(30, 120)
        .set_font("fatty")
        .set_text("- <A> pour changer leur forme")
        .set_color((1.0, 1.0, 1.0, 1.0))
        .get_id();

    contrast.add_text_mark()
        .set_position(30, 170)
        .set_font("fatty")
        .set_text("- <Z> pour les faire bouger")
        .set_color((1.0, 1.0, 1.0, 1.0))
        .get_id();

    contrast.add_text_mark()
        .set_position(30, 220)
        .set_font("fatty")
        .set_text("- <E> pour changer leur couleur")
        .set_color((1.0, 1.0, 1.0, 1.0))
        .get_id();

    contrast.add_text_mark()
        .set_position(30, 270)
        .set_font("fatty")
        .set_text("- <R> pour les redimensionner")
        .set_color((1.0, 1.0, 1.0, 1.0))
        .get_id();

    contrast.add_text_mark()
        .set_position(30, 320)
        .set_font("fatty")
        .set_text("- <F> pour leur appliquer une rotation")
        .set_color((1.0, 1.0, 1.0, 1.0))
        .get_id();

    let point1 = contrast.add_point_mark()
        .set_position((400.0, 400.0, 0.0))
        .set_size((200.0, 200.0))
        .set_color((1.0, 0.0, 0.0, 1.0))
        .set_shape(Shape::Triangle)
        .get_id();

    let point2 = contrast.add_point_mark()
        .set_position((400.0, 400.0, 0.0))
        .set_size((200.0, 200.0))
        .set_color((1.0, 0.0, 0.0, 1.0))
        .set_shape(Shape::Triangle)
        .get_id();

    let point3 = contrast.add_point_mark()
        .set_position((400.0, 400.0, 0.0))
        .set_size((200.0, 200.0))
        .set_color((1.0, 0.0, 0.0, 1.0))
        .set_shape(Shape::Triangle)
        .get_id();
/*
    for _ in 0..200_000 {
        contrast.add_point_mark().set_position((rng.gen_range::<f32>(0.0, WINDOW_WIDTH as f32), rng.gen_range::<f32>(0.0, WINDOW_HEIGHT as f32), 0.0))
            .set_size((8.0, 8.0))
            .set_color((rng.gen_range::<f32>(0.0, 1.0), rng.gen_range::<f32>(0.0, 1.0), rng.gen_range::<f32>(0.0, 1.0), 1.0))
            .set_shape(Shape::Triangle);
    }
*/
    contrast.mark_dirty_all();

    let marks = vec!(point1, point2, point3);

    luminance.add_mark_list_action_on_press(Key::Q, reshape_marks, &marks);
    luminance.add_mark_list_action_on_press(Key::W, move_marks, &marks);
    luminance.add_mark_list_action_on_press(Key::E, color_marks, &marks);
    luminance.add_mark_list_action_on_press(Key::R, resize_marks, &marks);
    luminance.add_mark_list_action_on_press(Key::F, rotate_marks, &marks);

    luminance.run();
}