use std::collections::BinaryHeap;
use properties::markid::MarkId;
use properties::position::Position;
use crate::MarkMacro;
use crate::marks::mark::Mark;
use crate::markscontainer::Contrast;

/// Structure representing a layer.
/// A Layer has a vector containing his marks and a depth, 0 means 
/// it will be displayed on first plan.
/// A Layer keeps track of indexes where marks have been removed 
/// to replace them later.
#[derive(Debug)]
pub struct Layer {
    pub(crate) marks : Vec<Mark>,
    pub(crate) depth : usize,
    pub(crate) invalid_indexes : BinaryHeap<usize>,
    pub(crate) contrast : *mut Contrast
}

impl Layer {
    /// Simply returns a new instance of Layer, initializing
    /// all attributes to their default values, except the depth.
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

        // Retrieve a copy of the mark in his current layer.
        let mut mark;
        unsafe {
            mark = (*self.contrast).layers.get_mut(markid.layer_index).unwrap().invalidate_and_get_mark(markid);
        }

        // Update the mark according to his new layer
        if self.invalid_indexes.is_empty() {
            mark.set_mark_index(self.marks.len());
        }
        else {
            mark.set_mark_index(self.invalid_indexes.pop().unwrap());
        }
        mark.set_layer_index(self.depth);
        mark.set_valid(true);

        // Update the markid passed as parameter so it stays coherent
        markid.mark_index = self.marks.len();
        markid.layer_index = self.depth;
        markid.valid = true;
            
        // Add the mark to the layer
        self.marks.push(mark);
    }

    /// Move every mark of the Layer.
    /// Example : if <position> is (50.0, 0.0, 0.0), every point of the mark 
    /// will move 50 pixels to the right.
    pub fn move_of<P : Into <Position>>(&mut self, position : P) -> &mut Self {
        let position : Position = position.into();

        for mark in &mut self.marks {
            mark.move_of(position);
        }
        self
    }

    /// Returns the number of valid marks in the layer.
    pub fn get_marks_nb(&self) -> usize {
        self.marks.len() - self.invalid_indexes.len()
    }

    /// Add a mark which was just created and is not in any layer.
    pub(crate) fn force_add_mark(&mut self, mut mark : Mark) {
        // If there is no invalid indexes, just push the mark
         if self.invalid_indexes.is_empty() {
            mark.set_mark_index(self.get_marks_nb());
            self.marks.push(mark);
        }
        // Else, replace the invalid mark with the new mark
        else {
            let first_invalid_index = self.invalid_indexes.pop().unwrap();
            mark.set_mark_index(first_invalid_index);
            let mut invalid_mark = self.marks.get_mut(first_invalid_index).unwrap();
            invalid_mark = &mut mark;
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
    /// with the id <mark>.
    pub(crate) fn get_mark(&self, markid : &MarkId) -> Option<&Mark> {
        self.marks.get(markid.mark_index)
    }

    /// Returns a mutable reference wrapped into an Option of the mark
    /// with the id <markid>.
    pub(crate) fn get_mark_mut(&mut self, markid : &MarkId) -> Option<&mut Mark> {
        self.marks.get_mut(markid.mark_index)
    }

    /// Returns a reference of the vector containing all the marks.
    pub(crate) fn get_all_marks(&self) -> &Vec<Mark> {
        &self.marks
    }
 
    /// Returns a mutable reference of the last added mark.
    pub(crate) fn get_last_mark_mut(&mut self) -> &mut Mark {
        self.marks.last_mut().unwrap()
    }

    /// Indicate whether or not the layer contains this mark.
    pub(crate) fn contains(&self, markid : &MarkId) -> bool {
        if let None = self.marks.get(markid.mark_index) {
            return false;
        }
        true
    }
}
