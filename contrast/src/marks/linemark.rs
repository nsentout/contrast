use crate::MarkMacro;
use properties::position::Position;
use properties::color::Color;
use properties::markid::MarkId;
use mark_macro_derive::MarkMacro;

/// This is the type that will receive our shaders when we will want to render our line marks.
/// We could describe it this way to be more clear :
/// type VertexSubLine = (size, color, rotation, origin, target, previous, next, thickness, line_mode).
pub type VertexSubLine = ([f32; 4], [f32; 3], [f32; 3], [f32; 3], [f32; 3], f32);


/// This is the structure that describes the marks of type Line (or polyline).
/// Each type of mark share some properties, that is an id and a color.
/// Line marks also have a vector of positions representing its points,
/// a thickness and a mode to draw them differently.
#[derive(MarkMacro, PartialEq, Clone, Debug)]
pub struct LineMark {
    pub(crate) markid : MarkId,
    pub(crate) color : Color,
    pub(crate) points : Vec<Position>,
    pub(crate) thickness : f32,
}

impl LineMark {
    /// Simply returns a new instance of LineMark, initializing
    /// all attributes to their default value.
    pub fn new() -> Self {
        LineMark {
            markid : MarkId::new(),
            color : Color::default(),
            points : Vec::<Position>::new(),
            thickness : 1.0,
        }
    }

    /// Converts a LineMark into a VertexSubLine, which is a type
    /// understandable by the renderer.
    pub fn to_subline(&self) -> Vec<VertexSubLine> {
        let mut sublines : Vec<VertexSubLine> = Vec::<VertexSubLine>::new();
        if self.points.len()>=2 {
            let mut previous = self.points[0];
            let mut origin = self.points[0];
            let mut target = self.points[0];
            for next in self.points.clone() {
                let vl : VertexSubLine = (
                *self.color.to_array(), *origin.to_array(), *target.to_array(),
                *previous.to_array(), *next.to_array(),self.thickness);
                sublines.push(vl);
                previous = origin;
                origin = target;
                target = next;
            }
            let vl : VertexSubLine = (
            *self.color.to_array(), *origin.to_array(), *target.to_array(),
            *previous.to_array(), *self.points[self.points.len()-1].to_array(),
            self.thickness);
            sublines.push(vl);
            sublines.remove(0);
            sublines.remove(0);
        }
        sublines
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

    pub fn get_thickness(&self) -> f32 {
        self.thickness
    }

    pub(crate) fn get_points_mut(&mut self) -> &mut Vec<Position> {
        &mut self.points
    }

    pub fn get_points(&mut self) -> &mut Vec<Position> {
        &mut self.points
    }

}
