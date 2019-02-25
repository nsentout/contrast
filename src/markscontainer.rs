use crate::marks::pointmark::PointMark;
use crate::marks::pointmark::VertexPoint;
use crate::marks::linemark::LineMark;
use crate::properties::color::Color;

#[derive(Debug)]
pub enum Mark {
	Point(PointMark),
	Line(LineMark),
}

impl Mark  {
    pub fn get_id(&self) -> usize {  //TODO: éviter de dupliquer ce que font ces matchs
        match self {
            Mark::Point(p) => p.common_properties.id,
            Mark::Line(l) => l.common_properties.id
        }
    }

    pub(self) fn set_id(&mut self, id : usize) -> &mut Self {
        match self {
            Mark::Point(p) => p.common_properties.id = id,
            Mark::Line(l) => l.common_properties.id = id
        }
        self
    }

    pub fn set_size(&mut self, width : f32, height : f32) -> &mut Self {
        match self {
            Mark::Point(p) => {
                p.common_properties.size.width = width;
                p.common_properties.size.height = height;
            },
            Mark::Line(l) => {
                l.common_properties.size.width = width;
                l.common_properties.size.height = height;
            }
        }
        self
    }

    pub fn get_color(&self) -> Color {
        match self {
            Mark::Point(p) => {
                Color {
                    r : p.common_properties.color.r,
                    g : p.common_properties.color.g,
                    b : p.common_properties.color.b,
                    a : p.common_properties.color.a
                }
            },
            Mark::Line(l) => {
                 Color {
                    r : l.common_properties.color.r,
                    g : l.common_properties.color.g,
                    b : l.common_properties.color.b,
                    a : l.common_properties.color.a
                }
            }
        }
    }

    pub fn set_color(&mut self, r : f32, g : f32, b : f32, a : f32) -> &mut Self {
        match self {
            Mark::Point(p) => {
                p.common_properties.color.r = r;
                p.common_properties.color.g = g;
                p.common_properties.color.b = b;
                p.common_properties.color.a = a;
            },
            Mark::Line(l) => {
                l.common_properties.color.r = r;
                l.common_properties.color.g = g;
                l.common_properties.color.b = b;
                l.common_properties.color.a = a;
            }
        }
        self
    }

    pub fn set_rotation(&mut self, rotation : f32) -> &mut Self {
        match self {
            Mark::Point(p) => p.common_properties.rotation = rotation,
            Mark::Line(l) =>  l.common_properties.rotation = rotation
        }
        self
    }
}

/// This is the main structure of the library. It contains all the marks
/// displayed on screen. The user can add, get, remove and modify marks 
/// as he wishes. The id of each mark represents their index in the vector,
/// which allows for adding and removal of marks in O(1).
pub struct Contrast {
    marks : Vec<Mark>
}

impl Contrast {
    /// Simply returns a new instance of Contrast, initializing
    /// the vector containing all the marks.
    pub fn new() -> Self { 
        Contrast {
            marks : Vec::<Mark>::new()
        }
    }

    /// Create a mark of type "point" with default values and add it into the main
    /// vector, then returns a mutable reference of this newly created mark,
    /// all of this in O(1). We return a mutable reference because we want
    /// to be able to modify it just after calling add_point_mark in a way
    /// similar to this : add_point_mark.set_rotation(90.0).
    pub fn add_point_mark(&mut self) -> &mut PointMark {
        let point = Mark::Point(PointMark::new(self.marks.len()));
        self.marks.push(point);
        
        match self.marks.last_mut().unwrap() {
            Mark::Point(p) => p,
            _ => panic!("A problem occured when adding a new point mark!")
        }
    }

    /// Same behavior than add_point_mark but it adds a mark of type "line".
    pub fn add_line_mark(&mut self) -> &mut LineMark {
        let line = Mark::Line(LineMark::new(self.marks.len()));
        self.marks.push(line);
        
        match self.marks.last_mut().unwrap() {
            Mark::Line(l) => l,
            _ => panic!("A problem occured when adding a new line mark!")
        }
    }

    /// Returns an Option of the mark at the index "id". If there is no mark having this id,
    /// returns None.
    pub fn get_mark(&mut self, id : usize) -> Option<&mut Mark> {
        self.marks.get_mut(id)
    }

    /// Remove the mark with the id mark. We will call this mark the target.
    /// We first set the id of the last element of the vector containing all the marks
    /// to the target's id (mark).
    /// We then swap the target with the last element. We can now safely remove the target.
    /// This way, the mark that was the last element before the removal holds now the id
    /// of the target. This explains why we can always use "self.marks.len()" when we
    /// want to give a unique id to a new mark. Furthermore, this allows us to remove
    /// an element in O(1).
    pub fn remove_mark(&mut self, mark : usize) {
        if !self.marks.is_empty() { self.marks.last_mut().unwrap().set_id(mark); }
        if self.marks.len() > mark { self.marks.swap_remove(mark); }
    }

    /// Convert the MarkPoints contained in the main vector into a vector
    /// of vertices understandable by the renderer, then returns it.
    pub fn get_pointmarks_properties(self) -> Vec<VertexPoint> {    // TODO: éviter cette copie
        let mut properties : Vec<VertexPoint> = Vec::<VertexPoint>::new();
        for pt in &self.marks {
            if let Mark::Point(p) = pt {
                properties.push(p.as_vertex());
            }
        }
        properties
    }

}

// TODO: mettre à jour les tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::marks::pointmark::Shape;
    use crate::MarkMacro;

    #[test]
    fn new()
    {
        assert_eq!(Contrast::new().get_pointmarks_properties().len(), 0);
    }

    #[test]
    fn add_point_mark()
    {
        let mut c = Contrast::new();
        
        let m1 = c.add_point_mark().get_id();
        
        assert_eq!(c.point_marks.len(), 1);
        assert_eq!(m1, 0);

        let m2 = c.add_point_mark().get_id();
        let m3 = c.add_point_mark().get_id();

        assert_eq!(c.point_marks.len(), 3);
        assert_eq!(m1, 0);
        assert_eq!(m2, 1);
        assert_eq!(m3, 2);
    }

    #[test]
    fn remove_point_mark()
    {
        let mut c = Contrast::new();

        let m1 = c.add_point_mark().get_id();
        let m2 = c.add_point_mark().get_id();

        assert_eq!(m1, 0);
        assert_eq!(m2, 1);

        c.remove_point_mark(m1);

        assert_eq!(c.point_marks.len(), 1);
        assert_eq!(c.point_marks.get(0).unwrap().get_id(), 0);
    }

    #[test]
    fn get_pointmarks_properties()
    {
        let mut c = Contrast::new();

        c.add_point_mark().set_position((1.0, 5.0, 9.0));
        c.add_point_mark().set_shape(Shape::Rectangle);
        c.add_point_mark().set_position((3.6, 5.0, 9.2)).set_shape(Shape::Triangle)
            .set_size(0.5, 0.3).set_rotation(90.0).set_color(1.0, 0.0, 0.5, 1.0)
            .set_selection_angle(120.0).set_start_radius(45.0);

        let marks_properties = c.get_pointmarks_properties();

        assert_eq!(marks_properties[0], ([1.0, 5.0, 9.0], [0.0, 0.0], [0.0, 0.0, 0.0, 0.0], 0.0, 0, 0.0, 0.0));
        assert_eq!(marks_properties[1], ([0.0, 0.0, 0.0], [0.0, 0.0], [0.0, 0.0, 0.0, 0.0], 0.0, 1, 0.0, 0.0));
        assert_eq!(marks_properties[2], ([3.6, 5.0, 9.2], [0.5, 0.3], [1.0, 0.0, 0.5, 1.0], 90.0, 2, 120.0, 45.0));
    }

    #[test]
    fn add_line_mark()
    {
        let mut c = Contrast::new();
        
        let m1 = c.add_line_mark().get_id();
        
        assert_eq!(c.line_marks.len(), 1);
        assert_eq!(m1, 0);

        let m2 = c.add_line_mark().get_id();
        let m3 = c.add_line_mark().get_id();

        assert_eq!(c.line_marks.len(), 3);
        assert_eq!(m1, 0);
        assert_eq!(m2, 1);
        assert_eq!(m3, 2);
    }

    #[test]
    fn remove_line_mark()
    {
        let mut c = Contrast::new();
        
        let m1 = c.add_line_mark().get_id();
        let m2 = c.add_line_mark().get_id();

        assert_eq!(m1, 0);
        assert_eq!(m2, 1);

        c.remove_line_mark(m1);

        assert_eq!(c.line_marks.len(), 1);
        assert_eq!(c.line_marks.get(0).unwrap().get_id(), 0);
    }
}