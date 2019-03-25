use crate::MarkMacro;
use properties::color::Color;
use properties::size::Size;
use properties::position::Position;
use crate::marks::pointmark::PointMark;
use crate::marks::linemark::LineMark;
use crate::marks::textmark::TextMark;
use self::MarkTy::*;
use std::slice::Iter;


/// Union of every type of mark.
#[derive(Clone)]
pub enum Mark {
	Point(PointMark),
	Line(LineMark),
    Text(TextMark)
}

/// Pure enum to distinguish the type of marks.
#[derive(Clone, Hash, Eq, PartialEq, Debug)]
pub enum MarkTy
{
    Point,
    Line,
    Text
}

impl MarkTy
{
    pub fn values() -> Iter<'static, MarkTy>
    {
        static MARKS: [MarkTy;  3] = [Point, Line, Text];
        MARKS.into_iter()
    }
}

/// Macro calling the getter $get of the MarkMacro trait on the mark $mark.
/// Example : mark_get!(mark_point_1, get_color) calls the get_color method
/// implemented in the procedural macro "mark_macro_derive" that returns
/// the color of "mark mark_point_1"
macro_rules! mark_get {
    ($mark:ident, $get:ident) => (
        match $mark {
            Mark::Point(p) => p.$get(),
            Mark::Line(l)  => l.$get(),
            Mark::Text(t)  => t.$get()
        }
    )
}

/// Macro calling the setter $set of the MarkMacro trait (with parameter $param) on the mark $mark.
/// Example : mark_set!(mark_point_1, set_color, (1.0, 0.0, 0.0, 1.0)) calls the set_color method
/// implemented in the procedural macro "mark_macro_derive" that set the color of "mark mark_point_1"
/// to (1.0, 0.0, 0.0, 1.0).
macro_rules! mark_set {
    ($mark:ident, $set:ident, $param:expr) => {
        {
            match $mark {
                Mark::Point(p) => { p.$set($param); } ,
                Mark::Line(l)  => { l.$set($param); },
                Mark::Text(t)  => { t.$set($param); }
            }
            $mark
        }
    }
}

/// Macro able to cast a Mark into any other type of Mark (Point, Line, Polygon, Text)
/// that returns a reference of it wrapped into an Option.
macro_rules! cast {
    ($mark:ident, $type:ident) => (
        if let Mark::$type(t) = $mark {
            Some(t)
        }
        else {
            None
        }
    )
}

/// Macro able to cast a Mark into any other type of Mark (Point, Line, Polygon, Text)
/// that returns a mutable reference of it wrapped into an Option.
macro_rules! cast_mut {
    ($mark:ident, $type:ident) => (
        if let Mark::$type(ref mut t) = $mark {
            Some(t)
        }
        else {
            None
        }
    )
}

/// Macro able to cast a Mark into any other type of Mark (Point, Line, Polygon, Text)
/// that either returns a reference of it if the Mark is the good type, or panic.
macro_rules! cast_unchecked {
    ($mark:ident, $type:ident) => (
        if let Mark::$type(t) = $mark {
            t
        }
        else {
            panic!("An error occured when casting a mark!! Mark type incorrect!")
        }
    )
}

/// Macro able to cast a Mark into any other type of mark (Point, Line, Polygon, Text)
/// that either returns a mutable reference of it if the Mark is the good type, or panic.
macro_rules! cast_mut_unchecked {
    ($mark:ident, $type:ident) => (
        if let Mark::$type(ref mut t) = $mark {
            t
        }
        else {
            panic!("An error occured when casting a mark!! Mark type incorrect!")
        }
    )
}


impl Mark {
    pub fn as_point_mark(&self) -> Option<&PointMark> {
        cast!(self, Point)
    }

    pub fn as_point_mark_mut(&mut self) -> Option<&mut PointMark> {
        cast_mut!(self, Point)
    }

    pub fn as_point_mark_unchecked(&self) -> &PointMark {
        cast_unchecked!(self, Point)
    }

    pub fn as_point_mark_mut_unchecked(&mut self) -> &mut PointMark {
        cast_mut_unchecked!(self, Point)
    }

    pub fn as_line_mark(&self) -> Option<&LineMark> {
        cast!(self, Line)
    }

    pub fn as_line_mark_mut(&mut self) -> Option<&mut LineMark> {
        cast_mut!(self, Line)
    }

    pub fn as_line_mark_unchecked(&self) -> &LineMark {
        cast_unchecked!(self, Line)
    }

    pub fn as_line_mark_mut_unchecked(&mut self) -> &mut LineMark {
        cast_mut_unchecked!(self, Line)
    }

    pub fn as_text_mark(&self) -> Option<&TextMark> {
        cast!(self, Text)
    }

    pub fn as_text_mark_mut(&mut self) -> Option<&mut TextMark> {
        cast_mut!(self, Text)
    }

    pub fn as_text_mark_unchecked(&self) -> &TextMark {
        cast_unchecked!(self, Text)
    }

    pub fn as_text_mark_mut_unchecked(&mut self) -> &mut TextMark {
        cast_mut_unchecked!(self, Text)
    }

    /// Move the mark according to the 'position'. Used by Layer to move
    /// all his marks.
    /// Example : if 'position' is (50.0, 0.0, 0.0), every point of the mark 
    /// will move 50 pixels to the right.
    pub fn move_of<P : Into <Position>>(&mut self, position : P) {
        let position : Position = position.into();

        match self {
            Mark::Point(p) => { 
                p.set_position(*p.get_position() + position); 
            },
            Mark::Line(l) => {
                for pt in l.get_points_mut() {
                    *pt += position.into();
                }
            },
            Mark::Text(t) => { 
                ()  //TODO
            }
        }
    }

    pub(crate) fn set_mark_index(&mut self, mark_index : usize) -> &mut Self {
        match self {
            Mark::Point(p) => p.common_properties.markid.mark_index = mark_index,
            Mark::Line(l) => l.common_properties.markid.mark_index = mark_index,
            Mark::Text(t) => t.common_properties.markid.mark_index = mark_index
        }
        self
    }

    pub(crate) fn set_layer_index(&mut self, layer_index : usize) -> &mut Self {
         match self {
            Mark::Point(p) => p.common_properties.markid.layer_index = layer_index,
            Mark::Line(l) => l.common_properties.markid.layer_index = layer_index,
            Mark::Text(t) => t.common_properties.markid.layer_index = layer_index
        }
        self
    }

    pub(crate) fn set_valid(&mut self, valid : bool) -> &mut Self {
        match self {
            Mark::Point(p) => p.common_properties.markid.valid = valid,
            Mark::Line(l) => l.common_properties.markid.valid = valid,
            Mark::Text(t) => t.common_properties.markid.valid = valid
        }
        self
    }
}

impl MarkMacro for Mark  {
    fn get_id(&self) -> properties::markid::MarkId {
        mark_get!(self, get_id)
    }

    fn get_size(&self) -> Size {
        mark_get!(self, get_size)
    }

    fn get_color(&self) -> Color {
        mark_get!(self, get_color)
    }

    fn get_rotation(&self) -> f32 {
        mark_get!(self, get_rotation)
    }

    fn get_layer_index(&self) -> usize {
        mark_get!(self, get_layer_index)
    }

    fn set_size<S : Into <properties::size::Size>>(&mut self, size : S) -> &mut Self {
        mark_set!(self, set_size, size)
    }

    fn set_color<C : Into <properties::color::Color>>(&mut self, color : C) -> &mut Self {
        mark_set!(self, set_color, color)
    }

    fn set_rotation(&mut self, rotation : f32) -> &mut Self {
        mark_set!(self, set_rotation, rotation)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::marks::pointmark::Shape;
    use crate::MarkMacro;

    #[test]
    fn as_point_mark() {
        let mark = Mark::Point(PointMark::new());
        assert!(mark.as_point_mark().is_some());
        mark.as_point_mark().unwrap().get_shape();
    }

    #[test]
    #[should_panic]
    fn as_point_mark_panic() {
        let mark = Mark::Line(LineMark::new());
        assert!(mark.as_point_mark().is_none());
        mark.as_point_mark().unwrap().get_shape();
    }

    #[test]
    fn as_point_mark_mut() {
        let mut mark = Mark::Point(PointMark::new());
        assert!(mark.as_point_mark_mut().is_some());
        mark.as_point_mark_mut().unwrap().set_shape(Shape::Triangle);
    }

    #[test]
    #[should_panic]
    fn as_point_mark_mut_panic() {
        let mut mark = Mark::Line(LineMark::new());
        assert!(mark.as_point_mark_mut().is_none());
        mark.as_point_mark_mut().unwrap().set_shape(Shape::Triangle);
    }

    #[test]
    fn as_point_mark_unchecked() {
        let mark = Mark::Point(PointMark::new());
        mark.as_point_mark_unchecked().get_shape();
    }

    #[test]
    #[should_panic]
    fn as_point_mark_unchecked_panic() {
        let mark = Mark::Line(LineMark::new());
        mark.as_point_mark_unchecked().get_shape();
    }

    #[test]
    fn as_point_mark_mut_unchecked() {
        let mut mark = Mark::Point(PointMark::new());
        mark.as_point_mark_mut_unchecked().set_shape(Shape::Triangle);
    }

    #[test]
    #[should_panic]
    fn as_point_mark_mut_unchecked_panic() {
        let mut mark = Mark::Line(LineMark::new());
        mark.as_point_mark_mut_unchecked().set_shape(Shape::Triangle);
    }
}