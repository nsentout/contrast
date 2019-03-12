/// Structure representing the identifier of a mark.
/// Contains an id, which is the index where the mark is stored,
/// as well as a number of layer.
#[derive(Copy, Clone, Debug)]
pub struct MarkId {
    pub mark_index : usize,
    pub layer_index : usize,
    pub valid : bool
}

impl MarkId {
    /// Simply returns a new instance of MarkId, initializing
    /// all attributes to their default values, except the mark_index.
    pub fn new(mark_index : usize) -> Self {
        MarkId {
            mark_index,
            layer_index : 0,
            valid : true
        }
    }
}

impl PartialEq for MarkId {
    fn eq(&self, other: &MarkId) -> bool {
        self.mark_index == other.mark_index 
            && self.layer_index == other.layer_index 
    }
}
