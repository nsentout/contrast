use crate::MarkMacro;
use crate::markproperties::MarkProperties;
use properties::position::Position;
use mark_macro_derive::MarkMacro;

/// This is the type that will receive our shaders when we will want to render our point marks.
/// We could describe it this way to be more clear :
/// type VertexPoint = (position, size, color, rotation, shape, selection_angle, start_radius).
pub type VertexPoint = ([f32; 3], [f32; 2], [f32; 4], f32, u32, f32, f32);

/// This enum describes every shape that should be drawable.
#[derive(Copy, Clone, Debug)]
pub enum Shape {
    None = 0,
    Rectangle = 1,
    Triangle = 2,
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
    Arrow = 19
}

/// This is the structure that describes the marks of type Point.
/// Each type of mark share some properties, that is an id, a size,
/// a color and a rotation. Those properties are described by the
/// attribute common_properties.
/// Point marks also have a position, a shape and a selection angle
/// and start radius for some specific shapes.
#[derive(MarkMacro, Clone, Debug)]
pub struct PointMark {
    pub(crate) common_properties : MarkProperties,
    pub(crate) center : Position,
    pub(crate) shape : Shape,
    pub(crate) selection_angle : f32,
    pub(crate) start_radius : f32,
}

impl PointMark {
    /// Simply returns a new instance of PointMark, initializing
    /// all attributes to their default values.
    pub fn new() -> Self {
        PointMark {
            common_properties : MarkProperties::new(),
            center : Position::default(),
            shape : Shape::None,
            selection_angle : 0.0,
            start_radius : 0.0
        }
    }

    /// Converts a MarkPoint into a VertexPoint, which is a type
    /// understandable by the renderer.
    pub fn as_vertex(&self) -> VertexPoint {
        (*self.center.to_array(), *self.common_properties.size.to_array(),
            *self.common_properties.color.to_array(), self.common_properties.rotation,
            self.shape as u32, self.selection_angle, self.start_radius)
    }

    /// Set the position of a mark. You can pass as argument a tuple of 3 floats or
    /// a Position directly
    pub fn set_position<P : Into <Position>>(&mut self, position : P) -> &mut Self {
        self.center = position.into();
        self
    }

    pub fn set_x(&mut self, x : f32) -> &mut Self {
        self.center.x = x;
        self
    }

    pub fn set_y(&mut self, y : f32) -> &mut Self {
        self.center.y = y;
        self
    }

    pub fn set_z(&mut self, z : f32) -> &mut Self {
        self.center.z = z;
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

    pub fn get_position(&self) -> &Position {
        &self.center
    }

    pub fn get_x(&self) -> f32 {
        self.center.x
    }

    pub fn get_y(&self) -> f32 {
        self.center.y
    }

    pub fn get_z(&self) -> f32 {
        self.center.z
    }

    pub fn get_shape(&self) -> &Shape {
        &self.shape
    }

    pub fn get_selection_angle(&self) -> f32 {
        self.selection_angle
    }

    pub fn get_start_radius(&self) -> f32 {
        self.start_radius
    }
}