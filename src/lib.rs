type Position2D = [f32; 2];
type Size = [f32; 2];
type Color = [f32; 3];

#[derive(Copy, Clone)]
pub enum Shape {
    None = 0,
    Rectangle = 1,
    Triangle = 2,
}

pub type MarkProperty = (Position2D, Size, Color, u32, u32);    // shape and id

pub struct Mark {
    center: Position2D,
    size : Size,
    color: Color,
    shape : Shape,
}

pub trait MarkTrait {
    fn new(Position2D, Size, Color, Shape) -> Self;
    fn get_position(&self) -> Position2D;
    fn get_size(&self) -> Size;
    fn get_color(&self) -> Color;
    fn get_properties(&self) -> (Position2D, Size, Color, Shape);
    fn set_position(&mut self, new_center : Position2D) -> &mut Self;
    fn set_size(&mut self, new_size : Size) -> &mut Self;
    fn set_color(&mut self, new_color : Color) -> &mut Self;
}

impl MarkTrait for Mark {
    fn new(center : Position2D, size : Size, color : Color, shape : Shape) -> Mark {
        Mark {
            center,
            size,
            color,
            shape
        }
    }

    fn get_position(&self) -> Position2D
    {
        self.center
    }

    fn get_size(&self) -> Size
    {
        self.size
    }

    fn get_color(&self) -> Color
    {
        self.color
    }

    fn get_properties(&self) -> (Position2D, Size, Color, Shape)
    {
        (self.center, self.size, self.color, self.shape)
    }

    fn set_position(&mut self, new_center : Position2D) -> &mut Self
    {
        self.center = new_center;
        self
    }

    fn set_size(&mut self, new_size : Size) -> &mut Self
    {
        self.size = new_size;
        self
    }

    fn set_color(&mut self, new_color : Color) -> &mut Self
    {
        self.color = new_color;
        self
    }

}

/***********************************************************************************/

const MAX_MARKS: usize = 10;

pub struct MarksManager {
    marks_properties : [MarkProperty; MAX_MARKS],    // comment faire pour que ce soit dynamique ?
    mark_count : u32,
    mark_size : usize,
}

impl MarksManager {
    pub fn create_marksmanager() -> Self
    {
        // je ne sais pas comment faire autrement, au secours
        let marks_properties : [MarkProperty; MAX_MARKS] = [([0.0, 0.0], [0.0, 0.0], [0.0, 0.0, 0.0], Shape::None as u32, 0); MAX_MARKS];

        MarksManager {
            marks_properties,
            mark_count : 0,
            mark_size : 0,
        }
    }

    pub fn add_mark(&mut self, mark : Mark)
    {
        let (center, size, color, shape) = mark.get_properties();
        println!("{}", shape as u32);
        self.marks_properties[self.mark_size] = (center, size, color, shape as u32, self.mark_count);
        self.mark_count += 1;
        self.mark_size += 1;
    }

    pub fn get_marks_properties(&self) -> [MarkProperty; MAX_MARKS]
    {
        self.marks_properties
    }
}

