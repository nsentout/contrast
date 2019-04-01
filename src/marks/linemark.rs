use crate::MarkMacro;
use crate::markproperties::MarkProperties;
use properties::position::Position;
use mark_macro_derive::MarkMacro;

/// This is the type that will receive our shaders when we will want to render our line marks.
/// We could describe it this way to be more clear :
/// type VertexSubLine = (size, color, rotation, origin, target, previous, next, thickness, line_mode).
pub type VertexSubLine = ([f32; 2], [f32; 4], f32, [f32; 3], [f32; 3], [f32; 3], [f32; 3], f32, u32);


/// Those are the different ways we shoud be able to
/// draw lines.
#[derive(Debug,Copy,Clone)]
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
#[derive(MarkMacro, Clone, Debug)]
pub struct LineMark {
    pub(crate) common_properties : MarkProperties,
    pub(crate) points : Vec<Position>,
    pub(crate) thickness : f32,
    pub(crate) mode : LineMode
}

impl LineMark {
    /// Simply returns a new instance of LineMark, initializing
    /// all attributes to their default value.
    pub fn new() -> Self {
        LineMark {
            common_properties : MarkProperties::new(),
            points : Vec::<Position>::new(),
            thickness : 0.0,
            mode : LineMode::Linear
        }
    }

    /// Converts a LineMark into a VertexSubLine, which is a type
    /// understandable by the renderer.
    pub fn to_subline(&self) -> Vec<VertexSubLine> {
        let mut sublines : Vec<VertexSubLine> = Vec::<VertexSubLine>::new();
        if self.points.len()>0 {
            let mode = &self.mode;
            let mut previous = self.points[0];
            let mut origin = self.points[0];
            let mut target = self.points[0];
            for next in self.points.clone() {
                let vl : VertexSubLine = (*self.common_properties.size.to_array(),
                *self.common_properties.color.to_array(), self.common_properties.rotation,
                *origin.to_array(), *target.to_array(), *previous.to_array(), *next.to_array(),self.thickness, *mode as u32);
                sublines.push(vl);
                previous = origin;
                origin = target;
                target = next;
            }
            let vl : VertexSubLine = (*self.common_properties.size.to_array(),
            *self.common_properties.color.to_array(), self.common_properties.rotation,
            *origin.to_array(), *target.to_array(), *previous.to_array(), *self.points[self.points.len()-1].to_array(),self.thickness, *mode as u32);
            sublines.push(vl);
            if sublines.len() > 0 {
                sublines.remove(0);
            }
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

    pub fn set_mode(&mut self, mode : LineMode) -> &mut Self {
        self.mode = mode;
        self
    }

    pub(crate) fn get_points_mut(&mut self) -> &mut Vec<Position> {
        &mut self.points
    }

}
