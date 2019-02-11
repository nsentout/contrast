/// General structures ///
#[derive(Copy, Clone, Debug)]
pub struct Position {
    pub x : f32,
    pub y : f32,
    pub z : f32
}

impl Position { // TODO: éviter de dupliquer la fonction as_array()
    pub fn as_array(self) -> [f32; 3] {
        [self.x, self.y, self.z]
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Size {
    pub width : f32,
    pub height : f32
}

impl Size {
    pub fn as_array(self) -> [f32; 2] {
        [self.width, self.height]
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Color {
    pub r : f32,
    pub g : f32,
    pub b : f32,
    pub a : f32
}

impl Color {
    pub fn as_array(self) -> [f32; 4] {
        [self.r, self.g, self.b, self.a]
    }
}


/// Mark common properties ///
#[derive(Copy, Clone, Debug)]
pub struct MarkProperties {
    pub id : u32,   // TODO: enlever les pub où c'est possible
    pub center: Position,
    pub size : Size,
    pub color: Color,
    pub rotation : f32,
}

impl MarkProperties {
    pub fn default() -> Self
    {
        MarkProperties {
            id : 0,
            center: Position { x : 0.0, y : 0.0, z : 0.0 },
            size : Size { width : 0.0, height : 0.0},
            color: Color { r : 0.0, g : 0.0, b : 0.0, a : 0.0 },
            rotation : 0.0,
        }
    }
}