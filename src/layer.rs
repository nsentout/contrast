use properties::markid::MarkId;
use properties::position::Position;
use crate::marks::mark::Mark;

/// Structure representing a layer.
/// A Layer has a vector containing his marks
/// and a depth, 0 means it will be displayed
/// on first plan.
#[derive(Debug)]
pub struct Layer {
    marks : Vec<Mark>,
    depth : usize
}

impl Layer {
    /// Simply returns a new instance of Layer, initializing
    /// all attributes to their default values, except the depth.
    pub(crate) fn new(depth : usize) -> Self {
        Layer {
            marks : Vec::<Mark>::new(),
            depth
        }
    }

    /// Move every mark of the Layer.
    /// Example : if <position> is (50.0, 0.0, 0.0), every point of the mark 
    /// will move 50 pixels to the right.
    pub fn set_position<P : Into <Position>>(&mut self, position : P) -> &mut Self {
        let position : Position = position.into();

        for mark in &mut self.marks {
            mark.move_of(position);
        }
        self
    }

    /// Returns the number of marks in the layer.
    pub fn get_marks_nb(&self) -> usize {
        self.marks.len()
    }

    /// Add a mark into the layer.
    pub(crate) fn add_mark(&mut self, mark : Mark) {
        self.marks.push(mark);
    }

    /// Invalidate the mark represented by markid, making it invisible
    pub(crate) fn invalidate_mark(&mut self, markid : &mut MarkId) {
        if self.contains(markid) {
            markid.valid = false;
            self.get_mark_mut(markid).unwrap().set_valid(false);
        }
    }

    /// Invalidate the mark represented by markid, making it invisible, and returns a clone of it.
    pub(crate) fn invalidate_and_get_mark(&mut self, markid : &mut MarkId) -> Mark {
        self.get_mark_mut(markid).unwrap().set_valid(false);
        self.get_mark_mut(markid).unwrap().clone()
    }

    /// Returns a reference wrapped into an Option of the mark
    /// with the id <mark>
    pub(crate) fn get_mark(&self, markid : &MarkId) -> Option<&Mark> {
        self.marks.get(markid.mark_index)
    }

    /// Returns a mutable reference wrapped into an Option of the mark
    /// with the id <mark>
    pub(crate) fn get_mark_mut(&mut self, markid : &MarkId) -> Option<&mut Mark> {
        self.marks.get_mut(markid.mark_index)
    }

    /// Returns a reference of the vector containing all the marks
    pub(crate) fn get_all_marks(&self) -> &Vec<Mark> {
        &self.marks
    }
 
    /// Returns a mutable reference of the last added mark.
    pub(crate) fn get_last_mark_mut(&mut self) -> &mut Mark {
        self.marks.last_mut().unwrap()
    }

    pub(self) fn contains(&self, markid : &MarkId) -> bool {
        if let None = self.marks.get(markid.layer_index) {
            return false;
        }
        true
    }
}
