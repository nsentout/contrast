use std::collections::LinkedList;
use std::collections::HashSet;
use contrast_properties::markid::MarkId;
use crate::marks::mark::Mark;
use crate::marks::mark::MarkTy;
use crate::marks::pointmark::PointMark;
use crate::marks::pointmark::VertexPoint;
use crate::marks::linemark::VertexSubLine;
use crate::marks::linemark::LineMark;
use crate::marks::textmark::TextMark;
use crate::marks::textmark::FontCache;
use crate::marks::textmark::VertexText;
use crate::marks::textmark::TextMarkCmd;
use crate::marks::textmark::Glyph;
use crate::layer::Layer;
use crate::MarkMacro;


/// This is the main structure of the library. It contains all the layers
/// added by the user, as well as the current layer.
/// The current layer is the layer to which contrast will add marks by
/// default. It is by default the layer 0, on first plan.
/// The user can add, get, remove and modify marks as he wishes, as well
/// as to retrieve the layers to apply some functions on its marks.
pub struct Contrast {
    pub(crate) layers : Vec<Layer>,
    pub(crate) current_layer_index : usize,
    pub(crate) update: HashSet<MarkTy>,
    pub(crate) fonts: FontCache
}

impl Contrast {
    /// Simply returns a new instance of Contrast, initializing
    /// the vector containing all the layers.
    pub fn new() -> Self {
        Contrast {
            layers : Vec::<Layer>::new(),
            current_layer_index : 0,
            update: HashSet::new(),
            fonts: FontCache::new()
        }
    }

    /// Initialize contrast. All this does is add a first layer to Contrast.
    pub fn init(&mut self) {
        let layer_0 = Layer::new(0, self);
        self.layers.push(layer_0);
    }

    /// Register font & police with one key name.
    pub fn register_font(&mut self, name: &str, font: &str, police: u32)
    {
        self.fonts.create_face(name, font, police);
    }

    pub fn contains_font(&self, name: &str) -> bool
    {
        self.fonts.contains(name)
    }

    /// Append one dirty mark container.
    pub fn mark_dirty(&mut self, id: MarkId)
    {
        match self.get_mark(&id)
        {
            Some(m) => {
                match m
                {
                    Mark::Point(_) => self.update.insert(MarkTy::Point),
                    Mark::Line(_) => self.update.insert(MarkTy::Line),
                    Mark::Text(_) => self.update.insert(MarkTy::Text)
                }
            },
            None => panic!("Invalid MarkId")
        };
    }

    // Append all dirty mark containers.
    pub fn mark_dirty_all(&mut self)
    {
        for mark in MarkTy::values() { self.update.insert(mark.clone()); }
    }

    /// Fetch updated mark and reset.
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

    /// Same behavior than add_point_mark but it adds a mark of type "Line".
    pub fn add_line_mark(&mut self) -> &mut LineMark {
        let line = Mark::Line(LineMark::new());
        self.layers.get_mut(self.current_layer_index).unwrap().force_add_mark(line);

        match self.layers.get_mut(self.current_layer_index).unwrap().get_last_mark_mut() {
            Mark::Line(l) => l,
            _ => panic!("A problem occured when adding a new line mark!")
        }
    }

    /// Same behavior than add_point_mark but it adds a mark of type "Text".
    pub fn add_text_mark(&mut self) -> &mut TextMark
    {
        let text = Mark::Text(TextMark::new());
        self.layers.get_mut(self.current_layer_index).unwrap().force_add_mark(text);

        match self.layers.get_mut(self.current_layer_index).unwrap().get_last_mark_mut() {
            Mark::Text(t) => t,
            _ => panic!("A problem occured when adding a new text mark!")
        }
    }

    /// Returns a reference wrapped into an Option of the mark represented by 'markid'.
    /// If there is no mark having this id, or if this mark is invalid, returns None.
    pub fn get_mark(&mut self, markid : &MarkId) -> Option<&Mark> {
        if markid.valid {
            return self.layers.get(markid.layer_index).unwrap().get_mark(markid);
        }
        None
    }

    /// Returns a mutable reference wrapped into an Option of the mark represented by 'markid'.
    /// If there is no mark having this id, or if this mark is invalid, returns None.
    pub fn get_mark_mut(&mut self, markid : &MarkId) -> Option<&mut Mark> {
        if markid.valid {
            return self.layers.get_mut(markid.layer_index).unwrap().get_mark_mut(markid);
        }
        None
    }

    /// Remove the mark with the id mark. This does not actually removes the mark from the container
    /// but it asks the layer to invalidate the mark, implying this mark won't be displayed and the
    /// user won't be allowed to retrieve it.
    pub fn remove_mark(&mut self, markid : &mut MarkId) {
        self.layers.get_mut(markid.layer_index).unwrap().invalidate_mark(markid);
    }

    /// Set the current layer. The current layer is the layer where contrast will push
    /// all marks by default.
    pub fn set_current_layer(&mut self, layer_index : usize) {
       // Add layers if necessary
        if layer_index >= self.layers.len() {
            self.add_layers(layer_index + 1 - self.layers.len());
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
    /// at the index 'layer_index'.
    pub fn get_layer(&self, layer_index : usize) -> Option<&Layer> {
        self.layers.get(layer_index)
    }

    /// Returns a mutable reference wrapped into an Option of the Layer
    /// at the index 'layer_index'.
    pub fn get_layer_mut(&mut self, layer_index : usize) -> Option<&mut Layer> {
        self.layers.get_mut(layer_index)
    }

    /// Convert the MarkPoints contained in the main vector into a vector
    /// of vertices understandable by the renderer, then returns it.
    pub fn get_pointmarks_properties(&mut self) -> Vec<VertexPoint> {
        self.layers.sort();
        let mut properties : Vec<VertexPoint> = Vec::<VertexPoint>::new();
        for layer in &mut self.layers {
            for mark in &mut layer.marks {
                if let Mark::Point(ref mut p) = mark {
                    let display = p.is_displayed();
                    p.set_displayed(true);
                    if p.is_valid() {
                        if !display {   // if first time the mark is displayed
                            p.prevent_animation();
                            properties.push(p.as_static_vertex());
                        }
                        else {
                            properties.push(p.as_anim_vertex());
                        }
                    }
                }
            }
        }
        properties
    }

    /// Convert the LineMarks contained in the main vector into a vector
    /// of sub-line understandable by the renderer, then returns it.
    pub fn get_linemarks_properties(&mut self) -> Vec<VertexSubLine> {
        self.layers.sort();
        let mut properties : Vec<VertexSubLine> = Vec::<VertexSubLine>::new();
        for layer in &self.layers {
            for mark in layer.get_all_marks() {
                if let Mark::Line(l) = mark {
                    properties.append(&mut l.to_subline());
                }
            }
        }
        properties
    }

    /// Convert the MarkText contained in the main vector into a vector of a lot of things...
    pub fn get_textmarks_properties(&mut self) -> (Vec<VertexText>,LinkedList<TextMarkCmd>,LinkedList<Glyph>) {
        let mut chars = LinkedList::new();
        let mut commands = LinkedList::new();
        let mut properties = Vec::new();
        let mut cur: usize = 0;
        for layer in &self.layers {
            for mark in layer.get_all_marks() {
                if let Mark::Text(t) = mark {
                    if self.contains_font(t.get_font())
                    {
                        let face = self.fonts.get_face(t.get_font()).unwrap();
                        face.prepare_string(t.get_text());
                        let vtx = face.drawing_commands(t.get_x(), t.get_y(), t.get_z(), t.get_text());
                        let color = mark.get_color().clone();
                        commands.push_front(TextMarkCmd::new(t.get_font(), color, cur, vtx.len()));
                        chars.extend(face.get_writable());
                        cur+= vtx.len();
                        properties.extend(vtx);
                    }
                }
            }
        }
        (properties,commands,chars)
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use properties::color::Color;
    use properties::size::Size;
    use crate::marks::pointmark::Shape;
    use crate::MarkMacro;

    fn vertex_point_is_equal(v1 : VertexPoint, v2 : VertexPoint) -> bool
    {
        if v1.1 == v2.1 && v1.2 == v2.2 && v1.3 == v2.3 && v1.4 == v2.4 && v1.5 == v2.5 && v1.6 == v2.6 && 
            v1.7 == v2.7 && v1.8 == v2.8 && v1.9 == v2.9 && v1.10 == v2.10 && v1.11 == v2.11 && 
            v1.12 == v2.12 && v1.13 == v2.13 && v1.14 == v2.14 {
            return true;
        }
        false
    }

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
            .set_size((0.5, 0.3)).set_rotation(90.0).set_color((1.0, 0.0, 0.5, 1.0));

        let marks_properties = c.get_pointmarks_properties();
        assert!(vertex_point_is_equal(marks_properties[0], ([1.0, 5.0, 9.0], [1.0, 5.0, 9.0], 0.0, [0.0, 0.0], [0.0, 0.0], 
            0.0, [0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0], 0.0, 0.0, 0.0, 0.0, 0, 0, 0.0)));
        assert!(vertex_point_is_equal(marks_properties[1], ([0.0, 0.0, 0.0], [0.0, 0.0, 0.0], 0.0, [0.0, 0.0], [0.0, 0.0], 
            0.0, [0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0], 0.0, 0.0, 0.0, 0.0, 1, 1, 0.0)));
        assert!(vertex_point_is_equal(marks_properties[2], ([3.6, 5.0, 9.2], [3.6, 5.0, 9.2], 0.0, [0.5, 0.3], [0.5, 0.3], 
            0.0, [1.0, 0.0, 0.5, 1.0], [1.0, 0.0, 0.5, 1.0], 0.0, 90.0, 90.0, 0.0, 2, 2, 0.0)));
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
    fn set_current_layer()
    {
        let mut c = Contrast::new();
        c.init();
        c.add_layers(1);

        let mut m1 = c.add_point_mark().get_id();
        let mut m2 = c.add_point_mark().get_id();

        assert_eq!(m1, c.get_layer(0).unwrap().marks.get(0).unwrap().get_id());
        assert_eq!(m2, c.get_layer(0).unwrap().marks.get(1).unwrap().get_id());
        assert!(c.get_layer(0).unwrap().invalid_indexes.is_empty());
        assert!(c.get_layer(1).unwrap().marks.is_empty());
        assert!(c.get_layer(1).unwrap().invalid_indexes.is_empty());

        c.set_current_layer(1);

        let m3 = c.add_point_mark().get_id();
        assert_eq!(m3, c.get_layer(1).unwrap().marks.get(0).unwrap().get_id());
        
        c.get_layer_mut(1).unwrap().add_mark(&mut m1);

        assert!(!c.get_layer(0).unwrap().invalid_indexes.is_empty());
        assert!(c.get_layer(1).unwrap().invalid_indexes.is_empty());
        assert_eq!(m2, c.get_layer(0).unwrap().marks.get(1).unwrap().get_id());
        assert_eq!(m1, c.get_layer(1).unwrap().marks.get(1).unwrap().get_id());
        assert_eq!(c.get_layer(0).unwrap().get_marks_nb(), 1);
        assert_eq!(c.get_layer(1).unwrap().get_marks_nb(), 2);
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
