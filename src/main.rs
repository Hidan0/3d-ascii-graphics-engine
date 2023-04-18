use std::f32::consts::PI;

#[allow(unused)]

const SCREEN_WIDTH: usize = 80;
const SCREEN_HEIGHT: usize = 22;
const ASPECT_RATIO: f32 = SCREEN_WIDTH as f32 / SCREEN_HEIGHT as f32;
const FOV: f32 = PI / 4.;

const NEAR: f32 = 0.1;
const FAR: f32 = 100.;

const OFFSET_X: f32 = 1.0;
const OFFSET_Y: f32 = 1.0;

#[derive(Debug, Clone, Copy)]
struct Mat4x4([[f32; 4]; 4]);

#[derive(Debug, Clone, Copy)]
struct Vec3([f32; 3]);

impl Vec3 {
    fn x(&self) -> f32 {
        self.0[0]
    }
    fn y(&self) -> f32 {
        self.0[1]
    }
    fn z(&self) -> f32 {
        self.0[2]
    }

    fn times_mat4x4(&self, m: &Mat4x4) -> Self {
        let [mx, my, mz, mw] = m.0;

        let x = self.x() * mx[0] + self.y() * my[0] + self.z() * mz[0] + mw[0];
        let y = self.x() * mx[1] + self.y() * my[1] + self.z() * mz[1] + mw[1];
        let z = self.x() * mx[2] + self.y() * my[2] + self.z() * mz[2] + mw[2];

        let w = self.x() * mx[3] + self.y() * my[3] + self.z() * mz[3] + mw[3];

        if w != 0.0 {
            return Self([x / w, y / w, z / w]);
        }

        Self([x, y, z])
    }

    fn add(&self, o: &Vec3) -> Self {
        Self([self.x() + o.x(), self.y() + o.y(), self.z() + o.z()])
    }
}

// const TRIANGLE: [Vec3; 3] = [
//     Vec3([0.5, 0.5, 1.]),
//     Vec3([0., 0.5, 1.]),
//     Vec3([-0.5, 0.5, 1.]),
// ];

const TRIANGLE: [Vec3; 3] = [
    Vec3([0.0, -0.5, 1.0]),
    Vec3([0.5, 0.5, 1.0]),
    Vec3([-0.5, 0.5, 1.0]),
];

fn main() {
    let tan_fov = (FOV / 2.).tan();
    let f_minus_n = FAR - NEAR;

    let m_proj = Mat4x4([
        [1. / ASPECT_RATIO * tan_fov, 0., 0., 0.],
        [0.0, 1. / tan_fov, 0.0, 0.0],
        [0., 0., FAR / f_minus_n, 1.],
        [0.0, 0.0, (-FAR * NEAR) / f_minus_n, 1.0],
    ]);

    let mut frame_buffer = [[b' '; SCREEN_WIDTH]; SCREEN_HEIGHT];

    let (scale_x, scale_y) = (0.5 * SCREEN_WIDTH as f32, 0.5 * SCREEN_HEIGHT as f32);

    let translate = Vec3([3., 3., 0.]);

    for _ in 0..1 {
        let mut screen_pos = [[0.0, 0.0]; TRIANGLE.len()]; // for each vert

        for (v, _) in TRIANGLE.iter().zip(screen_pos.iter_mut()) {
            let w_pos = v.add(&translate).times_mat4x4(&m_proj);

            let (screen_x, screen_y) = (
                w_pos.x() * scale_x + OFFSET_X,
                w_pos.y() * scale_y + OFFSET_Y,
            );

            frame_buffer[screen_x as usize][screen_y as usize] = b'.';
        }

        (0..SCREEN_HEIGHT).for_each(|l| {
            let row = std::str::from_utf8(&frame_buffer[l]).unwrap();
            println!("{}", row);
        });

        // print!("\x1b[{}A;", SCREEN_HEIGHT);
        std::thread::sleep(std::time::Duration::from_millis(30));
    }
}
