use na::Matrix4;

#[derive(Copy, Clone)]
pub struct Camera
{
    mat: Matrix4<f32>
}

impl Camera
{
    pub fn init(width: u32, height: u32) -> Self
    {
        Camera{mat: Matrix4::new_orthographic(0.0, width as f32, height as f32, 0.0, -1.0, 100.0)}
    }

    pub fn data(self) -> [[f32; 4]; 4]
    {
        self.mat.into()
    }

    pub fn resize(&mut self, width: i32, height: i32)
    {
        self.mat = Matrix4::new_orthographic(0.0, width as f32, height as f32, 0.0, -1.0, 100.0);
    }
}