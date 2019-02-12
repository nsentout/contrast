use pointmark::PointMark;
use pointmark::VertexPoint;
use linemark::LineMark;

// Main structure
pub struct Contrast {
    point_marks: Vec<PointMark>,
    line_marks: Vec<LineMark>
}

impl Contrast {
    pub fn init() -> Self { 
        Contrast {
            point_marks : Vec::<PointMark>::new(),
            line_marks : Vec::<LineMark>::new()
        }
    }

    // Returns a vector of mark properties understandable by luminance
    pub fn get_pointmarks_properties(self) -> Vec<VertexPoint> {    // TODO: éviter cette copie
        let mut properties : Vec<VertexPoint> = Vec::<VertexPoint>::new();
        for pt in &self.point_marks {
            properties.push(pt.as_vertex());
        }
        properties
    }

    pub fn add_point_mark(&mut self) -> &mut PointMark {
        // Create a PointMark
        let mut point = PointMark::default();
        point.common_properties.id = self.point_marks.len();

        // Add it into the PointMark vector
        self.point_marks.push(point);

        // Returns a mutable reference of this PointMark
        self.point_marks.last_mut().unwrap()
    }

    pub fn remove_point_mark(&mut self, mark: usize)
    {
        if self.point_marks.len() > mark { self.point_marks.swap_remove(mark); }
        if !self.point_marks.is_empty() { self.point_marks.last_mut().unwrap().common_properties.id = mark; }
    }

    pub fn add_line_mark(&mut self) -> &mut LineMark {
        // Create a LineMark
        let mut line = LineMark::default();
        line.common_properties.id = self.line_marks.len();

        // Add it into the LineMark vector
        self.line_marks.push(line);

        // Return a mutable reference of this LineMark
        self.line_marks.last_mut().unwrap()
    }

    pub fn remove_line_mark(&mut self, mark: usize)
    {
        if self.line_marks.len() > mark { self.line_marks.swap_remove(mark); }
        if !self.line_marks.is_empty() { self.line_marks.last_mut().unwrap().common_properties.id = mark; }
    }
}