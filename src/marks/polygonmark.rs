use crate::MarkMacro;
use crate::properties::markproperties::MarkProperties;
use crate::properties::position::Position;
use mark_macro_derive::MarkMacro;

/// This is the structure that describes the marks of type Polygon.
/// Each type of mark share some properties, that is an id, a size,
/// a color and a rotation. Those properties are described by the
/// attribute common_properties.
/// Polygon marks also have a vector of positions representing its points,
/// a stroke width and a boolean to indicate whether or not we must
/// draw the stroke.
#[derive(MarkMacro, Debug)]
pub struct PolygonMark {
    pub common_properties : MarkProperties,
    points : Vec<Position>,
    stroke_width : f32,
    fill : bool
}

impl PolygonMark {
    /// Simply returns a new instance of PolygonMark, initializing
    /// all attributes to their default value, except the id.
    pub fn new(id : usize) -> Self {
        PolygonMark {
            common_properties : MarkProperties::new(id),
            points : Vec::<Position>::new(),
            stroke_width : 0.0,
            fill : false
        }
    }

    pub fn add_point(&mut self, x : f32, y : f32, z : f32) -> &mut Self {
        self.points.push(Position { x, y, z });
        self
    }

    pub fn set_stroke_width(&mut self, stroke_width : f32) -> &mut Self {
        self.stroke_width = stroke_width;
        self
    }

    pub fn set_fill(&mut self) -> &mut Self {
        self.fill = true;
        self
    }

    pub fn set_empty(&mut self) -> &mut Self {
        self.fill = false;
        self
    }
}