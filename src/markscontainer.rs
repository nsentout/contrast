use pointmark::PointMark;
use pointmark::VertexPoint;
use linemark::LineMark;

// Main structure
pub struct Contrast {
    point_marks: Vec<PointMark>,
    line_marks: Vec<LineMark>,
    cpt_id : u32,
}

impl Contrast {
    pub fn init() -> Self { 
        Contrast {
            point_marks : Vec::<PointMark>::new(),
            line_marks: Vec::<LineMark>::new(),
            cpt_id : 0,
        }
    }

    pub fn get_pointmarks_properties(self) -> Vec<VertexPoint> {    // TODO: éviter cette copie
        let mut properties : Vec<VertexPoint> = Vec::<VertexPoint>::new();
        for pt in &self.point_marks {
            properties.push((pt.common_properties.center.as_array(), pt.common_properties.size.as_array(),
                            pt.common_properties.color.as_array(), pt.common_properties.rotation,
                            pt.shape as u32, pt.selection_angle, pt.start_radius));
        }
        properties
    }

    fn get_point_mark_mut(&mut self, mark : PointMark) -> Option<&mut PointMark> {
        self.point_marks.iter_mut().find(|x| **x == mark)   // TODO: faire en O(1)
    }

    pub fn add_point_mark(&mut self) -> &mut PointMark {
        // Create a PointMark
        let mut point = PointMark::default();
        point.common_properties.id = self.cpt_id;
        self.cpt_id += 1;

        // Add it into the PointMark vector
        self.point_marks.push(point);

        // Return a mutable reference of this PointMark
        self.get_point_mark_mut(point).unwrap()
    }

    fn get_line_mark_mut(&mut self, mark : LineMark) -> Option<&mut LineMark> {
        self.line_marks.iter_mut().find(|x| **x == mark)
    }

    pub fn add_line_mark(&mut self) -> &mut LineMark {
        // Create LineMark
        let mut line = LineMark::default();
        line.common_properties.id = self.cpt_id;
        self.cpt_id += 1;

        // Add it into the LineMark vector
        self.line_marks.push(line);

        // Return a mutable reference of this LineMark
        self.get_line_mark_mut(line).unwrap()
    }
}