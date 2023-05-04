use nalgebra_glm::Mat4x4;

pub struct Camera {
    pub projection_matrix: Mat4x4,
}

impl Camera {
    pub fn new_perspective_camera(fovy: f32, aspect_ratio: f32, far: f32, near: f32) -> Self {
        let tan_half_fov = (fovy * 0.5).tan();

        let mut projection_matrix = Mat4x4::zeros();
        projection_matrix[(0, 0)] = 1. / (aspect_ratio * tan_half_fov);
        projection_matrix[(1, 1)] = 1. / tan_half_fov;
        projection_matrix[(2, 2)] = far / (far - near);
        projection_matrix[(2, 3)] = 1.;
        projection_matrix[(3, 2)] = -(far * near) / (far - near);

        Self { projection_matrix }
    }
}
