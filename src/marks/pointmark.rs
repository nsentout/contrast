use crate::MarkMacro;
use crate::markproperties::MarkProperties;
use crate::TIMER;
use crate::elapsed_time_float;
use properties::position::Position;
use properties::color::Color;
use properties::size::Size;
use mark_macro_derive::MarkMacro;
use rand::Rng;

/// This is the type that will receive our shaders when we will want to render our point marks.
/// We could describe it this way to be more clear :
/// type VertexPoint = (position, target_pos, start_pos, size, target_size, start_size, color, target_color, 
///                      start_color, rotation, target_rotation, start_rotation, shape, target_shape, start_shape).
pub type VertexPoint = ([f32; 3], [f32; 3], f32, [f32; 2], [f32; 2], f32, [f32; 4], [f32; 4], f32, f32, f32, f32, u32, u32, f32);

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

impl Shape {
    pub fn rand<R: Rng>(rng: &mut R) -> Self {
        match rng.gen_range(1, 20) {
            1 => Shape::Rectangle,
            2 => Shape::Triangle,
            3 => Shape::Circle,
            4 => Shape::Point,
            5 => Shape::Squircle,
            6 => Shape::Diamond,
            7 => Shape::Donut,
            8 => Shape::Pin,
            9 => Shape::Club,
            10 => Shape::Heart,
            11 => Shape::Spade,
            12 => Shape::Chevron,
            13 => Shape::Clover,
            14 => Shape::Ring,
            15 => Shape::Tag,
            16 => Shape::Cross,
            17 => Shape::Asterisk,
            18 => Shape::Infinity,
            19 => Shape::Arrow,
            _ => Shape::None
        }
    }
}

/// old_attr : attr before the set_attr
/// target_attr : attr after the set_attr
/// start_attr : time in milliseconds when the animation started
#[derive(Clone, Debug)]
pub(crate) struct AttributeStates<A> {
    pub(crate) old_attr : A,
    pub(crate) target_attr : A,
    pub(crate) start_attr : f32
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
    pub(crate) current_center : Position,
    pub(crate) current_shape : Shape,

    pub(crate) size : AttributeStates<Size>,
    pub(crate) color : AttributeStates<Color>,
    pub(crate) rotation : AttributeStates<f32>,
    pub(crate) center : AttributeStates<Position>,
    pub(crate) shape : AttributeStates<Shape>,
    pub(crate) displayed : bool
}

impl PointMark {
    /// Simply returns a new instance of PointMark, initializing
    /// all attributes to their default values.
    pub fn new() -> Self {
        PointMark {
            common_properties : MarkProperties::new(),
            current_center : Position::default(),
            current_shape : Shape::None,
            size : AttributeStates {
                old_attr : Size::default(),
                target_attr : Size::default(),
                start_attr : 0.0
            },
            color : AttributeStates {
                old_attr : Color::default(),
                target_attr : Color::default(),
                start_attr : 0.0
            },
            rotation : AttributeStates {
                old_attr : 0.0,
                target_attr : 0.0,
                start_attr : 0.0
            },
            center : AttributeStates {
                old_attr : Position::default(),
                target_attr : Position::default(),
                start_attr : 0.0
            },
            shape : AttributeStates {
                old_attr : Shape::None,
                target_attr : Shape::None,
                start_attr : 0.0
            },
            displayed : false
        }
    }

    /// Converts a MarkPoint into a VertexPoint, which is a type
    /// understandable by the renderer.
    pub fn as_static_vertex(&self) -> VertexPoint {
        (*self.current_center.to_array(), *self.current_center.to_array(), 0.0,
            *self.common_properties.size.to_array(), *self.common_properties.size.to_array(), 0.0,
            *self.common_properties.color.to_array(), *self.common_properties.color.to_array(), 0.0,
            self.common_properties.rotation, self.common_properties.rotation, 0.0,
            self.current_shape as u32, self.current_shape as u32, 0.0)
    }

    pub fn as_anim_vertex(&self) -> VertexPoint {
        (*self.center.old_attr.to_array(), *self.center.target_attr.to_array(), self.center.start_attr,
            *self.size.old_attr.to_array(), *self.size.target_attr.to_array(), self.size.start_attr,
            *self.color.old_attr.to_array(), *self.color.target_attr.to_array(), self.color.start_attr,
            self.rotation.old_attr, self.rotation.target_attr, self.rotation.start_attr,
            self.shape.old_attr as u32, self.shape.target_attr as u32, self.shape.start_attr)
    }

    pub fn set_size<S : Into <properties::size::Size>>(&mut self, size : S) -> &mut Self
    {
        self.size.old_attr = self.common_properties.size;
        self.common_properties.size = size.into();
        self.size.target_attr = self.common_properties.size;
        self.size.start_attr = elapsed_time_float();
        self
    }

    pub fn set_color<C : Into <properties::color::Color>>(&mut self, color : C) -> &mut Self
    {
        self.color.old_attr = self.common_properties.color;
        self.common_properties.color = color.into();
        self.color.target_attr = self.common_properties.color;
        self.color.start_attr = elapsed_time_float();
        self
    }

    pub fn set_rotation(&mut self, rotation : f32) -> &mut Self
    {
        self.rotation.old_attr = self.common_properties.rotation;
        self.common_properties.rotation = rotation;
        self.rotation.target_attr = self.common_properties.rotation;
        self.rotation.start_attr = elapsed_time_float();
        self
     }

    /// Set the position of a mark. You can pass as argument a tuple of 3 floats or
    /// a Position directly
    pub fn set_position<P : Into <Position>>(&mut self, position : P) -> &mut Self {
        self.center.old_attr = self.current_center;
        self.current_center = position.into();
        self.center.target_attr = self.current_center;
        self.center.start_attr = elapsed_time_float();
        self
    }

    pub fn set_shape(&mut self, shape : Shape) -> &mut Self {
        self.shape.old_attr = self.current_shape;
        self.current_shape = shape;
        self.shape.target_attr = self.current_shape;
        self.shape.start_attr = elapsed_time_float();
        self
    }

    pub fn get_position(&self) -> &Position {
        &self.current_center
    }

    pub fn get_x(&self) -> f32 {
        self.current_center.x
    }

    pub fn get_y(&self) -> f32 {
        self.current_center.y
    }

    pub fn get_z(&self) -> f32 {
        self.current_center.z
    }

    pub fn get_shape(&self) -> &Shape {
        &self.current_shape
    }

    pub fn is_valid(&self) -> bool {
        self.common_properties.markid.valid
    }

    pub(crate) fn is_already_displayed(&self) -> bool {
        self.displayed
    }

    pub(crate) fn set_displayed(&mut self, displayed : bool) {
        self.displayed = displayed;
    }

    /// Prevent the animations done within the first second to negate the transition effect
    pub(crate) fn prepare_first_display(&mut self) {
        self.size.old_attr = self.common_properties.size;
        self.color.old_attr = self.common_properties.color;
        self.rotation.old_attr = self.common_properties.rotation;
        self.center.old_attr = self.current_center;
        self.shape.old_attr = self.current_shape;
    }
}