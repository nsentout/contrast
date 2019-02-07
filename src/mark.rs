/// General structures ///
#[derive(Copy, Clone, Debug)]
struct Color {
    r : f32,
    g : f32,
    b : f32,
    a : f32
}

#[derive(Copy, Clone, Debug)]
struct Position {
    x : f32,
    y : f32,
    z : f32
}

#[derive(Copy, Clone, Debug)]
struct Size {
    width : f32,
    height : f32
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
        self.point_marks.iter_mut().find(|x| **x == mark)
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

/// Mark common structures ///
#[derive(Copy, Clone, Debug)]
pub struct MarkProperties {
    id : u32,
    center: Position,
    size : Size,
    color: Color,
    rotation : f32,
}

impl MarkProperties {
    fn default() -> Self
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

// unused
#[derive(Copy, Clone, Debug)]
pub enum Mark {
    Point(PointMark),
    Line(LineMark),
    //Polygon,
    //Area,
    //Text 
}

// unused
pub trait MarkTrait
{
    fn set_position(&mut self, x : f32, y : f32, z : f32) -> & mut Mark;
    //fn set_size(&mut self, size : Size) -> &mut Self;
    //fn set_color(&mut self, color : Color) -> &mut Self;
    //fn set_rotation(&mut self, rotation : f32) -> &mut Self;
}

/*
// TODO: faire en sorte que toutes les types de marques partagent certaines fonctions (set_position, set_size, ...)
impl MarkTrait for Mark {
    fn set_position(&mut self, x : f32, y : f32, z : f32) -> &mut Mark {
        //self.common_properties.center = Position { x , y, z };
        //self
        match self {   
            Mark::Point(p) => p.set_position(x, y, z),
            Mark::Line(l) =>  l.set_position(x, y, z)
        }
    }
}
*/

/// Mark Point ///
pub type VertexPoint = ([f32; 3], [f32; 2], [f32; 4], f32, u32, f32, f32);
// position; size; color; rotation; shape; selection_angle; start_radius

#[derive(Copy, Clone, Debug)]
pub enum Shape {
    None = 0,
    Rectangle = 1,
    Triangle = 2,
    /*Circle = 3,
    Point = 4,
    Squircle = 5,
    Diamond = 6,
    Donut = 7,
    Pin = 8,
    Club = 9,
    Heart = 10,
    Spade = 11,
    Chevron = 12,
    Clover = 13,
    Ring = 14,
    Tag = 15,
    Cross = 16,
    Asterisk = 17,
    Infinity = 18,
    Arrow = 19*/
}

#[derive(Copy, Clone, Debug)]
pub struct PointMark {
    common_properties : MarkProperties,
    shape : Shape,
    selection_angle : f32,
    start_radius : f32
}

impl PointMark {
    fn default() -> Self {
        PointMark {
            common_properties : MarkProperties::default(),
            shape : Shape::None,
            selection_angle : 0.0,
            start_radius : 0.0
        }
    }

    pub fn set_position(&mut self, x : f32, y : f32, z : f32) -> &mut Self {
        self.common_properties.center = Position { x, y, z };
        self
    }

    pub fn set_shape(&mut self, shape : Shape) -> &mut Self {
        self.shape = shape;
        self
    }

    pub fn set_selection_angle(&mut self, selection_angle : f32) -> &mut Self {
        self.selection_angle = selection_angle;
        self
    }

    pub fn set_start_radius(&mut self, start_radius : f32) -> &mut Self {
        self.start_radius = start_radius;
        self
    }
}

impl std::cmp::PartialEq for PointMark {
    fn eq(&self, mark: &PointMark) -> bool {
        self.common_properties.id == mark.common_properties.id
    }
}

/// Mark Line ///
#[derive(Copy, Clone, Debug)]
pub enum LineMode {
    Linear,
    Dashed,
    Dotted
}

#[derive(Copy, Clone, Debug)]
pub struct LineMark {
    common_properties : MarkProperties,
    points : u32,   // TODO: changer par une liste
    thickness : f32,
    mode : LineMode
}

impl LineMark {
    fn default() -> Self {
        LineMark {
            common_properties : MarkProperties::default(),
            points : 0,
            thickness : 0.0,
            mode : LineMode::Linear
        }
    }

    pub fn set_position(&mut self, x : f32, y : f32, z : f32) -> &mut Self {
        self.common_properties.center = Position { x, y, z };
        self
    }

    pub fn set_thickness(&mut self, thickness : f32) -> &mut Self {
        self.thickness = thickness;
        self
    }

    pub fn set_mode(&mut self, mode : LineMode) -> &mut Self {
        self.mode = mode;
        self
    }
}

impl std::cmp::PartialEq for LineMark {
    fn eq(&self, mark: &LineMark) -> bool {
        self.common_properties.id == mark.common_properties.id
    }
}
