use crate::graphics::Triangle;

use super::rasterizer::rasterize;
use super::scene::Scene;
use super::Geometry;

pub struct Pipeline {
    pub screen_width: usize,
    pub screen_height: usize,
    offset_x: f32,
    offset_y: f32,
    pub aspect_ratio: f32,
}

impl Pipeline {
    pub fn new(screen_width: usize, screen_height: usize) -> Self {
        Self {
            screen_width,
            screen_height,
            offset_x: screen_width as f32 * 0.5,
            offset_y: screen_height as f32 * 0.5,
            aspect_ratio: screen_height as f32 / screen_width as f32,
        }
    }
    pub fn draw(&self, scene: &dyn Scene) {
        let triangles = self.triangle_assembler(scene);
        let triangles = self.post_process_triangles(scene, triangles);

        let frame_buffer = self.triangle_resterizer(triangles);

        (0..self.screen_height).for_each(|l| {
            let row = std::str::from_utf8(&frame_buffer[l]).unwrap();
            println!("{}", row);
        });

        print!("\x1b[{}A;", self.screen_height);
        std::thread::sleep(std::time::Duration::from_millis(30));
    }

    fn triangle_assembler(&self, scene: &dyn Scene) -> Vec<Triangle> {
        let mut triangles: Vec<Triangle> = Vec::new();

        for obj in scene.game_objects() {
            let vtx = obj.verteces();

            if vtx.len() < 2 {
                panic!("Not enough verteces");
            }

            if obj.indeces().len() % 3 != 0 {
                panic!("Not enough indeces");
            }

            for i in obj.indeces().chunks(3) {
                triangles.push(Triangle {
                    v0: vtx[i[0]],
                    v1: vtx[i[1]],
                    v2: vtx[i[2]],
                })
            }
        }

        triangles
    }

    fn post_process_triangles(&self, scene: &dyn Scene, triangles: Vec<Triangle>) -> Vec<Triangle> {
        triangles
            .iter()
            .map(|t| Triangle {
                v0: scene.camera().projection_matrix * t.v0,
                v1: scene.camera().projection_matrix * t.v1,
                v2: scene.camera().projection_matrix * t.v2,
            })
            .collect()
    }

    fn triangle_resterizer(&self, triangles: Vec<Triangle>) -> Vec<Vec<u8>> {
        let mut frame_buffer = vec![vec![b' '; self.screen_width]; self.screen_height];

        let mut screen_space = vec![[0.0_f32, 0.0_f32]; triangles.len() * 3];

        let mut screen_i = 0;
        for triangle in triangles {
            let ooz0 = 1. / triangle.v0.z;
            let ooz1 = 1. / triangle.v1.z;
            let ooz2 = 1. / triangle.v2.z;

            screen_space[screen_i] = [
                triangle.v0.x * ooz0 + self.offset_x,
                triangle.v0.y * ooz0 + self.offset_y,
            ];
            screen_space[screen_i + 1] = [
                triangle.v1.x * ooz1 + self.offset_x,
                triangle.v1.y * ooz1 + self.offset_y,
            ];
            screen_space[screen_i + 2] = [
                triangle.v2.x * ooz2 + self.offset_x,
                triangle.v2.y * ooz2 + self.offset_y,
            ];
            screen_i += 3;
        }

        rasterize(&screen_space, &mut frame_buffer);

        frame_buffer
    }
}
