pub mod markscontainer;
pub mod properties;
pub mod pointmark;
pub mod linemark;
pub mod camera;


/// This is the trait that all mark types will derive
/// from. They will hence have access to those methods
/// and their implementation, described in mark_macro_derive/src/lib.rs.
pub trait MarkMacro {
    fn get_id(&self) -> usize;
    fn set_size(&mut self, width : f32, height : f32) -> &mut Self;
    fn set_color(&mut self, r : f32, g : f32, b : f32, a : f32) -> &mut Self;
    fn set_rotation(&mut self, rotation : f32) -> &mut Self;
}