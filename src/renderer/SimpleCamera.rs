use nalgebra::{Matrix4, Perspective3, Point3, Vector3};

pub struct SimpleCamera {
    pub position: Point3<f32>,
    pub direction: Vector3<f32>,
    pub up: Vector3<f32>,
    pub aspect_ratio: f32,
    pub fovy: f32,
    pub znear: f32,
    pub zfar: f32,
    pub view_projection_matrix: Matrix4<f32>,
}

impl SimpleCamera {
    pub fn new(
        position: Point3<f32>,
        direction: Vector3<f32>,
        up: Vector3<f32>,
        // aspect_ratio: f32,
        fovy: f32,
        znear: f32,
        zfar: f32,
    ) -> Self {
        Self {
            position,
            direction,
            up,
            // aspect_ratio,
            aspect_ratio: 16.0 / 9.0, // default aspect ratio
            fovy,
            znear,
            zfar,
            view_projection_matrix: Matrix4::identity(),
        }
    }

    pub fn update_aspect_ratio(&mut self, aspect_ratio: f32) {
        self.aspect_ratio = aspect_ratio;
    }

    // pub fn build_view_projection_matrix(&self) -> Matrix4<f32> {
    //     let view = Matrix4::look_at_rh(&self.position, &(self.position + self.direction), &self.up);
    //     let proj =
    //         Perspective3::new(self.aspect_ratio, self.fovy, self.znear, self.zfar).to_homogeneous();
    //     proj * view
    // }

    pub fn update_view_projection_matrix(&mut self) {
        let view_matrix =
            Matrix4::look_at_rh(&self.position, &(self.position + self.direction), &self.up);
        let projection_matrix =
            Matrix4::new_perspective(self.aspect_ratio, self.fovy, self.znear, self.zfar);
        self.view_projection_matrix = projection_matrix * view_matrix;
    }

    pub fn update(&mut self) {
        self.update_view_projection_matrix();
    }
}
