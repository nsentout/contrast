/// Structure representing the identifier of a mark.
/// The first index mark_index indicates where to find the mark
/// in its layer.
/// The second index layer_index indicates where to find the
/// mark's layer in contrast.
/// It also contains a boolean which indicates whether or not this mark
/// is valid, meaning it will be displayed.
#[derive(Copy, Clone, Debug)]
pub struct MarkId {
    pub mark_index : usize,
    pub layer_index : usize,
    pub valid : bool
}

impl MarkId {
    /// Simply returns a new instance of MarkId, initializing
    /// all attributes to their default values.
    pub fn new() -> Self {
        MarkId {
            mark_index : 0,
            layer_index : 0,
            valid : true
        }
    }
}

impl PartialEq for MarkId {
    fn eq(&self, other: &MarkId) -> bool {
        self.mark_index == other.mark_index 
            && self.layer_index == other.layer_index
            && self.valid == other.valid
    }
}
