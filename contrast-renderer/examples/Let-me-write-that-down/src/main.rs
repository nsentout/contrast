//! This program displays some lines and show
//! that we can change their thickness.
//! 
//! You can press <escape> to close the window.

use contrast_renderer::LumiRenderer;
use contrast::MarkMacro;

const WINDOW_WIDTH : u32 = 800;
const WINDOW_HEIGHT : u32 = 800;

fn main()
{
    let mut renderer = LumiRenderer::init(WINDOW_WIDTH, WINDOW_HEIGHT, "Contrast");
    let contrast = renderer.get_contrast_mut();

    // Register some fonts so we can use them later.
    contrast.register_font("fatty", "../../crimson-b.ttf", 120);
    contrast.register_font("helvetica", "../../uhvr8a.pfb", 120);

    // Add some text marks.
    contrast.add_text_mark()
        .set_position((30.0, 100.0, 1.0))
        .set_font("fatty")
        .set_text("Red")
        .set_color((1.0, 0.0, 0.0, 1.0));

    contrast.add_text_mark()
        .set_position((30.0, 300.0, 1.0))
        .set_font("helvetica")
        .set_text("White")
        .set_color((1.0, 1.0, 1.0, 1.0));

    contrast.add_text_mark()
        .set_position((30.0, 500.0, 1.0))
        .set_font("fatty")
        .set_text("Blue")
        .set_color((0.0, 0.0, 1.0, 1.0));

    contrast.add_text_mark()
        .set_position((30.0, 700.0, 1.0))
        .set_font("helvetica")
        .set_text("Green")
        .set_color((0.0, 1.0, 0.0, 1.0));

    contrast.mark_dirty_all();
    renderer.run();
}