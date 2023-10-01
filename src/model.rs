use nalgebra_glm::{Mat4x4, Vec3, Vec4};

use crate::graphics::Geometry;

#[derive(Debug)]
pub struct GameObject {
    pub transform: Transform,
    pub model: Model,
}

impl GameObject {
    pub fn new(model: Model) -> Self {
        Self {
            transform: Default::default(),
            model,
        }
    }
}

impl Geometry for GameObject {
    fn verteces(&self) -> Vec<Vec4> {
        self.model
            .vertex_buffer
            .iter()
            .map(|v| self.transform.model_view() * Vec4::new(v.x, v.y, v.z, 1.))
            .collect()
    }

    fn indeces(&self) -> &Vec<usize> {
        &self.model.index_buffer
    }
}

#[derive(Debug)]
pub struct Model {
    vertex_buffer: Vec<Vec3>,
    index_buffer: Vec<usize>,
}

impl Model {
    pub fn triangle() -> Self {
        Self {
            vertex_buffer: vec![
                Vec3::new(0., 0.5, 1.),
                Vec3::new(0.5, -0.5, 1.),
                Vec3::new(-0.5, -0.5, 1.),
            ],
            index_buffer: vec![0, 1, 2],
        }
    }

    pub fn square() -> Self {
        Self {
            vertex_buffer: vec![
                Vec3::new(-0.5, 0.5, 1.),
                Vec3::new(0.5, 0.5, 1.),
                Vec3::new(-0.5, -0.5, 1.),
                Vec3::new(0.5, -0.5, 1.),
            ],
            index_buffer: vec![0, 1, 2, 1, 3, 2],
        }
    }

    pub fn cube() -> Self {
        Self {
            vertex_buffer: vec![
                Vec3::new(-0.5, -0.5, -0.5),
                Vec3::new(0.5, -0.5, -0.5),
                Vec3::new(-0.5, 0.5, -0.5),
                Vec3::new(0.5, 0.5, -0.5),
                Vec3::new(-0.5, -0.5, 0.5),
                Vec3::new(0.5, -0.5, 0.5),
                Vec3::new(-0.5, 0.5, 0.5),
                Vec3::new(0.5, 0.5, 0.5),
            ],
            index_buffer: vec![
                0, 2, 1, 2, 3, 1, 1, 3, 5, 3, 7, 5, 2, 6, 3, 3, 6, 7, 4, 5, 7, 4, 7, 6, 0, 4, 2, 2,
                4, 6, 0, 1, 4, 1, 5, 4,
            ],
        }
    }
}

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
