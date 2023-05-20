use nalgebra_glm::Vec4;

pub mod pipeline;
mod rasterizer;
pub mod scene;

pub trait Geometry {
    fn verteces(&self) -> Vec<Vec4>;
    fn indeces(&self) -> &Vec<usize>;
}

#[derive(Debug)]
struct Triangle {
    v0: Vec4,
    v1: Vec4,
    v2: Vec4,
}
