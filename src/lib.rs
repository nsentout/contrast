pub mod markscontainer;
pub mod markproperties;
pub mod marks;
pub mod camera;


/// This is the trait that all mark types will derive
/// from. They will hence have access to those methods
/// and their implementation, described in mark_macro_derive/src/lib.rs.
pub trait MarkMacro {
    /// Returns the id of a mark
    fn get_id(&self) -> usize;

    /// Set the size of a mark. You can pass as argument a tuple of 2 floats (width and height) or
    /// a Size directly
    fn set_size<S : Into <properties::size::Size>>(&mut self, size : S) -> &mut Self;

    /// Set the color of a mark. You can pass as argument a tuple of 4 floats (rgba) or
    /// a Color directly
    fn set_color<C : Into <properties::color::Color>>(&mut self, color : C) -> &mut Self;

    /// Set the rotation of a mark.
    fn set_rotation(&mut self, rotation : f32) -> &mut Self;
}