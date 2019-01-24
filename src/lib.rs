type Position2D = [f32; 2];
type Size = [f32; 2];
type RGBColor = [f32; 3];
pub type Vertex = (Position2D, Size, RGBColor, u32);

pub trait Mark {
    fn new(center : Position2D, size : Size, color : RGBColor) -> Self;
    fn get_vertices(&self) -> [Vertex; 1];
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

    fn get_vertices(&self) -> [Vertex; 1]
    {
        [(self.center, self.size, self.color, 1)]
    }
}
