use pointmark::PointMark;
use pointmark::VertexPoint;
use linemark::LineMark;

// Main structure
pub struct Contrast {
    point_marks: Vec<PointMark>,
    line_marks: Vec<LineMark>,
    cpt_point_id : u32,
    cpt_line_id : u32
}

impl Contrast {
    pub fn init() -> Self { 
        Contrast {
            point_marks : Vec::<PointMark>::new(),
            line_marks : Vec::<LineMark>::new(),
            cpt_point_id : 0,
            cpt_line_id : 0
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
        point.common_properties.id = self.cpt_point_id;
        self.cpt_point_id += 1;

        // Add it into the PointMark vector
        self.point_marks.push(point);

        // Returns a mutable reference of this PointMark
        self.point_marks.get_mut((self.cpt_point_id - 1) as usize).unwrap()
    }

    pub fn add_line_mark(&mut self) -> &mut LineMark {
        // Create a LineMark
        let mut line = LineMark::default();
        line.common_properties.id = self.cpt_line_id;
        self.cpt_line_id += 1;

        // Add it into the LineMark vector
        self.line_marks.push(line);

        // Return a mutable reference of this LineMark
        self.line_marks.get_mut((self.cpt_line_id - 1) as usize).unwrap()
    }
}