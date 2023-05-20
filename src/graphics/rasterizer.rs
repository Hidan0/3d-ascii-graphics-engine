use nalgebra_glm::Vec2;

pub fn rasterize(screen_space: &[[f32; 2]], frame_buffer: &mut Vec<Vec<u8>>) {
    let screen_width = frame_buffer.len();
    let screen_height = frame_buffer[0].len();

    for triangle in screen_space.chunks(3) {
        let (mut min_x, mut min_y): (f32, f32) = (screen_width as f32, screen_height as f32);
        let (mut max_x, mut max_y): (f32, f32) =
            (screen_width as f32 * -1., screen_height as f32 * -1.);

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
