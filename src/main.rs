use std::f32::consts::PI;

use model::GameObject;
use nalgebra_glm::{Mat4x4, Vec2, Vec3, Vec4};

use self::model::Model;

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
    let mut obj = GameObject::new(Model::square());

    obj.transform.translation = Vec3::new(0., 0., 5.);
    obj.transform.scale = Vec3::new(0.65, 0.65, 1.);

    let tan_fov = (FOV * 0.5).tan();
    let pp_m = Mat4x4::new(
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

        let mut screen_space = vec![[0.0_f32, 0.0_f32]; obj.model.len()];
        for (v, s) in obj.into_iter().zip(screen_space.iter_mut()) {
            let pos = pp_m * obj.transform.model_view() * Vec4::new(v.x, v.y, v.z, 1.);

            let ooz = 1. / pos.z;

            *s = [pos.x * ooz + OFFSET_X, pos.y * ooz + OFFSET_Y];
        }

        rasterize(&screen_space, &mut frame_buffer);

        (0..SCREEN_HEIGHT).for_each(|l| {
            let row = std::str::from_utf8(&frame_buffer[l]).unwrap();
            println!("{}", row);
        });

        print!("\x1b[{}A;", SCREEN_HEIGHT);
        std::thread::sleep(std::time::Duration::from_millis(30));

        obj.transform.rotation.z += 0.8_f32.to_radians();
    }
}

fn rasterize(screen_space: &[[f32; 2]], frame_buffer: &mut [[u8; SCREEN_WIDTH]; SCREEN_HEIGHT]) {
    if screen_space.len() % 3 != 0 {
        panic!("Not enough verts");
    }

    for triangle in screen_space.chunks(3) {
        let (mut min_x, mut min_y): (f32, f32) = (SCREEN_WIDTH as f32, SCREEN_HEIGHT as f32);
        let (mut max_x, mut max_y): (f32, f32) =
            (SCREEN_WIDTH as f32 * -1., SCREEN_HEIGHT as f32 * -1.);

        for v in triangle {
            let [x, y] = *v;
            if x < min_x {
                min_x = x;
            }
            if x > max_x {
                max_x = x;
            }

            if y < min_y {
                min_y = y;
            }
            if y > max_y {
                max_y = y;
            }
        }

        (min_y as usize..=max_y as usize).for_each(|y| {
            for x in min_x as usize..=max_x as usize {
                let p = Vec2::new(x as f32 + 0.5, y as f32 + 0.5);

                if is_inside_triangle(
                    Vec2::new(triangle[0][0], triangle[0][1]),
                    Vec2::new(triangle[1][0], triangle[1][1]),
                    Vec2::new(triangle[2][0], triangle[2][1]),
                    p,
                ) {
                    frame_buffer[y][x] = b'.';
                }
            }
        });
    }
}

fn is_inside_triangle(a: Vec2, b: Vec2, c: Vec2, p: Vec2) -> bool {
    let s1 = c.y - a.y;
    let s2 = c.x - a.x;
    let s3 = b.y - a.y;
    let s4 = p.y - a.y;

    let w1 = (a.x * s1 + s4 * s2 - p.x * s1) / (s3 * s2 - (b.x - a.x) * s1);
    let w2 = (s4 - w1 * s3) / s1;

    w1 >= 0. && w2 >= 0. && w1 + w2 <= 1.
}
