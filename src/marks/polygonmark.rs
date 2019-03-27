use crate::MarkMacro;
use crate::markproperties::MarkProperties;
use properties::position::Position;
use mark_macro_derive::MarkMacro;


/// This is the type that will receive our shaders when we will want to render our polygon marks.
/// We could describe it this way to be more clear :
/// type VertexPolygon = (color, rotation, origin).
pub type VertexPolygon = ([f32; 4], f32, [f32; 3]);

/// This is the structure that describes the marks of type Polygon.
/// Each type of mark share some properties, that is an id, a size,
/// a color and a rotation. Those properties are described by the
/// attribute common_properties.
/// Polygon marks also have a vector of positions representing its points,
/// a stroke width and a boolean to indicate whether or not we must
/// draw the stroke.
#[derive(MarkMacro, Clone, Debug)]
pub struct PolygonMark {
    pub(crate) common_properties : MarkProperties,
    pub(crate) points : Vec<Position>,
    pub(crate) stroke_width : f32,
    pub(crate) fill : bool /*TODO */
}

impl PolygonMark {
    /// Simply returns a new instance of PolygonMark, initializing
    /// all attributes to their default value.
    pub fn new() -> Self {
        PolygonMark {
            common_properties : MarkProperties::new(),
            points : Vec::<Position>::new(),
            stroke_width : 0.0,
            fill : false
        }
    }

    /// Converts a PolygonMark into a VertexPolygon, which is a type
    /// understandable by the renderer.
    pub fn as_vertex(&self) -> Vec<VertexPolygon> {
        let mut vertex_polygon : Vec<VertexPolygon> = Vec::<VertexPolygon>::new();
        if self.points.len()>2 {
            let origin = self.points[0];
            let v_origin : VertexPolygon = (*self.common_properties.color.to_array(),
            self.common_properties.rotation,
            *origin.to_array());
            let mut previous = self.points[0];
            for curr_vertex in self.points.clone() {
                if (previous != curr_vertex) {
                    vertex_polygon.push(v_origin);
                    let v_left : VertexPolygon = (*self.common_properties.color.to_array(),
                    self.common_properties.rotation,
                    *curr_vertex.to_array());
                    vertex_polygon.push(v_left);
                    let v_right : VertexPolygon = (*self.common_properties.color.to_array(),
                    self.common_properties.rotation,
                    *previous.to_array());
                    vertex_polygon.push(v_right);
                    previous = curr_vertex;
                }
            }
        }
        vertex_polygon
    }

    /// Add a point to a polygon. You can pass as argument a tuple of 3 floats or
    /// a Position directly
    pub fn add_point<P : Into <Position>>(&mut self, point : P) -> &mut Self {
        self.points.push(point.into());
        self
    }

    pub fn set_stroke_width(&mut self, stroke_width : f32) -> &mut Self {
        self.stroke_width = stroke_width;
        self
    }

    pub fn set_fill(&mut self) -> &mut Self {
        self.fill = true;
        self
    }

    pub fn set_empty(&mut self) -> &mut Self {
        self.fill = false;
        self
    }

}
