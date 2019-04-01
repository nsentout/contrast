use crate::MarkMacro;
use crate::markproperties::MarkProperties;
use crate::elapsed_time_float;
use crate::ANIM_DURATION;
use properties::position::Position;
use properties::color::Color;
use properties::size::Size;
use mark_macro_derive::MarkMacro;
use rand::Rng;

/// This is the type that will receive our shaders when we will want to render our point marks.
/// We could describe it this way to be clearer :
/// type VertexPoint = (old_center, target_center, start_center, old_size, target_size, start_size, old_color, target_color, 
///                      start_color, old_rotation, target_rotation, start_rotation, old_shape, target_shape, start_shape).
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
    pub fn from_integer(i : u32) -> Self {
        match i {
            0 => Shape::None,
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

    /// Simply returns a random Shape.
    pub fn rand<R: Rng>(rng: &mut R) -> Self {
        Shape::from_integer(rng.gen_range(1, 20))
    }
}

/// Structure used to handle animations.
/// 'old_value' refers to the value of the attribute before it has been modified by a setter.
/// 'target_value' refers to the value of the attribute after it has been modified by a setter.
/// 'start_anim' refers to the time in seconds when the animation started.
/// We will use those attributes in the shaders to perform a smooth animation.
/// To perform an animation, we make a linear interpolation between 'old_value' and 'target_value' in
/// the shaders and we use 'start_anim' to make it look smooth.
#[derive(Clone, Debug)]
pub(crate) struct AnimationAttribute<A> {
    pub(crate) old_value : A,
    pub(crate) target_value : A,
    pub(crate) start_anim : f32
}

/// This is the structure that describes the marks of type Point.
/// Each type of mark share some properties, that is an id, a size,
/// a color and a rotation. Those properties are described by the
/// attribute common_properties.
/// Point marks also have a position and a shape.
/// To support animations, we must add additionnal attributes for
/// each property of our mark. They are described in details in
/// AnimationAttribute.
/// Finally, we need a boolean telling us whether or not the mark
/// has already been displayed, allowing us to disable the
/// animation at the first display of our mark.
#[derive(MarkMacro, Clone, Debug)]
pub struct PointMark {
    pub(crate) common_properties : MarkProperties,
    pub(crate) current_center : Position,
    pub(crate) current_shape : Shape,

    pub(crate) size : AnimationAttribute<Size>,
    pub(crate) color : AnimationAttribute<Color>,
    pub(crate) rotation : AnimationAttribute<f32>,
    pub(crate) center : AnimationAttribute<Position>,
    pub(crate) shape : AnimationAttribute<Shape>,
    pub(crate) is_displayed : bool
}

impl PointMark {
    /// Simply returns a new instance of PointMark, initializing
    /// all attributes to their default value.
    pub fn new() -> Self {
        PointMark {
            common_properties : MarkProperties::new(),
            current_center : Position::default(),
            current_shape : Shape::None,
            size : AnimationAttribute {
                old_value : Size::default(),
                target_value : Size::default(),
                start_anim : 0.0
            },
            color : AnimationAttribute {
                old_value : Color::default(),
                target_value : Color::default(),
                start_anim : 0.0
            },
            rotation : AnimationAttribute {
                old_value : 0.0,
                target_value : 0.0,
                start_anim : 0.0
            },
            center : AnimationAttribute {
                old_value : Position::default(),
                target_value : Position::default(),
                start_anim : 0.0
            },
            shape : AnimationAttribute {
                old_value : Shape::None,
                target_value : Shape::None,
                start_anim : 0.0
            },
            is_displayed : false
        }
    }

    /// Converts a MarkPoint into a VertexPoint, which is a type understandable
    /// by the renderer, then returns it.
    pub fn to_vertex(&self) -> VertexPoint {
        (*self.center.old_value.to_array(), *self.center.target_value.to_array(), self.center.start_anim,
            *self.size.old_value.to_array(), *self.size.target_value.to_array(), self.size.start_anim,
            *self.color.old_value.to_array(), *self.color.target_value.to_array(), self.color.start_anim,
            self.rotation.old_value, self.rotation.target_value, self.rotation.start_anim,
            self.shape.old_value as u32, self.shape.target_value as u32, self.shape.start_anim)
    }

    /// Set the size of a mark. You can pass as argument a tuple of 2 floats or
    /// a Size directly.
    pub fn set_size<S : Into <Size>>(&mut self, size : S) -> &mut Self
    {
        // Change the size only if the previous animation is finished
        if elapsed_time_float() > self.size.start_anim + ANIM_DURATION  {
            self.size.old_value = self.common_properties.size;
            self.common_properties.size = size.into();
            self.size.target_value = self.common_properties.size;
            self.size.start_anim = elapsed_time_float();
        }
        // Prevent the animations done within the first second to negate the transition effect
        // Without this code, there would be an animation from the default size (0.0, 0.0)
        // to the actual size of the mark when we display the mark for the first time.
        else if !self.is_displayed {
            self.size.old_value = self.common_properties.size;
            self.common_properties.size = size.into();
            self.size.target_value = self.common_properties.size;
            self.size.start_anim = -10.0;
        }
        self
    }

    /// Set the color of a mark. You can pass as argument a tuple of 4 floats or
    /// a Color directly.
    pub fn set_color<C : Into <Color>>(&mut self, color : C) -> &mut Self
    {
        if elapsed_time_float() > self.color.start_anim + ANIM_DURATION  {
            self.color.old_value = self.common_properties.color;
            self.common_properties.color = color.into();
            self.color.target_value = self.common_properties.color;
            self.color.start_anim = elapsed_time_float();
        }
        else if !self.is_displayed {
            self.color.old_value = self.common_properties.color;
            self.common_properties.color = color.into();
            self.color.target_value = self.common_properties.color;
            self.color.start_anim = -10.0;
        }
        self
    }

    pub fn set_rotation(&mut self, rotation : f32) -> &mut Self
    {
        if elapsed_time_float() > self.rotation.start_anim + ANIM_DURATION  {
            self.rotation.old_value = self.common_properties.rotation;
            self.common_properties.rotation = rotation.into();
            self.rotation.target_value = self.common_properties.rotation;
            self.rotation.start_anim = elapsed_time_float();
        }
        else if !self.is_displayed {
            self.rotation.old_value = self.common_properties.rotation;
            self.common_properties.rotation = rotation;
            self.rotation.target_value = self.common_properties.rotation;
            self.rotation.start_anim = -10.0;
        }
        self
     }

    /// Set the position of a mark. You can pass as argument a tuple of 3 floats or
    /// a Position directly
    pub fn set_position<P : Into <Position>>(&mut self, center : P) -> &mut Self {
        if elapsed_time_float() > self.center.start_anim + ANIM_DURATION  {
            self.center.old_value = self.current_center;
            self.current_center = center.into();
            self.center.target_value = self.current_center;
            self.center.start_anim = elapsed_time_float();
        }
        else if !self.is_displayed {
            self.center.old_value = self.current_center;
            self.current_center = center.into();
            self.center.target_value = self.current_center;
            self.center.start_anim = -10.0;
        }
        self
    }

    pub fn set_shape(&mut self, shape : Shape) -> &mut Self {
        if elapsed_time_float() > self.shape.start_anim + ANIM_DURATION  {
            self.shape.old_value = self.current_shape;
            self.current_shape = shape;
            self.shape.target_value = self.current_shape;
            self.shape.start_anim = elapsed_time_float();
        }
        else if !self.is_displayed {
            self.shape.old_value = self.current_shape;
            self.current_shape = shape;
            self.shape.target_value = self.current_shape;
            self.shape.start_anim = -10.0;
        }
        self
    }

    pub fn get_position(&self) -> &Position {
        &self.current_center
    }

    pub fn get_shape(&self) -> &Shape {
        &self.current_shape
    }

    pub fn is_valid(&self) -> bool {
        self.common_properties.markid.valid
    }

    pub(crate) fn set_displayed(&mut self, is_displayed : bool) {
        self.is_displayed = is_displayed;
    }

}