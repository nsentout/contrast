use properties::*;

#[derive(Debug)]
pub enum LineMode {
    Linear,
    Dashed,
    Dotted
}

#[derive(Debug)]
pub struct LineMark {
    pub common_properties : MarkProperties,
    points : Vec<Position>,
    thickness : f32,
    mode : LineMode
}

impl LineMark {
    pub fn new(id : usize) -> Self {
        LineMark {
            common_properties : MarkProperties::default(id),
            points : Vec::<Position>::new(),
            thickness : 0.0,
            mode : LineMode::Linear
        }
    }

    pub fn get_id(&self) -> usize
    {
        self.common_properties.id
    }

    pub fn set_thickness(&mut self, thickness : f32) -> &mut Self {
        self.thickness = thickness;
        self
    }

    pub fn set_mode(&mut self, mode : LineMode) -> &mut Self {
        self.mode = mode;
        self
    }

    pub fn add_point(&mut self, x : f32, y : f32, z : f32) -> &mut Self {
        self.points.push(Position { x, y, z });
        self
    }
}