//! This program show an real application we
//! could make with contrast.
//! We read some data (here, there are just 2D coordinates) from a text file
//! and we display them on a graph.
//! Pressing <space> will randomly change the color of each point.
//! 
//! 
//! You can also press <escape> to close the window.

use contrast_renderer::LumiRenderer;
use contrast_renderer::Key;
use contrast::MarkMacro;
use contrast::marks::pointmark::Shape;
use contrast::marks::linemark::LineMode;
use contrast::markscontainer::Contrast;
use contrast::properties::MarkId;
use contrast::properties::Color;
use contrast::properties::Position;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::path::Path;
use rand::Rng;

const WINDOW_WIDTH : u32 = 800;
const WINDOW_HEIGHT : u32 = 800;


fn color_marks(contrast : &mut Contrast, markids : &Vec<MarkId>) {
    let mut rng = rand::thread_rng();

    // Give our marks a random color.
    for m in markids {
        contrast.get_mark_mut(&m).unwrap().set_color((rng.gen_range::<f32>(0.0, 1.0), rng.gen_range::<f32>(0.0, 1.0), rng.gen_range::<f32>(0.0, 1.0), 1.0));
    }
    contrast.mark_dirty_all();
}

// Parse a file containing our data and return its lines in a vector.
fn parse_file(file_name : &str) -> Vec<String> {
    let file = File::open(Path::new(file_name)).unwrap();
    let reader = BufReader::new(&file);
    reader.lines().collect::<Result<_, _>>().unwrap()
}

fn main()
{
    let mut renderer = LumiRenderer::init(WINDOW_WIDTH, WINDOW_HEIGHT, "Scatter plot");
    renderer.set_background_color(Color::grey());
    let contrast = renderer.get_contrast_mut();

    let lines = parse_file("data");
    let mut marks = Vec::<MarkId>::new();

    // We split each line in two to retrieve the x and y coordinate to create a point mark
    for l in lines {
        let pos : Vec<&str> = l.split(",").collect();
        marks.push(contrast.add_point_mark()
                    .set_position((pos[0].parse::<f32>().unwrap(), pos[1].parse::<f32>().unwrap()))
                    .set_size((20.0, 20.0))
                    .set_shape(Shape::Point)
                    .set_color(Color::red())
                    .get_id());
    }

    let bottom_left = Position { x : 30.0, y : WINDOW_HEIGHT as f32 - 30.0, z : 0.0 };

    // Add both axis
    contrast.add_line_mark()
        .add_point(bottom_left)
        .add_point((WINDOW_WIDTH as f32 - bottom_left.x, bottom_left.y, 0.0))
        .set_thickness(3.0)
        .set_color(Color::black())
        .set_mode(LineMode::Linear);

    contrast.add_line_mark()
        .add_point(bottom_left)
        .add_point((bottom_left.x, WINDOW_HEIGHT as f32 - bottom_left.y, 0.0))
        .set_thickness(3.0)
        .set_color(Color::black())
        .set_mode(LineMode::Linear);

    // Add text to name the axis
    contrast.register_font("fatty", "../../crimson-b.ttf", 40);

    contrast.add_text_mark()
        .set_position((750.0, 750.0, 1.0))
        .set_font("fatty")
        .set_text("X")
        .set_color(Color::black());

    contrast.add_text_mark()
        .set_position((50.0, 40.0, 1.0))
        .set_font("fatty")
        .set_text("Y")
        .set_color(Color::black());

    contrast.mark_dirty_all();

    // Randomly color the points when pressing <space>
    renderer.add_mark_list_action_on_press(Key::Space, color_marks, &marks);

    renderer.run();
}