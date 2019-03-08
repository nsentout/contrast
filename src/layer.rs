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

    /// Indicate whether or not the layer contains marks
    pub fn has_no_mark(&self) -> bool {
        self.marks.is_empty()
    }

    /// Add a mark into the layer.
    pub(crate) fn add_mark(&mut self, mark : Mark) {
        self.marks.push(mark);
    }

    /// Returns the depth at which is the layer.
    pub(crate) fn get_depth(&self) -> usize {
        self.depth
    }

    /// Returns a reference wrapped into an Option of the mark
    /// with the id <mark>
    pub(crate) fn get_mark(&self, mark : &MarkId) -> Option<&Mark> {
        self.marks.get(mark.id)
    }

    /// Returns a mutable reference wrapped into an Option of the mark
    /// with the id <mark>
    pub(crate) fn get_mark_mut(&mut self, mark : &MarkId) -> Option<&mut Mark> {
        self.marks.get_mut(mark.id)
    }

    /// Returns a reference of the vector containing all the marks
    pub(crate) fn get_all_marks(&self) -> &Vec<Mark> {
        &self.marks
    }

    // Simply calls swap_remove on the marks vector
    pub(crate) fn swap_remove_mark(&mut self, mark : &MarkId) -> Mark {
        self.marks.swap_remove(mark.id)
    }
 
    /// Returns a mutable reference of the last added mark.
    pub(crate) fn get_last_mark_mut(&mut self) -> &mut Mark {
        self.marks.last_mut().unwrap()
    }
}
