type Position2D = [f32; 2];
type Size = [f32; 2];
type RGBColor = [f32; 3];
pub type Vertex = (Position2D, Size, RGBColor, u32);

pub trait Mark {
    fn new(center : Position2D, size : Size, color : RGBColor) -> Self;
    fn get_position(&self) -> Position2D;
    fn get_size(&self) -> Size;
    fn get_color(&self) -> RGBColor;
    fn get_vertices(&self) -> (Position2D, Size, RGBColor);
    /*fn set_position(&mut self, new_center : Position2D) -> Self;
    fn set_size(&mut self, new_size : Size) -> Self;
    fn set_color(&mut self, new_color : RGBColor) -> Self;*/
}


pub struct MarkRectangle
{
    center: Position2D,
    size : Size,
    color: RGBColor,
}

impl Mark for MarkRectangle {
    fn new(center : Position2D, size : Size, color : RGBColor) -> MarkRectangle {
        let polygon = MarkRectangle {
            center,
            size,
            color,
        };

        polygon
    }

    fn get_position(&self) -> Position2D
    {
        self.center
    }

    fn get_size(&self) -> Size
    {
        self.size
    }

    fn get_color(&self) -> RGBColor
    {
        self.color
    }

    fn get_vertices(&self) -> (Position2D, Size, RGBColor)
    {
        (self.center, self.size, self.color)
    }
/*
    fn set_position(&mut self, new_center : Position2D) -> Self
    {
        self.center = new_center;
        self
    }

    fn set_size(&mut self, new_size : Size) -> Self
    {
        self.size = new_size;
        *self
    }

    fn set_color(&mut self, new_color : RGBColor) -> Self
    {
        self.color = new_color;
        *self
    }
*/
}

/***********************************************************************************/

const MAX_MARKS: usize = 10;

pub struct MarksManager {
    marks : [Vertex; MAX_MARKS],    // comment faire pour que ce soit dynamique ?
    mark_count : u32,
    mark_size : usize,
}

impl MarksManager {
    pub fn create_marksmanager() -> Self
    {
        // je ne sais pas comment faire autrement, au secours
        let mut marks : [Vertex; MAX_MARKS] = [([0.0, 0.0], [0.0, 0.0], [0.0, 0.0, 0.0], 0); MAX_MARKS];

        MarksManager {
            marks,
            mark_count : 0,
            mark_size : 0,
        }
    }

    pub fn add_mark(&mut self, mark : MarkRectangle)
    {
        let (center, size, color) = mark.get_vertices();
        self.marks[self.mark_size] = (center, size, color, self.mark_count);
        self.mark_count += 1;
        self.mark_size += 1;
    }

    pub fn get_marks(&self) -> [Vertex; MAX_MARKS]
    {
        self.marks
    }
}

