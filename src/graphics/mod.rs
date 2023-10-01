use nalgebra_glm::{Vec2, Vec4};

pub mod pipeline;
mod rasterizer;
pub mod scene;

pub trait Geometry {
    fn verteces(&self) -> Vec<Vec4>;
    fn indeces(&self) -> &Vec<usize>;
}

#[derive(Debug, Clone)]
struct Triangle {
    v0: Vec4,
    v1: Vec4,
    v2: Vec4,
}

pub struct ScreenSpaceTriangle {
    v0: Vec2,
    v1: Vec2,
    v2: Vec2,
}

impl Triangle {
    fn to_screen_space_triangle(&self, offset_x: f32, offset_y: f32) -> ScreenSpaceTriangle {
        let ooz0: f32 = 1. / self.v0.z;
        let ooz1: f32 = 1. / self.v1.z;
        let ooz2: f32 = 1. / self.v2.z;

        ScreenSpaceTriangle {
            v0: Vec2::new(self.v0.x * ooz0 + offset_x, self.v0.y * ooz0 + offset_y),
            v1: Vec2::new(self.v1.x * ooz1 + offset_x, self.v1.y * ooz1 + offset_y),
            v2: Vec2::new(self.v2.x * ooz2 + offset_x, self.v2.y * ooz2 + offset_y),
        }
    }

    fn is_front_facing(&self) -> bool {
        ((self.v1.xyz() - self.v0.xyz()).cross(&(self.v2.xyz() - self.v0.xyz())))
            .dot(&self.v0.xyz())
            >= 0.
    }
}
