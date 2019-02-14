use pointmark::PointMark;
use pointmark::VertexPoint;
use linemark::LineMark;

/*  
 *   This is the main structure of the library. It contains all the marks
 *   displayed on screen. The user can add and remove marks as he wishes
 *   but he can't modify the marks (at the moment).
 */
pub struct Contrast {
    point_marks: Vec<PointMark>,
    line_marks: Vec<LineMark>
}

impl Contrast {
    /*
     *   Simply returns a new instance of Contrast, initializing
     *   the vectors containing the marks.
     */
    pub fn new() -> Self { 
        Contrast {
            point_marks : Vec::<PointMark>::new(),
            line_marks : Vec::<LineMark>::new()
        }
    }

    /*
     *   Create a mark of type Point with default values and add it into the vector
     *   containing all the PointMark, then returns a mutable reference of this
     *   newly created mark, all of this in O(1). We return a mutable reference because we 
     *   want to be able to modify it just after calling add_point_mark in a way
     *   similar to this : add_point_mark.set_rotation(90.0).
     */
    pub fn add_point_mark(&mut self) -> &mut PointMark {
        let point = PointMark::new(self.point_marks.len());
        self.point_marks.push(point);
        self.point_marks.last_mut().unwrap()
    }

    /*
     *   Remove the point mark with the id <mark>. We will call this mark the target.
     *   We first set the id of the last element of the vector containing all 
     *   the PointMark to the target's id (<mark>).
     *   We then swap the target with the last element. We can now safely remove the target.
     *   This way, the mark that was the last element before the removal holds now the id
     *   of the target. This explains why we can always use "self.point_marks.len()" when
     *   we want to give a unique id to a new mark. Furthermore, this allows us to remove
     *   an element in O(1).
     */
    pub fn remove_point_mark(&mut self, mark: usize)
    {
        if !self.point_marks.is_empty() { self.point_marks.last_mut().unwrap().common_properties.id = mark; }
        if self.point_marks.len() > mark { self.point_marks.swap_remove(mark); }
    }

    /*  
     *  Convert the vector of MarkPoint to a vector of vertices
     *  understandable by the renderer, then returns it.
     */
    pub fn get_pointmarks_properties(self) -> Vec<VertexPoint> {    // TODO: éviter cette copie
        let mut properties : Vec<VertexPoint> = Vec::<VertexPoint>::new();
        for pt in &self.point_marks {
            properties.push(pt.as_vertex());
        }
        properties
    }

    /*
     *  Same behavior than add_point_mark but it adds a mark of type Line.
     */
    pub fn add_line_mark(&mut self) -> &mut LineMark {
        let line = LineMark::new(self.line_marks.len());
        self.line_marks.push(line);
        self.line_marks.last_mut().unwrap()
    }

    /*
     *  Same behavior than remove_point_mark but it removes a mark of type Line.
     */
    pub fn remove_line_mark(&mut self, mark: usize)
    {
        if !self.line_marks.is_empty() { self.line_marks.last_mut().unwrap().common_properties.id = mark; }
        if self.line_marks.len() > mark { self.line_marks.swap_remove(mark); }
    }
}

#[test]
fn new()
{
    assert_eq!(Contrast::new().get_pointmarks_properties().len(), 0);
}

#[test]
fn add_point_mark()
{
    let mut c = Contrast::new();

    let p = { c.add_point_mark().get_id() };
    let s = { c.get_pointmarks_properties().len() };
    assert_eq!(p, 0);
    assert_eq!(s, 1);
}

#[test]
fn remove_point_mark()
{
    let mut c = Contrast::new();
    let p1 = { c.add_point_mark().get_id() };
    { c.add_point_mark().get_id() };

    c.remove_point_mark(p1);
    assert_eq!(c.get_pointmarks_properties().len(), 1);
}