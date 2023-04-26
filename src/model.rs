use nalgebra_glm::{Mat4x4, Vec3};

#[allow(unused)]
#[derive(Debug, Clone, Default)]
pub struct Model {
    pub transform: Transform,
    pub mesh: Vec<Vec3>,
}

#[allow(unused)]
impl Model {
    pub fn triangle() -> Self {
        Self {
            transform: Default::default(),
            mesh: vec![
                Vec3::new(-0.5, -0.5, 0.0),
                Vec3::new(0.5, -0.5, 0.0),
                Vec3::new(0.0, 0.5, 0.0),
            ],
        }
    }
    pub fn square() -> Self {
        Self {
            transform: Default::default(),
            mesh: vec![
                Vec3::new(-0.5, 0.5, 0.),
                Vec3::new(0.5, 0.5, 0.),
                Vec3::new(-0.5, -0.5, 0.),
                Vec3::new(0.5, 0.5, 0.),
                Vec3::new(0.5, -0.5, 0.),
                Vec3::new(-0.5, -0.5, 0.),
            ],
        }
    }
}

#[allow(unused)]
#[derive(Debug, Clone)]
pub struct Transform {
    pub translation: Vec3,
    pub scale: Vec3,
    pub rotation: Vec3,
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            translation: Default::default(),
            scale: Vec3::repeat(1.),
            rotation: Default::default(),
        }
    }
}

impl Transform {
    /// Model View Matrix
    /// Matrix corresponds to Translate * Ry * Rx * Rz * Scale
    /// Rotations correspond to Tait-bryan angles of Y(1), X(2), Z(3)
    pub fn model_view(&self) -> Mat4x4 {
        let (s3, c3) = self.rotation.z.sin_cos();
        let (s2, c2) = self.rotation.x.sin_cos();
        let (s1, c1) = self.rotation.y.sin_cos();

        Mat4x4::new(
            self.scale.x * (c1 * c3 + s1 * s2 * s3),
            self.scale.x * (c2 * s3),
            self.scale.x * (c1 * s2 * s3 - c3 * s1),
            0.0,
            self.scale.y * (c3 * s1 * s2 - c1 * s3),
            self.scale.y * (c2 * c3),
            self.scale.y * (c1 * c3 * s2 + s1 * s3),
            0.0,
            self.scale.z * (c2 * s1),
            self.scale.z * (-s2),
            self.scale.z * (c1 * c2),
            0.0,
            self.translation.x,
            self.translation.y,
            self.translation.z,
            1.0,
        )
    }
}
