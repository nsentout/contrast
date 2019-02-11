use properties::*;

pub type VertexPoint = ([f32; 3], [f32; 2], [f32; 4], f32, u32, f32, f32);
// position; size; color; rotation; shape; selection_angle; start_radius

#[derive(Copy, Clone, Debug)]
pub enum Shape {
    None = 0,
    Rectangle = 1,
    Triangle = 2,
    /*Circle = 3,
    Point = 4,
    Squircle = 5,
    Diamond = 6,
    Donut = 7,
    Pin = 8,
    Club = 9,
    Heart = 10,
    Spade = 11,
    Chevron = 12,
    Clover = 13,
    Ring = 14,
    Tag = 15,
    Cross = 16,
    Asterisk = 17,
    Infinity = 18,
    Arrow = 19*/
}

#[derive(Copy, Clone, Debug)]
pub struct PointMark {
    pub common_properties : MarkProperties,
    pub shape : Shape,
    pub selection_angle : f32,
    pub start_radius : f32
}

impl PointMark {
    pub fn default() -> Self {
        PointMark {
            common_properties : MarkProperties::default(),
            shape : Shape::None,
            selection_angle : 0.0,
            start_radius : 0.0
        }
    }

    pub fn set_position(&mut self, x : f32, y : f32, z : f32) -> &mut Self {
        self.common_properties.center = Position { x, y, z };
        self
    }
    // TODO: rendre ces méthodes communes à toutes les marques
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

    pub fn set_shape(&mut self, shape : Shape) -> &mut Self {
        self.shape = shape;
        self
    }

    pub fn set_selection_angle(&mut self, selection_angle : f32) -> &mut Self {
        self.selection_angle = selection_angle;
        self
    }

    pub fn set_start_radius(&mut self, start_radius : f32) -> &mut Self {
        self.start_radius = start_radius;
        self
    }
}

impl std::cmp::PartialEq for PointMark {
    fn eq(&self, mark: &PointMark) -> bool {
        self.common_properties.id == mark.common_properties.id
    }
}