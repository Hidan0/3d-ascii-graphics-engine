use std::f32::consts::FRAC_PI_4;

use nalgebra_glm::Vec3;
use nalgebra_glm::Vec4;

use crate::camera::Camera;
use crate::model::{GameObject, Model};
use crate::render::rasterize;

pub struct App {
    pub screen_width: usize,
    pub screen_height: usize,
    offset_x: f32,
    offset_y: f32,
    aspect_ratio: f32,
}

impl App {
    pub fn new(screen_width: usize, screen_height: usize) -> Self {
        Self {
            screen_width,
            screen_height,
            offset_x: screen_width as f32 * 0.5,
            offset_y: screen_height as f32 * 0.5,
            aspect_ratio: screen_height as f32 / screen_width as f32,
        }
    }

    pub fn run(&self) {
        let camera = Camera::new_perspective_camera(FRAC_PI_4, self.aspect_ratio, 0.1, 10.);

        let mut obj = GameObject::new(Model::square());

        obj.transform.scale = Vec3::new(4., 4., 1.);

        for _ in 0..500 {
            let mut frame_buffer = vec![vec![b' '; self.screen_width]; self.screen_height];

            let mut screen_space = vec![[0.0_f32, 0.0_f32]; obj.model.len()];
            for (v, s) in obj.into_iter().zip(screen_space.iter_mut()) {
                let pos = camera.projection_matrix
                    * obj.transform.model_view()
                    * Vec4::new(v.x, v.y, v.z, 1.);

                let ooz = 1. / pos.z;

                *s = [pos.x * ooz + self.offset_x, pos.y * ooz + self.offset_y];
            }

            rasterize(&screen_space, &mut frame_buffer);

            (0..self.screen_height).for_each(|l| {
                let row = std::str::from_utf8(&frame_buffer[l]).unwrap();
                println!("{}", row);
            });

            print!("\x1b[{}A;", self.screen_height);
            std::thread::sleep(std::time::Duration::from_millis(30));

            obj.transform.rotation.z += 0.8_f32.to_radians();
        }
    }
}
