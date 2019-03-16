use std::collections::HashSet;
use properties::markid::MarkId;
use crate::MarkMacro;
use crate::marks::mark::Mark;
use crate::marks::mark::MarkTy;
use crate::marks::pointmark::PointMark;
use crate::marks::pointmark::VertexPoint;
use crate::marks::linemark::SubLine;
use crate::marks::linemark::LineMark;
use crate::layer::Layer;


/// This is the main structure of the library. It contains all the layers
/// added by the user, as well as the current layer.
/// The current layer is the layer to which contrast will add marks by
/// default. It is by default the layer 0, on first plan.
/// The user can add, get, remove and modify marks as he wishes, as well
/// as get the layers to apply some functions on its marks.
pub struct Contrast {
    pub(crate) layers : Vec<Layer>,
    pub(crate) current_layer_index : usize,
    pub(crate) update: HashSet<MarkTy>
}

impl Contrast {
    /// Simply returns a new instance of Contrast, initializing
    /// the vector containing all the layers.
    pub fn new() -> Self {
        Contrast {
            layers : Vec::<Layer>::new(),
            current_layer_index : 0,
            update: HashSet::new()
        }
    }

    /// Initialize contrast. At the moment, all this does is add a first layer to Contrast.
    pub fn init(&mut self) {
        let layer_0 = Layer::new(0, self);
        self.layers.push(layer_0);
    }

    /// Append a mark container dirty.
    pub fn mark_dirty(&mut self, id: MarkId)
    {
        match self.get_mark(&id)
        {
            Some(m) => {
                match m
                {
                    Mark::Point(_) => self.update.insert(MarkTy::Point),
                    Mark::Line(_) => self.update.insert(MarkTy::Point)
                }
            },
            None => panic!("Invalid MarkId")
        };
    }

    /// Fetch Updated mark and reset.
    pub fn fetch_update(&mut self) -> HashSet<MarkTy>
    {
        let update = self.update.clone();
        self.update.clear();
        update
    }

    /// Create a mark of type "point" with default values and add it into current
    /// layer, then returns a mutable reference of this newly created mark,
    /// all of this in O(1). We return a mutable reference because we want
    /// to be able to modify it just after calling add_point_mark in a way
    /// similar to this : add_point_mark.set_rotation(90.0).
    pub fn add_point_mark(&mut self) -> &mut PointMark {
        let point = Mark::Point(PointMark::new());
        self.layers.get_mut(self.current_layer_index).unwrap().force_add_mark(point);

        match self.layers.get_mut(self.current_layer_index).unwrap().get_last_mark_mut() {
            Mark::Point(p) => p,
            _ => panic!("A problem occured when adding a new point mark!")
        }
    }

    /// Same behavior than add_point_mark but it adds a mark of type "line".
    pub fn add_line_mark(&mut self) -> &mut LineMark {
        let line = Mark::Line(LineMark::new());
        self.layers.get_mut(self.current_layer_index).unwrap().force_add_mark(line);

        match self.layers.get_mut(self.current_layer_index).unwrap().get_last_mark_mut() {
            Mark::Line(l) => l,
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
        if markid.valid {
            return self.layers.get_mut(markid.layer_index).unwrap().get_mark_mut(markid);
        }
        None
    }

    /// Remove the mark with the id mark. This function asks the layer to invalidate the mark,
    /// implying this mark won't be displayed.
    pub fn remove_mark(&mut self, markid : &mut MarkId) {
        self.layers.get_mut(markid.layer_index).unwrap().invalidate_mark(markid);
    }

    /// Set the current layer. The current layer is the layer where contrast will push
    /// all marks by default.
    pub fn set_current_layer(&mut self, layer_index : usize) {
        // Add layers if necessary
        if layer_index > self.layers.len() {
            self.add_layers(layer_index - self.layers.len());
        }

        self.current_layer_index = layer_index;
    }

    /// Add new layers into contrast.
    pub fn add_layers(&mut self, nb : usize) {
        for _ in 0..nb {
            let new_layer = Layer::new(self.layers.len(), self);
            self.layers.push(new_layer);
        }
    }

    /// Returns a reference wrapped into an Option of the Layer
    /// at the index <layer_index>.
    pub fn get_layer(&self, layer_index : usize) -> Option<&Layer> {
        self.layers.get(layer_index)
    }

    /// Returns a mutable reference wrapped into an Option of the Layer
    /// at the index <layer_index>.
    pub fn get_layer_mut(&mut self, layer_index : usize) -> Option<&mut Layer> {
        self.layers.get_mut(layer_index)
    }

    /// Convert the MarkPoints contained in the main vector into a vector
    /// of vertices understandable by the renderer, then returns it.
    pub fn get_pointmarks_properties(&mut self) -> Vec<VertexPoint> {
        //self.layers.sort();
        let mut properties : Vec<VertexPoint> = Vec::<VertexPoint>::new();
        for layer in &self.layers {
            //println!("{}", layer.depth);
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
    /// of sub-line understandable by the renderer, then returns it.
    pub fn get_linemarks_properties(self) -> Vec<SubLine> {
        let mut properties : Vec<SubLine> = Vec::<SubLine>::new();
        for layer in &self.layers {
            for mark in layer.get_all_marks() {
                if let Mark::Line(l) = mark {
                    properties.append(&mut l.to_subline());
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
        c.init();

        let m1 = c.add_point_mark().get_id();

        assert_eq!(c.get_pointmarks_properties().len(), 1);

        let m2 = c.add_point_mark().get_id();
        let m3 = c.add_point_mark().get_id();

        assert_eq!(c.get_pointmarks_properties().len(), 3);
    }

    #[test]
    fn remove_point_mark()
    {
        let mut c = Contrast::new();
        c.init();

        let mut m1 = c.add_point_mark().get_id();
        let mut m2 = c.add_point_mark().get_id();

        c.remove_mark(&mut m1);

        assert_eq!(c.get_pointmarks_properties().len(), 1);
    }

    #[test]
    fn get_pointmarks_properties()
    {
        let mut c = Contrast::new();
        c.init();

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
        c.init();

        let m1 = c.add_line_mark().get_id();

        assert_eq!(c.get_linemarks_properties().len(), 1);

        let m2 = c.add_line_mark().get_id();
        let m3 = c.add_line_mark().get_id();

        assert_eq!(c.get_linemarks_properties().len(), 3);
    }

    #[test]
    fn remove_line_mark()
    {
        let mut c = Contrast::new();
        c.init();

        let mut m1 = c.add_line_mark().get_id();
        let mut m2 = c.add_line_mark().get_id();

        c.remove_mark(&mut m1);

        assert_eq!(c.get_linemarks_properties().len(), 1);
    }

    #[test]
    fn add_mark_into_layer()
    {
        let mut c = Contrast::new();
        c.init();
        c.add_layers(2);

        let mut m1 = c.add_point_mark().set_position((100.0, 150.0, 0.0)).get_id();
        let mut m2 = c.add_point_mark().set_position((200.0, 250.0, 1.0)).get_id();
        let mut m3 = c.add_point_mark().set_position((300.0, 350.0, 2.0)).get_id();

        for i in 0..3 {
            for j in 0..3 {
                for k in 0..3 {
                    c.get_layer_mut(i).unwrap().add_mark(&mut m1);
                    c.get_layer_mut(j).unwrap().add_mark(&mut m2);
                    c.get_layer_mut(k).unwrap().add_mark(&mut m3);

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
        c.init();

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
        c.init();

        let m1 = c.add_line_mark().set_size((10.0, 20.0)).get_id();
        let m2 = c.add_line_mark().set_size((30.0, 40.0)).get_id();

        assert_eq!(c.get_mark_mut(&m1).unwrap().get_size(), Size { width : 10.0, height : 20.0 });
        assert_eq!(c.get_mark_mut(&m2).unwrap().get_size(), Size { width : 30.0, height : 40.0 });
    }

    #[test]
    fn get_and_set_color()
    {
        let mut c = Contrast::new();
        c.init();

        let m1 = c.add_line_mark().set_color((0.1, 0.2, 0.3, 0.4)).get_id();
        let m2 = c.add_line_mark().set_color((0.5, 0.6, 0.7, 0.8)).get_id();

        assert_eq!(c.get_mark_mut(&m1).unwrap().get_color(), Color { r : 0.1, g : 0.2, b : 0.3, a : 0.4 });
        assert_eq!(c.get_mark_mut(&m2).unwrap().get_color(), Color { r : 0.5, g : 0.6, b : 0.7, a : 0.8 });
    }

    #[test]
    fn get_and_set_rotation()
    {
        let mut c = Contrast::new();
        c.init();

        let m1 = c.add_line_mark().set_rotation(90.0).get_id();
        let m2 = c.add_line_mark().set_rotation(180.0).get_id();

        assert_eq!(c.get_mark_mut(&m1).unwrap().get_rotation(), 90.0);
        assert_eq!(c.get_mark_mut(&m2).unwrap().get_rotation(), 180.0);
    }
}
