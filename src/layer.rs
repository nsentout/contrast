use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::mem;
use properties::markid::MarkId;
use properties::position::Position;
use crate::marks::mark::Mark;
use crate::markscontainer::Contrast;

/// Structure representing a layer.
/// A Layer has a vector containing his marks and a depth, 0 means 
/// it will be displayed on first plan.
/// A Layer keeps track of indexes where marks have been removed 
/// to replace them later.
pub struct Layer {
    pub(crate) marks : Vec<Mark>,
    pub(crate) depth : usize,
    pub(crate) invalid_indexes : BinaryHeap<usize>,
    pub(crate) contrast : *mut Contrast
}

impl Ord for Layer
{
    fn cmp(&self, other: &Layer) -> Ordering
    {
        self.depth.cmp(&other.depth)
    }
}

impl PartialOrd for Layer
{
    fn partial_cmp(&self, other: &Layer) -> Option<Ordering>
    {
        Some(self.cmp(other))
    }
}

impl PartialEq for Layer
{
    fn eq(&self, other: &Layer) -> bool
    {
        self.depth == other.depth
    }
}

impl Eq for Layer {}

impl Layer {
    /// Simply returns a new instance of Layer, initializing
    /// all attributes to their default value, except the depth.
    pub(crate) fn new(depth : usize, contrast : *mut Contrast) -> Self {
        Layer {
            marks : Vec::<Mark>::new(),
            depth,
            invalid_indexes : BinaryHeap::new(),
            contrast
        }
    }

    /// Apply a function to each mark of this layer.
    pub fn apply_to_marks(&mut self, f: fn(&mut Mark)) {
        for mut mark in &mut self.marks {
            f(&mut mark);
        }
    }

    /// Add a mark into the layer.
    pub fn add_mark(&mut self, markid : &mut MarkId) {
        // If the mark is already in the layer, returns
        if markid.layer_index == self.depth { return; }

        // Retrieve a copy of the mark in his current layer
        let mut mark;
        unsafe {
            mark = (*self.contrast).layers.get_mut(markid.layer_index).unwrap().invalidate_and_get_mark(markid);
        }

        // Update the mark according to his new layer
        let new_mark_index;
        let mut dirty_layer = false;
        if self.invalid_indexes.is_empty() {
            new_mark_index = self.marks.len();
        }
        else {
            dirty_layer = true;
            new_mark_index = self.invalid_indexes.pop().unwrap();
        }

        mark.set_mark_index(new_mark_index);
        mark.set_layer_index(self.depth);
        mark.set_valid(true);

        // Update the markid passed as parameter so it stays coherent
        markid.mark_index = new_mark_index;
        markid.layer_index = self.depth;
        markid.valid = true;
            
        // Add the mark to the layer
        if !dirty_layer {
            self.marks.push(mark);
        }
        else {
            let invalid_mark = self.marks.get_mut(new_mark_index).unwrap();
            mem::replace(invalid_mark, mark);
        }
    }

    /// Move every mark of the Layer.
    /// Example : if 'position' is (50.0, 0.0, 0.0), every point of the mark 
    /// will move 50 pixels to the right.
    pub fn move_of<P : Into <Position>>(&mut self, position : P) -> &mut Self {
        let position : Position = position.into();

        for mark in &mut self.marks {
            mark.move_of(position);
        }
        self
    }

    /// Returns a reference of the vector containing all the marks.
    pub fn get_all_marks(&self) -> &Vec<Mark> {
        &self.marks
    }
 

    /// Returns the number of valid marks in the layer.
    pub fn get_marks_nb(&self) -> usize {
        self.marks.len() - self.invalid_indexes.len()
    }

    /// Add a mark which was just created and is not in any layer.
    pub(crate) fn force_add_mark(&mut self, mut mark : Mark) {
        mark.set_layer_index(self.depth);
        // If there is no invalid indexes, just push the mark
        if self.invalid_indexes.is_empty() {
            mark.set_mark_index(self.marks.len());
            self.marks.push(mark);
        }
        // Else, replace the invalid mark with the new mark
        else {
            let first_invalid_index = self.invalid_indexes.pop().unwrap();
            mark.set_mark_index(first_invalid_index);
            let invalid_mark = self.marks.get_mut(first_invalid_index).unwrap();
            mem::replace(invalid_mark, mark);
        }
    }

    /// Invalidate the mark represented by markid, making it invisible.
    pub(crate) fn invalidate_mark(&mut self, markid : &mut MarkId) {
        if self.contains(markid) {
            markid.valid = false;
            self.get_mark_mut(markid).unwrap().set_valid(false);
            self.invalid_indexes.push(markid.mark_index);
        }
    }

    /// Invalidate the mark represented by markid, making it invisible, and returns a clone of it.
    pub(crate) fn invalidate_and_get_mark(&mut self, markid : &mut MarkId) -> Mark {
        self.get_mark_mut(markid).unwrap().set_valid(false);
        self.invalid_indexes.push(markid.mark_index);
        self.get_mark_mut(markid).unwrap().clone()
    }

    /// Returns a reference wrapped into an Option of the mark
    /// represented by 'markid'.
    pub(crate) fn get_mark(&self, markid : &MarkId) -> Option<&Mark> {
        self.marks.get(markid.mark_index)
    }

    /// Returns a mutable reference wrapped into an Option of the mark
    /// represented by 'markid'.
    pub(crate) fn get_mark_mut(&mut self, markid : &MarkId) -> Option<&mut Mark> {
        self.marks.get_mut(markid.mark_index)
    }

    /// Returns a mutable reference of the last added mark.
    pub(crate) fn get_last_mark_mut(&mut self) -> &mut Mark {
        self.marks.last_mut().unwrap()
    }

    /// Indicate whether or not the layer contains the mark represented by 'markid'.
    pub(crate) fn contains(&self, markid : &MarkId) -> bool {
        if let None = self.marks.get(markid.mark_index) {
            return false;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::MarkMacro;
    use crate::marks::pointmark::VertexPoint;

    fn vertex_point_is_equal(v1 :VertexPoint ,v2 : VertexPoint) -> bool
    {
        if v1.1 == v2.1 && v1.2 == v2.2 && v1.3 == v2.3 && v1.4 == v2.4 && v1.5 == v2.5 && v1.6 == v2.6 && 
            v1.7 == v2.7 && v1.8 == v2.8 && v1.9 == v2.9 && v1.10 == v2.10 && v1.11 == v2.11 && 
            v1.12 == v2.12 && v1.13 == v2.13 && v1.14 == v2.14 {
            return true;
        }
        false
    }

    #[test]
    fn add_mark()
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
                    assert_eq!(marks_properties.len(), 3);
                    assert_eq!(c.get_mark(&m1).unwrap().get_id(), m1);
                    assert_eq!(c.get_mark(&m2).unwrap().get_id(), m2);
                    assert_eq!(c.get_mark(&m3).unwrap().get_id(), m3);

                    if i != j && j != k && i != k {
                        assert!(vertex_point_is_equal(marks_properties[i], ([100.0, 150.0, 0.0], [100.0, 150.0, 0.0], 0.0, [0.0, 0.0], [0.0, 0.0], 
                            0.0, [0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0], 0.0, 0.0, 0.0, 0.0, 0, 0, 0.0)));
                        assert!(vertex_point_is_equal(marks_properties[j], ([200.0, 250.0, 1.0], [200.0, 250.0, 1.0], 0.0, [0.0, 0.0], [0.0, 0.0], 
                            0.0, [0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0], 0.0, 0.0, 0.0, 0.0, 0, 0, 0.0)));
                        assert!(vertex_point_is_equal(marks_properties[k], ([300.0, 350.0, 2.0], [300.0, 350.0, 2.0], 0.0, [0.0, 0.0], [0.0, 0.0], 
                            0.0, [0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0], 0.0, 0.0, 0.0, 0.0, 0, 0, 0.0)));
                    }
                }
            }
        }
    }
}
