use crate::MarkMacro;
use properties::position::Position;
use properties::color::Color;
use properties::markid::MarkId;
use mark_macro_derive::MarkMacro;


/// This is the type that will receive our shaders when we will want to render our polygon marks.
/// We could describe it this way to be more clear :
/// type VertexPolygon = (color, rotation, origin).                                             //centroid
pub type VertexPolygon = ([f32; 4], f32, [f32; 3], [f32; 3], [f32; 3], [f32; 3], f32, [f32; 3], u32);

/// This is the structure that describes the marks of type Polygon.
/// Each type of mark share some properties, that is an id and a color.
/// Polygon marks also have a vector of positions representing its points,
/// a stroke width and a boolean to indicate whether or not we must
/// draw the stroke.
#[derive(MarkMacro, PartialEq, Clone, Debug)]
pub struct PolygonMark {
    pub(crate) markid : MarkId,
    pub(crate) color : Color,
    pub(crate) rotation : f32,
    pub(crate) points : Vec<Position>,
    pub(crate) stroke_width : f32,
    pub(crate) fill : bool
}

impl PolygonMark {
    /// Simply returns a new instance of PolygonMark, initializing
    /// all attributes to their default value.
    pub fn new() -> Self {
        PolygonMark {
            markid : MarkId::new(),
            color : Color::default(),
            rotation : 0.0,
            points : Vec::<Position>::new(),
            stroke_width : 15.0,
            fill : false
        }
    }

    /// Compute the centroid of the polygon
    /// and return the centroid
    /// which is the arithmetic mean position
    /// of all the points in the polygon
    fn compute_centroid(&self) -> Position {
        let mut centroid = Position { x : 0.0, y : 0.0, z : 0.0 };
        for point in self.points.clone() {
            centroid += point;
        }
        centroid = Position { x : centroid.x / self.points.len() as f32,
                              y : centroid.y / self.points.len() as f32,
                              z : centroid.z / self.points.len() as f32};
        centroid
    }

    /// Converts a PolygonMark into a VertexPolygon, which is a type
    /// understandable by the renderer.
    pub fn as_vertex(&self) -> Vec<VertexPolygon> {
        let mut vertex_polygon : Vec<VertexPolygon> = Vec::<VertexPolygon>::new();
        let centroid : Position = self.compute_centroid();
        if self.points.len()>2 {
            let mut previous = self.points[0];
            let mut origin = self.points[0];
            let mut target = self.points[0];
            let mut fill_mode = 0;
            if self.fill  {
                fill_mode = 1;
            }
            for next in self.points.clone() {
                if previous == origin { previous = self.points[self.points.len()-1]}
                let vl : VertexPolygon = (*self.color.to_array(),
                self.rotation,
                *origin.to_array(), *target.to_array(), *previous.to_array(),
                *next.to_array(),self.stroke_width, *centroid.to_array(), fill_mode);
                vertex_polygon.push(vl);
                previous = origin;
                origin = target;
                target = next;
            }
            let vl : VertexPolygon = (*self.color.to_array(),
            self.rotation,
            *origin.to_array(), *target.to_array(),
            *previous.to_array(), *self.points[0].to_array(),self.stroke_width,
            *centroid.to_array(), fill_mode);
            vertex_polygon.push(vl);
            let vr : VertexPolygon = (*self.color.to_array(),
            self.rotation,
            *self.points[self.points.len()-1].to_array(), *self.points[0].to_array(),
            *self.points[self.points.len()-2].to_array(), *self.points[1].to_array(),
            self.stroke_width, *centroid.to_array(), fill_mode);
            vertex_polygon.push(vr);
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

    pub fn get_stroke_width(&self) -> f32 {
        self.stroke_width
    }

    pub(crate) fn get_points_mut(&mut self) -> &mut Vec<Position> {
        &mut self.points
    }

    pub fn get_points(&mut self) -> &mut Vec<Position> {
        &mut self.points
    }
    // TODO : ajouter getter
}
//TODO : ajouter test compute_centroid
