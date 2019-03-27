//! This program demonstrates how to bind actions to our keys.
//! When pressing <space>, one of the mark will move to a random position
//! and change its text.
//! When pressing <F>, both marks will move to a random position
//! and change their text.
//! 
//! You can press <escape> to close the window.

use contrast_renderer::LumiRenderer;
use contrast_renderer::Key;
use contrast_properties::markid::MarkId;
use contrast::MarkMacro;
use contrast::markscontainer::Contrast;
use rand::Rng;

const WINDOW_WIDTH : u32 = 800;
const WINDOW_HEIGHT : u32 = 800;
const RANDOM_TEXT : [&str; 3] = ["I'm here", "Over there", "Here now"];

fn move_one_text(contrast : &mut Contrast, markid : &MarkId) {
    // Just a random number generator.
    let mut rng = rand::thread_rng();

    // Retrieve our mark thanks to the mark's id. We can now modify it as we wish.
    let mark = contrast.get_mark_mut(&markid).unwrap().as_text_mark_mut_unchecked();

    // Change randomly the position and the text of our mark.
    mark.set_position((rng.gen_range::<f32>(0.0, WINDOW_WIDTH as f32 - 200.0), rng.gen_range::<f32>(50.0, WINDOW_HEIGHT as f32), 0.0));
    mark.set_text(RANDOM_TEXT[rng.gen_range::<usize>(0, 3)]);

    contrast.mark_dirty(*markid);
}

fn move_both_texts(contrast : &mut Contrast, markids : &Vec<MarkId>) {
    let mut rng = rand::thread_rng();

    // Change randomly the position and the text of our marks. 
    for m in markids {
        let mark = contrast.get_mark_mut(&m).unwrap().as_text_mark_mut_unchecked();
        mark.set_position((rng.gen_range::<f32>(0.0, WINDOW_WIDTH as f32 - 200.0), rng.gen_range::<f32>(50.0, WINDOW_HEIGHT as f32), 0.0));
        mark.set_text(RANDOM_TEXT[rng.gen_range::<usize>(0, 3)]);
    }

    contrast.mark_dirty_all();
}

fn main()
{
    let mut renderer = LumiRenderer::init(WINDOW_WIDTH, WINDOW_HEIGHT, "Contrast");
    let contrast = renderer.get_contrast_mut();

    contrast.register_font("fatty", "../../crimson-b.ttf", 50);

    // We retrieve our mark's id. This allows us to later modify our mark later.
    let m1 = contrast.add_text_mark()
        .set_position((30.0, 100.0, 1.0))
        .set_font("fatty")
        .set_text("Red")
        .set_color((1.0, 0.0, 0.0, 1.0))
        .get_id();

    let m2 = contrast.add_text_mark()
        .set_position((200.0, 100.0, 1.0))
        .set_font("fatty")
        .set_text("Blue")
        .set_color((0.0, 0.0, 1.0, 1.0))
        .get_id();

    contrast.mark_dirty_all();

    // Create a vector containing both marks.
    let marks = vec!(m1, m2);

    // Add an action that will modify the mark 'm1' on press of the space key.
    renderer.add_mark_action_on_press(Key::Space, move_one_text, &m1);

    // Add an action that will modify both marks on press of the F key.
    renderer.add_mark_list_action_on_press(Key::F, move_both_texts, &marks);

    renderer.run();
}