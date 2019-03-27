#[macro_use]
extern crate lazy_static;
use std::time::Instant;

/// Timer accessible everywhere in the library.
lazy_static! {
    static ref TIMER: Instant = Instant::now();
}

/// Returns the number of milliseconds passed since launch.
pub fn elapsed_time_float() -> f32 {
    let elapsed = TIMER.elapsed();
    let t64 = elapsed.as_secs() as f64 + (elapsed.subsec_millis() as f64 * 1e-3);
    let t = t64 as f32;
    t
}

pub mod markscontainer;
pub mod markproperties;
pub mod marks;
pub mod layer;
pub mod camera;


/// This is the trait that all mark types will derive
/// from. They will hence have access to those methods
/// and their implementation, described in mark_macro_derive/src/lib.rs.
pub trait MarkMacro
{
    /// Returns the id of a mark.
    fn get_id(&self) -> properties::markid::MarkId;

    /// Returns the size of a mark.
    fn get_size(&self) -> properties::size::Size;

    /// Returns the color of a mark.
    fn get_color(&self) -> properties::color::Color;

    /// Returns the rotation of a mark.
    fn get_rotation(&self) -> f32;

    /// Returns the layer index whose mark is bound to.
    fn get_layer_index(&self) -> usize;

    /// Set the size of a mark. You can pass as argument a tuple of 2 floats (width and height) or
    /// a Size directly.
    fn set_size<S : Into <properties::size::Size>>(&mut self, size : S) -> &mut Self;

    /// Set the color of a mark. You can pass as argument a tuple of 4 floats (rgba) or
    /// a Color directly.
    fn set_color<C : Into <properties::color::Color>>(&mut self, color : C) -> &mut Self;

    /// Set the rotation of a mark.
    fn set_rotation(&mut self, rotation : f32) -> &mut Self;
}