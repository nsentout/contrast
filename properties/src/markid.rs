/// Structure representing the identifier of a mark.
/// Contains an id, which is the index where the mark is stored,
/// as well as a number of layer.
#[derive(Copy, Clone, Debug)]
pub struct MarkId {
    pub id : usize,
    pub layer : usize
}

impl MarkId {
    /// Simply returns a new instance of MarkId, initializing
    /// all attributes to their default values, except the id.
    pub fn new(id : usize) -> Self {
        MarkId {
            id,
            layer : 0
        }
    }
}
