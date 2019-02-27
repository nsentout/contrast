use crate::MarkMacro;
use crate::marks::pointmark::PointMark;
use crate::marks::pointmark::VertexPoint;
use crate::marks::linemark::LineMark;
use properties::color::Color;
use properties::size::Size;

/// Union of every type of mark.
#[derive(Debug)]
pub enum Mark {
	Point(PointMark),
	Line(LineMark),
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

impl Mark  {
    pub fn get_id(&self) -> usize {
        mark_get!(self, get_id)
    }

    pub fn get_size(&self) -> Size {
        mark_get!(self, get_size)
    }

    pub fn get_color(&self) -> Color {
        mark_get!(self, get_color)
    }

    pub fn get_rotation(&self) -> f32 {
        mark_get!(self, get_rotation)
    }

    pub(self) fn set_id(&mut self, id : usize) -> &mut Self {
        match self {    // cannot use the macro because set_id is not a method of MarkMacro
            Mark::Point(p) => p.common_properties.id = id,
            Mark::Line(l) => l.common_properties.id = id
        }
        self
    }

    pub fn set_size<S : Into <properties::size::Size>>(&mut self, size : S) -> &mut Self {
        mark_set!(self, set_size, size)
    }

    pub fn set_color<C : Into <properties::color::Color>>(&mut self, color : C) -> &mut Self {
        mark_set!(self, set_color, color)
    }

    pub fn set_rotation(&mut self, rotation : f32) -> &mut Self {
        mark_set!(self, set_rotation, rotation)
    }
}

/// This is the main structure of the library. It contains all the marks
/// displayed on screen. The user can add, get, remove and modify marks 
/// as he wishes. The id of each mark represents their index in the vector,
/// which allows for adding and removal of marks in O(1).
pub struct Contrast {
    marks : Vec<Mark>
}

impl Contrast {
    /// Simply returns a new instance of Contrast, initializing
    /// the vector containing all the marks.
    pub fn new() -> Self { 
        Contrast {
            marks : Vec::<Mark>::new()
        }
    }

    /// Create a mark of type "point" with default values and add it into the main
    /// vector, then returns a mutable reference of this newly created mark,
    /// all of this in O(1). We return a mutable reference because we want
    /// to be able to modify it just after calling add_point_mark in a way
    /// similar to this : add_point_mark.set_rotation(90.0).
    pub fn add_point_mark(&mut self) -> &mut PointMark {
        let point = Mark::Point(PointMark::new(self.marks.len()));
        self.marks.push(point);
        
        match self.marks.last_mut().unwrap() {
            Mark::Point(p) => p,
            _ => panic!("A problem occured when adding a new point mark!")
        }
    }

    /// Same behavior than add_point_mark but it adds a mark of type "line".
    pub fn add_line_mark(&mut self) -> &mut LineMark {
        let line = Mark::Line(LineMark::new(self.marks.len()));
        self.marks.push(line);
        
        match self.marks.last_mut().unwrap() {
            Mark::Line(l) => l,
            _ => panic!("A problem occured when adding a new line mark!")
        }
    }

    /// Returns an Option of the mark at the index "id". If there is no mark having this id,
    /// returns None.
    pub fn get_mark(&mut self, id : usize) -> Option<&mut Mark> {
        self.marks.get_mut(id)
    }

    /// Remove the mark with the id mark. We will call this mark the target.
    /// We first set the id of the last element of the vector containing all the marks
    /// to the target's id (mark).
    /// We then swap the target with the last element. We can now safely remove the target.
    /// This way, the mark that was the last element before the removal holds now the id
    /// of the target. This explains why we can always use "self.marks.len()" when we
    /// want to give a unique id to a new mark. Furthermore, this allows us to remove
    /// an element in O(1).
    pub fn remove_mark(&mut self, mark : usize) {
        if !self.marks.is_empty() { self.marks.last_mut().unwrap().set_id(mark); }
        if self.marks.len() > mark { self.marks.swap_remove(mark); }
    }

    /// Convert the MarkPoints contained in the main vector into a vector
    /// of vertices understandable by the renderer, then returns it.
    pub fn get_pointmarks_properties(self) -> Vec<VertexPoint> {    // TODO: éviter cette copie
        let mut properties : Vec<VertexPoint> = Vec::<VertexPoint>::new();
        for pt in &self.marks {
            if let Mark::Point(p) = pt {
                properties.push(p.as_vertex());
            }
        }
        properties
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::marks::pointmark::Shape;
    use crate::MarkMacro;

    #[test]
    fn new()
    {
        assert_eq!(Contrast::new().get_pointmarks_properties().len(), 0);
    }

    #[test]
    fn add_point_mark()
    {
        let mut c = Contrast::new();
        
        let m1 = c.add_point_mark().get_id();
        
        assert_eq!(c.marks.len(), 1);
        assert_eq!(m1, 0);

        let m2 = c.add_point_mark().get_id();
        let m3 = c.add_point_mark().get_id();

        assert_eq!(c.marks.len(), 3);
        assert_eq!(m1, 0);
        assert_eq!(m2, 1);
        assert_eq!(m3, 2);
    }

    #[test]
    fn remove_point_mark()
    {
        let mut c = Contrast::new();

        let m1 = c.add_point_mark().get_id();
        let m2 = c.add_point_mark().get_id();

        assert_eq!(m1, 0);
        assert_eq!(m2, 1);

        c.remove_mark(m1);

        assert_eq!(c.marks.len(), 1);
        assert_eq!(c.marks.get(0).unwrap().get_id(), 0);
    }

    #[test]
    fn get_pointmarks_properties()
    {
        let mut c = Contrast::new();

        c.add_point_mark().set_position((1.0, 5.0, 9.0));
        c.add_point_mark().set_shape(Shape::Rectangle);
        c.add_point_mark().set_position((3.6, 5.0, 9.2)).set_shape(Shape::Triangle)
            .set_size((0.5, 0.3)).set_rotation(90.0).set_color((1.0, 0.0, 0.5, 1.0))
            .set_selection_angle(120.0).set_start_radius(45.0);

        let marks_properties = c.get_pointmarks_properties();

        assert_eq!(marks_properties[0], ([1.0, 5.0, 9.0], [0.0, 0.0], [0.0, 0.0, 0.0, 0.0], 0.0, 0, 0.0, 0.0));
        assert_eq!(marks_properties[1], ([0.0, 0.0, 0.0], [0.0, 0.0], [0.0, 0.0, 0.0, 0.0], 0.0, 1, 0.0, 0.0));
        assert_eq!(marks_properties[2], ([3.6, 5.0, 9.2], [0.5, 0.3], [1.0, 0.0, 0.5, 1.0], 90.0, 2, 120.0, 45.0));
    }

    #[test]
    fn add_line_mark()
    {
        let mut c = Contrast::new();
        
        let m1 = c.add_line_mark().get_id();
        
        assert_eq!(c.marks.len(), 1);
        assert_eq!(m1, 0);

        let m2 = c.add_line_mark().get_id();
        let m3 = c.add_line_mark().get_id();

        assert_eq!(c.marks.len(), 3);
        assert_eq!(m1, 0);
        assert_eq!(m2, 1);
        assert_eq!(m3, 2);
    }

    #[test]
    fn remove_line_mark()
    {
        let mut c = Contrast::new();
        
        let m1 = c.add_line_mark().get_id();
        let m2 = c.add_line_mark().get_id();

        assert_eq!(m1, 0);
        assert_eq!(m2, 1);

        c.remove_mark(m1);

        assert_eq!(c.marks.len(), 1);
        assert_eq!(c.marks.get(0).unwrap().get_id(), 0);
    }

    #[test]
    fn get_id()
    {
        let mut c = Contrast::new();

        c.add_point_mark();
        c.add_point_mark();

        let m0 = c.get_mark(0).unwrap().get_id();
        let m1 = c.get_mark(1).unwrap().get_id();

        assert_eq!(m0, 0); 
        assert_eq!(m1, 1); 
    }

    #[test]
    fn get_and_set_size()
    {
        let mut c = Contrast::new();

        c.add_point_mark();
        c.add_point_mark();

        let m0 = c.get_mark(0).unwrap().get_id();
        let m1 = c.get_mark(1).unwrap().get_id();

        c.get_mark(m0).unwrap().set_size((10.0, 20.0));
        c.get_mark(m1).unwrap().set_size((30.0, 40.0));

        assert_eq!(c.get_mark(m0).unwrap().get_size(), Size { width : 10.0, height : 20.0 });
        assert_eq!(c.get_mark(m1).unwrap().get_size(), Size { width : 30.0, height : 40.0 });
    }

    #[test]
    fn get_and_set_color()
    {
        let mut c = Contrast::new();

        c.add_line_mark();
        c.add_line_mark();

        let m0 = c.get_mark(0).unwrap().get_id();
        let m1 = c.get_mark(1).unwrap().get_id();

        c.get_mark(m0).unwrap().set_color((0.1, 0.2, 0.3, 0.4));
        c.get_mark(m1).unwrap().set_color((0.5, 0.6, 0.7, 0.8));

        assert_eq!(c.get_mark(m0).unwrap().get_color(), Color { r : 0.1, g : 0.2, b : 0.3, a : 0.4 }); 
        assert_eq!(c.get_mark(m1).unwrap().get_color(), Color { r : 0.5, g : 0.6, b : 0.7, a : 0.8 }); 
    }

    #[test]
    fn get_and_set_rotation()
    {
        let mut c = Contrast::new();

        c.add_line_mark();
        c.add_line_mark();

        let m0 = c.get_mark(0).unwrap().get_id();
        let m1 = c.get_mark(1).unwrap().get_id();

        c.get_mark(m0).unwrap().set_rotation(90.0);
        c.get_mark(m1).unwrap().set_rotation(180.0);

        assert_eq!(c.get_mark(m0).unwrap().get_rotation(), 90.0); 
        assert_eq!(c.get_mark(m1).unwrap().get_rotation(), 180.0); 
    }
}