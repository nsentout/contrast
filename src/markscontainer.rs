use properties::markid::MarkId;
use crate::MarkMacro;
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
    total_marks : usize,
    valid_marks : usize
}

impl Contrast {
    /// Simply returns a new instance of Contrast, initializing
    /// the vector containing all the marks.
    pub fn new() -> Self {
        let mut contrast = Contrast {
            layers : Vec::<Layer>::new(),
            total_marks : 0,
            valid_marks : 0
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
        self.layers.get(markid.layer_index).unwrap().get_mark(markid)
    }

    /// Returns a mutable reference wrapped into an Option of the mark at the index "id". 
    /// If there is no mark having this id, returns None.
    pub fn get_mark_mut(&mut self, markid : &MarkId) -> Option<&mut Mark> {
        self.layers.get_mut(markid.layer_index).unwrap().get_mark_mut(markid)
    }

    /// Remove the mark with the id mark. We will call this mark the target.
    /// We first set the id of the last element of the vector containing all the marks
    /// to the target's id (mark).
    /// We then swap the target with the last element. We can now safely remove the target.
    /// This way, the mark that was the last element before the removal holds now the id
    /// of the target. This explains why we can always use "self.marks.len()" when we
    /// want to give a unique id to a new mark. Furthermore, this allows us to remove
    /// an element in O(1).
    pub fn remove_mark(&mut self, markid : &mut MarkId) {
        self.layers.get_mut(markid.layer_index).unwrap().invalidate_mark(markid);
    }

    /// Add a new layer into contrast.  //TODO: add layers automatically
    pub fn add_layer(&mut self) {
        self.layers.push(Layer::new(self.layers.len()));
    }

    /// Assign a new layer to a mark.   //TODO: move it into Mark
    pub fn set_mark_layer(&mut self, markid : &mut MarkId, layer_index : usize) {

        // If already in the layer, returns
        if layer_index == markid.layer_index { return; }

        let current_layer_size = self.layers.get(markid.layer_index).unwrap().get_marks_nb();
        let wanted_layer_size = self.layers.get(layer_index).unwrap().get_marks_nb();

        // Retrieve a copy of the mark in his current layer
        let mut mark = self.layers.get_mut(markid.layer_index).unwrap().invalidate_and_get_mark(markid);

        // Update the mark according to his new layer
        mark.set_mark_index(wanted_layer_size); 
        mark.set_layer_index(layer_index);
        mark.set_valid(true);

        // Update the markid passed as parameter so it stays coherent
        markid.mark_index = wanted_layer_size;
        markid.layer_index = layer_index;
        markid.valid = true;
            
        // Add the mark of the wanted layer
        self.layers.get_mut(layer_index).unwrap().add_mark(mark); 
    }

    /// Returns a reference wrapped into an Option of the Layer 
    /// at the index <layer>.
    /// should it be public ?
    pub fn get_layer(&self, layer_index : usize) -> Option<&Layer> {
        self.layers.get(layer_index)
    }

    /// Returns a mutable reference wrapped into an Option of the Layer 
    /// at the index <layer>.
    pub fn get_layer_mut(&mut self, layer_index : usize) -> Option<&mut Layer> {
        self.layers.get_mut(layer_index)
    }

    /// Returns the number of marks in total.
    pub fn get_marks_nb(&self) -> usize {
        self.total_marks
    }

    /// Convert the MarkPoints contained in the main vector into a vector
    /// of vertices understandable by the renderer, then returns it.
    pub fn get_pointmarks_properties(&self) -> Vec<VertexPoint> {
        let mut properties : Vec<VertexPoint> = Vec::<VertexPoint>::new();
        for layer in &self.layers {
            for mark in layer.get_all_marks() {
                if let Mark::Point(p) = mark {
                    if mark.is_valid() {
                        properties.push(p.as_vertex());
                    }
                }
            }
        }
        properties
    }

	/// Convert the LineMarks contained in the main vector into a vector
    /// of vertices understandable by the renderer, then returns it.
    pub fn get_linemarks_properties(&self) -> Vec<VertexLine> {
        let mut properties : Vec<VertexLine> = Vec::<VertexLine>::new();
        for layer in &self.layers {
            for mark in layer.get_all_marks() {
                if let Mark::Line(l) = mark {
                    if mark.is_valid() {
                        properties.append(&mut l.as_vertex());
                    }
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

        assert_eq!(c.get_marks_nb(), 1);

        let m2 = c.add_point_mark().get_id();
        let m3 = c.add_point_mark().get_id();

        assert_eq!(c.get_marks_nb(), 3);
    }

    #[test]
    fn remove_point_mark()
    {
        let mut c = Contrast::new();

        let mut m1 = c.add_point_mark().get_id();
        let mut m2 = c.add_point_mark().get_id();

        c.remove_mark(&mut m1);

        assert_eq!(c.get_marks_nb(), 1);
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

        assert_eq!(c.get_marks_nb(), 1);

        let m2 = c.add_line_mark().get_id();
        let m3 = c.add_line_mark().get_id();

        assert_eq!(c.get_marks_nb(), 3);
    }

    #[test]
    fn remove_line_mark()
    {
        let mut c = Contrast::new();

        let mut m1 = c.add_line_mark().get_id();
        let mut m2 = c.add_line_mark().get_id();

        c.remove_mark(&mut m1);

        assert_eq!(c.get_marks_nb(), 1);
    }

    #[test]
    fn set_mark_layer()
    {
        let mut c = Contrast::new();
        c.add_layer();
        c.add_layer();

        let mut m1 = c.add_point_mark().set_position((100.0, 150.0, 0.0)).get_id();
        let mut m2 = c.add_point_mark().set_position((200.0, 250.0, 1.0)).get_id();
        let mut m3 = c.add_point_mark().set_position((300.0, 350.0, 2.0)).get_id();

        for i in 0..3 {
            for j in 0..3 {
                for k in 0..3 {
                    c.set_mark_layer(&mut m1, i);
                    c.set_mark_layer(&mut m2, j);
                    c.set_mark_layer(&mut m3, k);

                    let marks_properties = c.get_pointmarks_properties();

                    if (i != j && j != k && i != k) {
                        assert_eq!(marks_properties[i], ([100.0, 150.0, 0.0], [0.0, 0.0], [0.0, 0.0, 0.0, 0.0], 0.0, 0, 0.0, 0.0));
                        assert_eq!(marks_properties[j], ([200.0, 250.0, 1.0], [0.0, 0.0], [0.0, 0.0, 0.0, 0.0], 0.0, 0, 0.0, 0.0));
                        assert_eq!(marks_properties[k], ([300.0, 350.0, 2.0], [0.0, 0.0], [0.0, 0.0, 0.0, 0.0], 0.0, 0, 0.0, 0.0));
                    }
                }
            }
        }
    }

    #[test]
    fn get_id()
    {
        let mut c = Contrast::new();

        let m1 = c.add_line_mark().get_id();
        let m2 = c.add_line_mark().get_id();

        let expected_m1_id = MarkId { mark_index : 0, layer_index : 0, valid : true };
        let expected_m2_id = MarkId { mark_index : 1, layer_index : 0, valid : true };

        assert_eq!(m1, expected_m1_id);
        assert_eq!(m2, expected_m2_id);
    }

    #[test]
    fn get_and_set_size()
    {
        let mut c = Contrast::new();

        let m1 = c.add_line_mark().set_size((10.0, 20.0)).get_id();
        let m2 = c.add_line_mark().set_size((30.0, 40.0)).get_id();

        assert_eq!(c.get_mark_mut(&m1).unwrap().get_size(), Size { width : 10.0, height : 20.0 });
        assert_eq!(c.get_mark_mut(&m2).unwrap().get_size(), Size { width : 30.0, height : 40.0 });
    }

    #[test]
    fn get_and_set_color()
    {
        let mut c = Contrast::new();

        let m1 = c.add_line_mark().set_color((0.1, 0.2, 0.3, 0.4)).get_id();
        let m2 = c.add_line_mark().set_color((0.5, 0.6, 0.7, 0.8)).get_id();

        assert_eq!(c.get_mark_mut(&m1).unwrap().get_color(), Color { r : 0.1, g : 0.2, b : 0.3, a : 0.4 });
        assert_eq!(c.get_mark_mut(&m2).unwrap().get_color(), Color { r : 0.5, g : 0.6, b : 0.7, a : 0.8 });
    }

    #[test]
    fn get_and_set_rotation()
    {
        let mut c = Contrast::new();

        let m1 = c.add_line_mark().set_rotation(90.0).get_id();
        let m2 = c.add_line_mark().set_rotation(180.0).get_id();

        assert_eq!(c.get_mark_mut(&m1).unwrap().get_rotation(), 90.0);
        assert_eq!(c.get_mark_mut(&m2).unwrap().get_rotation(), 180.0);
    }
}
