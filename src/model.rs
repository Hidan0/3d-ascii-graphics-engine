use nalgebra_glm::{Mat4x4, Vec3};

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

impl<'a> IntoIterator for &'a GameObject {
    type Item = &'a Vec3;

    type IntoIter = ModelIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        ModelIterator {
            vertex_buffer: &self.model.vertex_buffer,
            index_buffer: &self.model.index_buffer,
            current_index: 0,
        }
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
                Vec3::new(0., 0.5, 0.),
                Vec3::new(0.5, -0.5, 0.),
                Vec3::new(-0.5, -0.5, 0.),
            ],
            index_buffer: vec![0, 1, 2],
        }
    }

    pub fn square() -> Self {
        Self {
            vertex_buffer: vec![
                Vec3::new(-0.5, 0.5, 0.),
                Vec3::new(0.5, 0.5, 0.),
                Vec3::new(-0.5, -0.5, 0.),
                Vec3::new(0.5, -0.5, 0.),
            ],
            index_buffer: vec![0, 1, 2, 1, 3, 2],
        }
    }

    pub fn len(&self) -> usize {
        self.index_buffer.len()
    }
}

pub struct ModelIterator<'a> {
    vertex_buffer: &'a Vec<Vec3>,
    index_buffer: &'a Vec<usize>,
    current_index: usize,
}

impl<'a> Iterator for ModelIterator<'a> {
    type Item = &'a Vec3;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_index < self.index_buffer.len() {
            let index = self.index_buffer[self.current_index];
            self.current_index += 1;
            return Some(&self.vertex_buffer[index]);
        }
        None
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
