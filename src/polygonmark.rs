use properties::*;

/*
 *  This is the structure that describes the marks of type Polygon.
 *  Each type of mark share some properties, that is an id, a position,
 *  a size, a color and a rotation. Those properties are described by the
 *  attribute common_properties.
 *  Polygon marks also have a vector of positions representing its points,
 *  a stroke width and a boolean to indicate whether or not we must
 *  draw the stroke.
 */
#[derive(Debug)]
pub struct PolygonMark {
    pub common_properties : MarkProperties,
    points : Vec<Position>,
    stroke_width : f32,
    full : bool
}

impl PolygonMark {
    /*
     *   Simply returns a new instance of PolygonMark, initializing
     *   all attributes to their default value, except the id.
     */
    pub fn new(id : usize) -> Self {
        PolygonMark {
            common_properties : MarkProperties::default(id),
            points : Vec::<Position>::new(),
            stroke_width : 0.0,
            full : false
        }
    }

    pub fn add_point(&mut self, x : f32, y : f32, z : f32) -> &mut Self {
        self.points.push(Position { x, y, z });
        self
    }

    pub fn get_id(&self) -> usize
    {
        self.common_properties.id
    }

    pub fn set_size(&mut self, width : f32, height : f32) -> &mut Self {
        self.common_properties.size = Size { width, height };
        self
    }

    pub fn set_color(&mut self, r : f32, g : f32, b : f32, a : f32) -> &mut Self {
        self.common_properties.color = Color { r, g, b, a };
        self
    }

    pub fn set_rotation(&mut self, rotation : f32) -> &mut Self {
        self.common_properties.rotation = rotation;
        self
    }

    pub fn set_stroke_width(&mut self, stroke_width : f32) -> &mut Self {
        self.stroke_width = stroke_width;
        self
    }

    pub fn set_full(&mut self) -> &mut Self {
        self.full = true;
        self
    }

    pub fn set_empty(&mut self) -> &mut Self {
        self.full = false;
        self
    }
}