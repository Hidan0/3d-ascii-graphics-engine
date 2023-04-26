use std::f32::consts::PI;

use model::Model;
use nalgebra_glm::{Mat4x4, Vec3, Vec4};

mod model;

const SCREEN_WIDTH: usize = 80;
const SCREEN_HEIGHT: usize = 32;
const ASPECT_RATIO: f32 = SCREEN_HEIGHT as f32 / SCREEN_WIDTH as f32;
const FOV: f32 = PI / 4.;

const NEAR: f32 = 0.1;
const FAR: f32 = 10.;

const OFFSET_X: f32 = SCREEN_WIDTH as f32 * 0.5;
const OFFSET_Y: f32 = SCREEN_HEIGHT as f32 * 0.5;

fn main() {
    let mut obj = Model::triangle();

    obj.transform.translation = Vec3::new(0., 0., 1.);
    obj.transform.scale = Vec3::new(0.5, 0.5, 0.5);

    let tan_fov = (FOV * 0.5).tan();
    let proj_m = Mat4x4::new(
        1. / (ASPECT_RATIO * tan_fov),
        0.,
        0.,
        0.,
        0.,
        1. / tan_fov,
        0.,
        0.,
        0.,
        0.,
        FAR / (FAR - NEAR),
        (-FAR * NEAR) / (FAR - NEAR),
        0.,
        0.,
        1.,
        0.,
    );

    for _ in 0..500 {
        let mut frame_buffer = [[b' '; SCREEN_WIDTH]; SCREEN_HEIGHT];

        for v in &obj.mesh {
            let pos = proj_m * obj.transform.model_view() * Vec4::new(v.x, v.y, v.z, 1.);

            let ooz = 1. / pos.z;

            let (s_x, s_y) = (pos.x * ooz + OFFSET_X, pos.y * ooz + OFFSET_Y);

            if s_x >= 0. && s_x < SCREEN_WIDTH as f32 && s_y >= 0. && s_y < SCREEN_HEIGHT as f32 {
                frame_buffer[s_y as usize][s_x as usize] = b'#';
            }
        }

        (0..SCREEN_HEIGHT).for_each(|l| {
            let row = std::str::from_utf8(&frame_buffer[l]).unwrap();
            println!("{}", row);
        });

        print!("\x1b[{}A;", SCREEN_HEIGHT);
        std::thread::sleep(std::time::Duration::from_millis(30));

        obj.transform.rotation.z += 0.8_f32.to_radians();
    }
}
