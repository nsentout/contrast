use properties::*;

#[derive(Copy, Clone, Debug)]
pub enum LineMode {
    Linear,
    Dashed,
    Dotted
}

#[derive(Copy, Clone, Debug)]
pub struct LineMark {
    pub common_properties : MarkProperties,
    points : u32,   // TODO: changer par une liste
    thickness : f32,
    mode : LineMode
}

impl LineMark {
    pub fn default() -> Self {
        LineMark {
            common_properties : MarkProperties::default(),
            points : 0,
            thickness : 0.0,
            mode : LineMode::Linear
        }
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

impl std::cmp::PartialEq for LineMark {
    fn eq(&self, mark: &LineMark) -> bool {
        self.common_properties.id == mark.common_properties.id
    }
}