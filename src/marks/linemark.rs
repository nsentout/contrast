use crate::MarkMacro;
use crate::markproperties::MarkProperties;
use properties::position::Position;
use mark_macro_derive::MarkMacro;

/// This is the type that will receive our shaders when we will want to render our line marks.
/// We could describe it this way to be more clear :
/// type VertexLine = (size, color, rotation, points, thickness, line_mode).
pub type VertexLine = ([f32; 2], [f32; 4], f32, Vec<Position>, f32, u32);


/// Those are the different ways we shoud be able to
/// draw lines.
#[derive(Debug)]
pub enum LineMode {
    Linear,
    Dashed,
    Dotted
}

/// This is the structure that describes the marks of type Line (or polyline).
/// Each type of mark share some properties, that is an id, a size,
/// a color and a rotation. Those properties are described by the
/// attribute common_properties.
/// Line marks also have a vector of positions representing its points,
/// a thickness and a mode to draw them differently.
#[derive(MarkMacro, Debug)]
pub struct LineMark {
    pub(crate) common_properties : MarkProperties,
    points : Vec<Position>,
    thickness : f32,
    mode : LineMode
}

impl LineMark {
    /// Simply returns a new instance of LineMark, initializing
    /// all attributes to their default value, except the id.
    pub fn new(id : usize) -> Self {
        LineMark {
            common_properties : MarkProperties::new(id),
            points : Vec::<Position>::new(),
            thickness : 0.0,
            mode : LineMode::Linear
        }
    }
    
    /// Add a point to a line. You can pass as argument a tuple of 3 floats or
    /// a Position directly
    pub fn add_point<P : Into <Position>>(&mut self, point : P) -> &mut Self {
        self.points.push(point.into());
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