use super::rasterizer::draw_triangle;
use super::scene::Scene;
use super::Geometry;
use crate::graphics::Triangle;

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
        let mut pre_triangles = self.triangle_assembler(scene);
        let triangles = self.post_process_triangles(scene, &mut pre_triangles);

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

    fn post_process_triangles(
        &self,
        scene: &dyn Scene,
        triangles: &mut [Triangle],
    ) -> Vec<Triangle> {
        self.triangles_to_view_space(scene, triangles);
        let cull_flags = self.backface_culling(triangles);

        assert!(triangles.len() == cull_flags.len());

        let mut out = Vec::new();
        for (i, _) in triangles.iter().enumerate() {
            if cull_flags[i] {
                out.push(triangles[i].clone());
            }
        }

        out
    }

    fn triangles_to_view_space(&self, scene: &dyn Scene, triangles: &mut [Triangle]) {
        triangles.iter_mut().for_each(|t| {
            t.v0 = scene.camera().projection_matrix * t.v0;
            t.v1 = scene.camera().projection_matrix * t.v1;
            t.v2 = scene.camera().projection_matrix * t.v2;
        });
    }

    fn backface_culling(&self, triangles: &[Triangle]) -> Vec<bool> {
        triangles.iter().map(|t| t.is_front_facing()).collect()
    }

    fn triangle_resterizer(&self, triangles: Vec<Triangle>) -> Vec<Vec<u8>> {
        let mut frame_buffer = vec![vec![b' '; self.screen_width]; self.screen_height];

        for (i, triangle) in triangles.into_iter().enumerate() {
            let screen_space_triangle =
                triangle.to_screen_space_triangle(self.offset_x, self.offset_y);

            draw_triangle(&mut frame_buffer, &screen_space_triangle, i);
        }

        frame_buffer
    }
}
