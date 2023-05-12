#![allow(dead_code)]

use nalgebra_glm::Vec3;
pub struct Pipeline;

impl Pipeline {
    pub fn draw(&self) {
        Self::transform_vertices();
        Self::triangle_assembler();
        Self::post_process_triangles();
        Self::triangle_resterizer();
    }

    fn transform_vertices() {
        todo!()
    }
    fn triangle_assembler() {
        todo!()
    }
    fn post_process_triangles() {
        todo!()
    }
    fn triangle_resterizer() {
        todo!()
    }
}

struct Triangle {
    v0: Vec3,
    v1: Vec3,
    v2: Vec3,
}
