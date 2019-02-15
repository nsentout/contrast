use crate::pointmark::PointMark;
use crate::pointmark::VertexPoint;
use crate::linemark::LineMark;

/// This is the main structure of the library. It contains all the marks
/// displayed on screen. The user can add and remove marks as he wishes
/// but he can't modify the marks (at the moment).
pub struct Contrast {
    point_marks: Vec<PointMark>,
    line_marks: Vec<LineMark>
}

impl Contrast {
    /// Simply returns a new instance of Contrast, initializing
    /// the vectors containing the marks.
    pub fn new() -> Self { 
        Contrast {
            point_marks : Vec::<PointMark>::new(),
            line_marks : Vec::<LineMark>::new()
        }
    }

    /// Create a mark of type Point with default values and add it into the vector
    /// containing all the PointMark, then returns a mutable reference of this
    /// newly created mark, all of this in O(1). We return a mutable reference because we
    /// want to be able to modify it just after calling add_point_mark in a way
    /// similar to this : add_point_mark.set_rotation(90.0).
    pub fn add_point_mark(&mut self) -> &mut PointMark {
        let point = PointMark::new(self.point_marks.len());
        self.point_marks.push(point);
        self.point_marks.last_mut().unwrap()
    }

    /// Remove the point mark with the id mark. We will call this mark the target.
    /// We first set the id of the last element of the vector containing all
    /// the PointMark to the target's id (mark).
    /// We then swap the target with the last element. We can now safely remove the target.
    /// This way, the mark that was the last element before the removal holds now the id
    /// of the target. This explains why we can always use "self.point_marks.len()" when
    /// we want to give a unique id to a new mark. Furthermore, this allows us to remove
    /// an element in O(1).
    pub fn remove_point_mark(&mut self, mark: usize)
    {
        if !self.point_marks.is_empty() { self.point_marks.last_mut().unwrap().common_properties.id = mark; }
        if self.point_marks.len() > mark { self.point_marks.swap_remove(mark); }
    }


    /// Convert the vector of MarkPoint to a vector of vertices
    /// understandable by the renderer, then returns it.
    pub fn get_pointmarks_properties(self) -> Vec<VertexPoint> {    // TODO: éviter cette copie
        let mut properties : Vec<VertexPoint> = Vec::<VertexPoint>::new();
        for pt in &self.point_marks {
            properties.push(pt.as_vertex());
        }
        properties
    }

    /// Same behavior than add_point_mark but it adds a mark of type Line.
    pub fn add_line_mark(&mut self) -> &mut LineMark {
        let line = LineMark::new(self.line_marks.len());
        self.line_marks.push(line);
        self.line_marks.last_mut().unwrap()
    }

    /// Same behavior than remove_point_mark but it removes a mark of type Line.
    pub fn remove_line_mark(&mut self, mark: usize)
    {
        if !self.line_marks.is_empty() { self.line_marks.last_mut().unwrap().common_properties.id = mark; }
        if self.line_marks.len() > mark { self.line_marks.swap_remove(mark); }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pointmark::Shape;
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
        
        assert_eq!(c.point_marks.len(), 1);
        assert_eq!(m1, 0);

        let m2 = c.add_point_mark().get_id();
        let m3 = c.add_point_mark().get_id();

        assert_eq!(c.point_marks.len(), 3);
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

        c.remove_point_mark(m1);

        assert_eq!(c.point_marks.len(), 1);
        assert_eq!(c.point_marks.get(0).unwrap().get_id(), 0);
    }

    #[test]
    fn get_pointmarks_properties()
    {
        let mut c = Contrast::new();

        c.add_point_mark().set_position(1.0, 5.0, 9.0);
        c.add_point_mark().set_shape(Shape::Rectangle);
        c.add_point_mark().set_position(3.6, 5.0, 9.2).set_shape(Shape::Triangle)
            .set_size(0.5, 0.3).set_rotation(90.0).set_color(1.0, 0.0, 0.5, 1.0)
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
        
        assert_eq!(c.line_marks.len(), 1);
        assert_eq!(m1, 0);

        let m2 = c.add_line_mark().get_id();
        let m3 = c.add_line_mark().get_id();

        assert_eq!(c.line_marks.len(), 3);
        assert_eq!(m1, 0);
        assert_eq!(m2, 1);
        assert_eq!(m3, 2);
    }

    #[test]
    fn remove_line_mark()
    {
        let mut c = Contrast::new();
        
        let m1 = { c.add_line_mark().get_id() };
        let m2 = { c.add_line_mark().get_id() };

        assert_eq!(m1, 0);
        assert_eq!(m2, 1);

        c.remove_line_mark(m1);

        assert_eq!(c.line_marks.len(), 1);
        assert_eq!(c.line_marks.get(0).unwrap().get_id(), 0);
    }
}