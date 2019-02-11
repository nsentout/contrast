use pointmark::PointMark;
use linemark::LineMark;

/// General structures ///
#[derive(Copy, Clone, Debug)]
pub struct Color {
    pub r : f32,
    pub g : f32,
    pub b : f32,
    pub a : f32
}

#[derive(Copy, Clone, Debug)]
pub struct Position {
    pub x : f32,
    pub y : f32,
    pub z : f32
}

#[derive(Copy, Clone, Debug)]
pub struct Size {
    pub width : f32,
    pub height : f32
}

/// Mark common structures ///
#[derive(Copy, Clone, Debug)]
pub struct MarkProperties {
    pub id : u32,   // TODO: enlever les pub
    pub center: Position,
    pub size : Size,
    pub color: Color,
    pub rotation : f32,
}

impl MarkProperties {
    pub fn default() -> Self
    {
        MarkProperties {
            id : 0,
            center: Position { x : 0.0, y : 0.0, z : 0.0 },
            size : Size { width : 0.0, height : 0.0},
            color: Color { r : 0.0, g : 0.0, b : 0.0, a : 0.0 },
            rotation : 0.0,
        }
    }
}

// Main structure
pub struct Contrast {
    point_marks: Vec<PointMark>,
    line_marks: Vec<LineMark>,
    cpt_id : u32,
}

impl Contrast {
    pub fn init() -> Self { 
        Contrast {
            point_marks : Vec::<PointMark>::new(),
            line_marks: Vec::<LineMark>::new(),
            cpt_id : 0,
        }
    }

    fn get_point_mut_mark(&mut self, mark : PointMark) -> Option<&mut PointMark>
    {
        self.point_marks.iter_mut().find(|x| **x == mark)   // TODO: faire en O(1)
    }

    pub fn add_point_mark(&mut self) -> &mut PointMark {
        // Create a PointMark
        let mut point = PointMark::default();
        point.common_properties.id = self.cpt_id;
        self.cpt_id += 1;

        // Add it into the PointMark vector
        self.point_marks.push(point);

        // Return a mutable reference of this PointMark
        self.get_point_mut_mark(point).unwrap()
    }

    fn get_line_mut_mark(&mut self, mark : LineMark) -> Option<&mut LineMark>
    {
        self.line_marks.iter_mut().find(|x| **x == mark)
    }

    pub fn add_line_mark(&mut self) -> &mut LineMark {
        // Create LineMark
        let mut line = LineMark::default();
        line.common_properties.id = self.cpt_id;
        self.cpt_id += 1;

        // Add it into the LineMark vector
        self.line_marks.push(line);

        // Return a mutable reference of this LineMark
        self.get_line_mut_mark(line).unwrap()
    }
}