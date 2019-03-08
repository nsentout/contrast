use properties::markid::MarkId;
use crate::marks::mark::Mark;
use crate::marks::pointmark::PointMark;
use crate::marks::pointmark::VertexPoint;
use crate::marks::linemark::VertexLine;
use crate::marks::linemark::LineMark;
use crate::layer::Layer;

/// This is the main structure of the library. It contains all the marks
/// displayed on screen. The user can add, get, remove and modify marks
/// as he wishes. The id of each mark represents their index in the vector,
/// which allows for adding and removal of marks in O(1).
pub struct Contrast {
    layers : Vec<Layer>,
    total_marks : usize
}

impl Contrast {
    /// Simply returns a new instance of Contrast, initializing
    /// the vector containing all the marks.
    pub fn new() -> Self {
        let mut contrast = Contrast {
            layers : Vec::<Layer>::new(),
            total_marks : 0
        };

        contrast.layers.push(Layer::new(0));
        contrast
    }

    /// Create a mark of type "point" with default values and add it into the main
    /// vector, then returns a mutable reference of this newly created mark,
    /// all of this in O(1). We return a mutable reference because we want
    /// to be able to modify it just after calling add_point_mark in a way
    /// similar to this : add_point_mark.set_rotation(90.0).
    pub fn add_point_mark(&mut self) -> &mut PointMark {
        let point = Mark::Point(PointMark::new(self.total_marks));
        self.layers.get_mut(0).unwrap().add_mark(point);
        self.total_marks += 1;

        match self.layers.get_mut(0).unwrap().get_last_mark_mut() {
            Mark::Point(p) => p,
            _ => panic!("A problem occured when adding a new point mark!")
        }
    }

    /// Same behavior than add_point_mark but it adds a mark of type "line".
    pub fn add_line_mark(&mut self) -> &mut LineMark {
        let line = Mark::Line(LineMark::new(self.total_marks));
        self.layers.get_mut(0).unwrap().add_mark(line);
        self.total_marks += 1;

        match self.layers.get_mut(0).unwrap().get_last_mark_mut() {
            Mark::Line(p) => p,
            _ => panic!("A problem occured when adding a new line mark!")
        }
    }

    /// Returns a reference wrapped into an Option of the mark at the index "id". 
    /// If there is no mark having this id, returns None.
    pub fn get_mark(&mut self, markid : &MarkId) -> Option<&Mark> {
        self.layers.get(markid.layer).unwrap().get_mark(markid)
    }

    /// Returns a mutable reference wrapped into an Option of the mark at the index "id". 
    /// If there is no mark having this id, returns None.
    pub fn get_mark_mut(&mut self, markid : &MarkId) -> Option<&mut Mark> {
        self.layers.get_mut(markid.layer).unwrap().get_mark_mut(&*markid)
    }

    /// Remove the mark with the id mark. We will call this mark the target.
    /// We first set the id of the last element of the vector containing all the marks
    /// to the target's id (mark).
    /// We then swap the target with the last element. We can now safely remove the target.
    /// This way, the mark that was the last element before the removal holds now the id
    /// of the target. This explains why we can always use "self.marks.len()" when we
    /// want to give a unique id to a new mark. Furthermore, this allows us to remove
    /// an element in O(1).
    pub fn remove_mark(&mut self, markid : &MarkId) {
        let layer = self.layers.get_mut(markid.layer).unwrap();

        if !layer.has_no_mark() {
            layer.get_last_mark_mut().set_id(*markid);
        }

        if layer.get_marks_nb() > markid.id { 
            layer.swap_remove_mark(&*markid);
            self.total_marks -= 1;
        }
    }

    pub(crate) fn remove_and_get_mark(&mut self, markid : &MarkId) -> Option<Mark> {
        let layer : &mut Layer = self.layers.get_mut(markid.layer).unwrap();

        if !layer.has_no_mark() { 
            layer.get_last_mark_mut().set_id(*markid);
        }

        if layer.get_marks_nb() > markid.id { 
            Some(layer.swap_remove_mark(markid))
        }
        else {
            None
        }
    }

    /// Add a new layer into contrast.
    pub fn add_layer(&mut self) {
        self.layers.push(Layer::new(self.layers.len()));
    }

    /// Assign a new layer to a mark.
    /// DOES NOT WORK
    pub fn set_mark_layer(&mut self, markid : &MarkId, layer : usize) {
        let mark = self.remove_and_get_mark(markid).unwrap();
        self.layers.get_mut(layer).unwrap().add_mark(mark);
    }

    /// Returns a reference wrapped into an Option of the Layer 
    /// at the index <layer>.
    pub fn get_layer(&self, layer : usize) -> Option<&Layer> {
        self.layers.get(layer)
    }

    /// Returns a mutable reference wrapped into an Option of the Layer 
    /// at the index <layer>.
    pub fn get_layer_mut(&mut self, layer : usize) -> Option<&mut Layer> {
        self.layers.get_mut(layer)
    }

    /// Returns the number of marks in total.
    pub fn get_marks_nb(&self) -> usize {
        self.total_marks
    }

    /// Convert the MarkPoints contained in the main vector into a vector
    /// of vertices understandable by the renderer, then returns it.
    pub fn get_pointmarks_properties(self) -> Vec<VertexPoint> {
        let mut properties : Vec<VertexPoint> = Vec::<VertexPoint>::new();
        for layer in &self.layers {
            for mark in layer.get_all_marks() {
                if let Mark::Point(p) = mark {
                    properties.push(p.as_vertex());
                }
            }
        }
        properties
    }

	/// Convert the LineMarks contained in the main vector into a vector
    /// of vertices understandable by the renderer, then returns it.
    pub fn get_linemarks_properties(self) -> Vec<VertexLine> {
        let mut properties : Vec<VertexLine> = Vec::<VertexLine>::new();
        for layer in &self.layers {
            for mark in layer.get_all_marks() {
                if let Mark::Line(l) = mark {
                    properties.append(&mut l.as_vertex());
                }
            }
        }
        properties
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use properties::color::Color;
    use properties::size::Size;
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

        let m1 = c.add_point_mark().set_rotation(45.0).get_id();
        let m2 = c.add_point_mark().set_rotation(90.0).get_id();

        assert_eq!(m1.id, 0);
        assert_eq!(m2.id, 1);
        assert_eq!(m1.layer, 0);
        assert_eq!(m2.layer, 0);

        c.remove_mark(&m1);
        let m2 = &c.layers.get_mut(0).unwrap().get_last_mark_mut().get_id();

        assert_eq!(c.get_marks_nb(), 1);
        assert_eq!(m2.id, 0);
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

        let m0 = c.get_mark_mut(0).unwrap().get_id();
        let m1 = c.get_mark_mut(1).unwrap().get_id();

        assert_eq!(m0, 0);
        assert_eq!(m1, 1);
    }

    #[test]
    fn get_and_set_size()
    {
        let mut c = Contrast::new();

        c.add_point_mark();
        c.add_point_mark();

        let m0 = c.get_mark_mut(0).unwrap().get_id();
        let m1 = c.get_mark_mut(1).unwrap().get_id();

        c.get_mark_mut(m0).unwrap().set_size((10.0, 20.0));
        c.get_mark_mut(m1).unwrap().set_size((30.0, 40.0));

        assert_eq!(c.get_mark_mut(m0).unwrap().get_size(), Size { width : 10.0, height : 20.0 });
        assert_eq!(c.get_mark_mut(m1).unwrap().get_size(), Size { width : 30.0, height : 40.0 });
    }

    #[test]
    fn get_and_set_color()
    {
        let mut c = Contrast::new();

        c.add_line_mark();
        c.add_line_mark();

        let m0 = c.get_mark_mut(0).unwrap().get_id();
        let m1 = c.get_mark_mut(1).unwrap().get_id();

        c.get_mark_mut(m0).unwrap().set_color((0.1, 0.2, 0.3, 0.4));
        c.get_mark_mut(m1).unwrap().set_color((0.5, 0.6, 0.7, 0.8));

        assert_eq!(c.get_mark_mut(m0).unwrap().get_color(), Color { r : 0.1, g : 0.2, b : 0.3, a : 0.4 });
        assert_eq!(c.get_mark_mut(m1).unwrap().get_color(), Color { r : 0.5, g : 0.6, b : 0.7, a : 0.8 });
    }

    #[test]
    fn get_and_set_rotation()
    {
        let mut c = Contrast::new();

        c.add_line_mark();
        c.add_line_mark();

        let m0 = c.get_mark_mut(0).unwrap().get_id();
        let m1 = c.get_mark_mut(1).unwrap().get_id();

        c.get_mark_mut(m0).unwrap().set_rotation(90.0);
        c.get_mark_mut(m1).unwrap().set_rotation(180.0);

        assert_eq!(c.get_mark_mut(m0).unwrap().get_rotation(), 90.0);
        assert_eq!(c.get_mark_mut(m1).unwrap().get_rotation(), 180.0);
    }
}
