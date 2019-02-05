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

// structure temporaire pour tester l'architecture
pub struct Contrast {
    marks: Vec<Mark>
}

impl Contrast {
    pub fn init() -> Self { 
        Contrast {
            marks : Vec::<Mark>::new()
        }
    }

    // TODO: incrementer l'id quand on ajoute une marque

    pub fn add_point_mark(&mut self) -> PointMark {
        let point = PointMark::default();
        let mark = Mark::Point(point);

        self.marks.push(mark);
        point
    }

    pub fn add_line_mark(&mut self) -> LineMark {
        let line = LineMark::default();
        let mark = Mark::Line(line);

        self.marks.push(mark);
        line
    }
/*
    pub fn add_line_mark(&mut self) -> Mark {
        let mark = Mark::Line(LineMark::default());

        self.marks.push(mark);
        *self.marks.get(0).unwrap()
    }
*/
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

pub enum Mark {
    Point(PointMark),
    Line(LineMark),
    //Polygon,
    //Area,
    //Text 
}

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
#[derive(Copy, Clone, Debug)]
pub enum Shape {
    None = 0,
    Rectangle = 1,
    /*Triangle = 2,
    Circle = 3,
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
