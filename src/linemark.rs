use properties::*;

/*
 *  Those are the different ways we shoud be able to
 *  draw lines.
 */
#[derive(Debug)]
pub enum LineMode {
    Linear,
    Dashed,
    Dotted
}

/*
 *  This is the structure that describes the marks of type Line (or polyline).
 *  Each type of mark share some properties, that is an id, a position,
 *  a size, a color and a rotation. Those properties are described by the
 *  attribute common_properties.
 *  Line marks also have a vector of positions representing its points,
 *  a thickness and a mode to draw them differently.
 */
#[derive(Debug)]
pub struct LineMark {
    pub common_properties : MarkProperties,
    points : Vec<Position>,
    thickness : f32,
    mode : LineMode
}

impl LineMark {
    /*
     *   Simply returns a new instance of LineMark, initializing
     *   all attributes to their default value, except the id.
     */
    pub fn new(id : usize) -> Self {
        LineMark {
            common_properties : MarkProperties::default(id),
            points : Vec::<Position>::new(),
            thickness : 0.0,
            mode : LineMode::Linear
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

    pub fn set_thickness(&mut self, thickness : f32) -> &mut Self {
        self.thickness = thickness;
        self
    }

    pub fn set_mode(&mut self, mode : LineMode) -> &mut Self {
        self.mode = mode;
        self
    }
}