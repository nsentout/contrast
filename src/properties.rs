/*
 *  Structure representing a 3D position
 */
#[derive(Copy, Clone, Debug)]
pub struct Position {
    pub x : f32,
    pub y : f32,
    pub z : f32
}

impl Position { // TODO: éviter de dupliquer la fonction as_array()
    /*
     *  Convert a position structure to an array.
     *  Useful when converting our marks to vertices. 
     */
    pub fn as_array(self) -> [f32; 3] {
        [self.x, self.y, self.z]
    }
}

/*
 *  Structure representing a size
 */
#[derive(Copy, Clone, Debug)]
pub struct Size {
    pub width : f32,
    pub height : f32
}

impl Size {
    /*
     *  Convert a size structure to an array.
     *  Useful when converting our marks to vertices. 
     */
    pub fn as_array(self) -> [f32; 2] {
        [self.width, self.height]
    }
}

/*
 *  Structure representing a RGBA color
 */
#[derive(Copy, Clone, Debug)]
pub struct Color {
    pub r : f32,
    pub g : f32,
    pub b : f32,
    pub a : f32
}

impl Color {
    /*
     *  Convert a color structure to an array.
     *  Useful when converting our marks to vertices. 
     */
    pub fn as_array(self) -> [f32; 4] {
        [self.r, self.g, self.b, self.a]
    }
}


/*
 *  Structure representing the properties shared
 *  by every type of marks, that is an id, a position,
 *  a size, a color and a rotation.
 */
#[derive(Debug)]
pub struct MarkProperties {
    pub id : usize,
    pub center: Position,
    pub size : Size,
    pub color: Color,
    pub rotation : f32,
}

impl MarkProperties {
    /*
     *   Simply returns a new instance of MarkProperties, initializing
     *   all attributes to their default values, except the id.
     */
    pub fn default(id : usize) -> Self
    {
        MarkProperties {
            id,
            center: Position { x : 0.0, y : 0.0, z : 0.0 },
            size : Size { width : 0.0, height : 0.0},
            color: Color { r : 0.0, g : 0.0, b : 0.0, a : 0.0 },
            rotation : 0.0,
        }
    }
}