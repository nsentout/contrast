use crate::elapsed_time_float;
use crate::ANIM_DURATION;
use properties::position::Position;
use properties::color::Color;
use properties::size::Size;
use properties::markid::MarkId;
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

/// Macro allowing to set an animated property of our point mark.
/// '$point' is the mark, $property is the property we want to animate,
/// and $param is the value we want to give our $property.
/// Example : anim_set!(point_1, color, Color::red()) will perform an
/// animation to the mark point_1 from its previous color to the red color.
macro_rules! anim_set {
    ($point:ident, $property:ident, $param:expr) => {
        {
            // Change the property only if the previous animation is finished
            if elapsed_time_float() > $point.$property.start_anim + ANIM_DURATION && $point.is_displayed {
                $point.$property.old_value = $point.$property.target_value;
                $point.$property.target_value = $param;
                $point.$property.start_anim = elapsed_time_float();
            }
            // Prevent the animations done within the first second to negate the transition effect
            // Without this code, there would be an animation from the default value of the property
            // to the targeted property when we display the mark for the first time.
            else if !$point.is_displayed {
                $point.$property.old_value = $point.$property.target_value;
                $point.$property.target_value = $param;
                // Saying that the animation started 10 seconds before the launch of the timer
                // ensures there will be no animation.
                $point.$property.start_anim = -10.0;
            }
            $point
        }
    }
}

/// Structure used to handle animations.
/// 'old_value' refers to the value of the attribute before the start of an animation.
/// 'target_value' refers to the value of the attribute at the end of an animation.
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
/// Each type of mark share some properties, that is an id and a
/// color.
/// Point marks also have a size, a rotation, a position and a shape.
/// To support animations, we must use additionnal attributes for
/// each property of our mark. They are described in details in
/// AnimationAttribute.
/// Finally, we need a boolean telling us whether or not the mark
/// has already been displayed, allowing us to disable the
/// animation at the first display of our mark.
#[derive(Clone, Debug)]
pub struct PointMark {
    pub(crate) markid : MarkId,
    pub(crate) color : AnimationAttribute<Color>,
    pub(crate) size : AnimationAttribute<Size>,
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
            markid : MarkId::new(),
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

    /// Converts a PointMark into a VertexPoint, which is a type understandable
    /// by the renderer, then returns it.
    pub fn to_vertex(&self) -> VertexPoint {
        (*self.center.old_value.to_array(), *self.center.target_value.to_array(), self.center.start_anim,
            *self.size.old_value.to_array(), *self.size.target_value.to_array(), self.size.start_anim,
            *self.color.old_value.to_array(), *self.color.target_value.to_array(), self.color.start_anim,
            self.rotation.old_value, self.rotation.target_value, self.rotation.start_anim,
            self.shape.old_value as u32, self.shape.target_value as u32, self.shape.start_anim)
    }

    /// Set the color of a mark. You can pass as argument a tuple of 4 floats or
    /// a Color directly.
    pub fn set_color<C : Into <Color>>(&mut self, color : C) -> &mut Self {
        anim_set!(self, color, color.into())
    }

    /// Set the size of a mark. You can pass as argument a tuple of 2 floats or
    /// a Size directly.
    pub fn set_size<S : Into <Size>>(&mut self, size : S) -> &mut Self {
        anim_set!(self, size, size.into())
    }

    pub fn set_rotation(&mut self, rotation : f32) -> &mut Self {
        anim_set!(self, rotation, rotation)
    }

    /// Set the position of a mark. You can pass as argument a tuple of 2 or 3 floats, or
    /// a Position directly
    pub fn set_position<P : Into <Position>>(&mut self, center : P) -> &mut Self {
        anim_set!(self, center, center.into())
    }

    pub fn set_shape(&mut self, shape : Shape) -> &mut Self {
        anim_set!(self, shape, shape)
    }

    pub fn get_id(&self) -> MarkId {
        self.markid
    }

    pub fn get_size(&self) -> Size {
        self.size.target_value
    }

    pub fn get_color(&self) -> Color {
        self.color.target_value
    }

    pub fn get_rotation(&self) -> f32 {
        self.rotation.target_value
    }

    pub fn get_position(&self) -> &Position {
        &self.center.target_value
    }

    pub fn get_shape(&self) -> &Shape {
        &self.shape.target_value
    }

    pub fn get_layer_index(&self) -> usize {
        self.markid.layer_index
    }

    pub fn is_valid(&self) -> bool {
        self.markid.valid
    }

    pub(crate) fn set_displayed(&mut self, is_displayed : bool) {
        self.is_displayed = is_displayed;
    }

}