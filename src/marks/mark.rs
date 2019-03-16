use crate::MarkMacro;
use properties::markid::MarkId;
use properties::color::Color;
use properties::size::Size;
use properties::position::Position;
use crate::marks::pointmark::PointMark;
use crate::marks::linemark::LineMark;
use self::MarkTy::*;
use std::slice::Iter;


/// Union of every type of mark.
#[derive(Clone, Debug)]
pub enum Mark {
	Point(PointMark),
	Line(LineMark),
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
            Mark::Line(l)  => l.$get()
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
                Mark::Line(l)  => { l.$set($param); }
            }
            $mark
        }
    }
}

impl Mark {
    /// Move the mark according to the <position>. Used by Layer to move
    /// all his marks.
    /// Example : if <position> is (50.0, 0.0, 0.0), every point of the mark 
    /// will move 50 pixels to the right.
    pub(crate) fn move_of<P : Into <Position>>(&mut self, position : P) {
        let position : Position = position.into();

        match self {
            Mark::Point(p) => { 
                p.set_position(*p.get_position() + position); 
            },
            Mark::Line(l) => {
                for pt in l.get_points_mut() {
                    *pt += position.into();
                }
            }
        }
    }

    pub(crate) fn set_mark_index(&mut self, mark_index : usize) -> &mut Self {
        match self {
            Mark::Point(p) => p.common_properties.markid.mark_index = mark_index,
            Mark::Line(l) => l.common_properties.markid.mark_index = mark_index
        }
        self
    }

    pub(crate) fn set_valid(&mut self, valid : bool) -> &mut Self {
        match self {
            Mark::Point(p) => p.common_properties.markid.valid = valid,
            Mark::Line(l) => l.common_properties.markid.valid = valid
        }
        self
    }

    pub(crate) fn set_id(&mut self, markid : MarkId) -> &mut Self {
        match self {
            Mark::Point(p) => p.common_properties.markid.mark_index = markid.mark_index,
            Mark::Line(l) => l.common_properties.markid.mark_index = markid.mark_index
        }
        self
    }

    pub(crate) fn is_valid(&self) -> bool {
        match self {
            Mark::Point(p) => p.common_properties.markid.valid,
            Mark::Line(l) => l.common_properties.markid.valid
        }
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

    fn set_layer_index(&mut self, layer_index : usize) -> &mut Self {
        mark_set!(self, set_layer_index, layer_index)
    }
}