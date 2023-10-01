use super::ScreenSpaceTriangle;
use nalgebra_glm::Vec2;
use std::mem;

const COLORS: [u8; 6] = [b'X', b'A', b'@', b'#', b'$', b'%'];

pub fn draw_triangle(frame_buffer: &mut [Vec<u8>], triangle: &ScreenSpaceTriangle, i: usize) {
    let mut pv0 = &triangle.v0;
    let mut pv1 = &triangle.v1;
    let mut pv2 = &triangle.v2;

    // sorting vertices by y
    if pv1.y < pv0.y {
        mem::swap(&mut pv0, &mut pv1);
    }

    if pv2.y < pv1.y {
        mem::swap(&mut pv2, &mut pv1);
    }

    if pv1.y < pv0.y {
        mem::swap(&mut pv1, &mut pv0);
    }

    // checking triangle type
    if pv0.y == pv1.y {
        // natural flat top
        if pv1.x < pv0.x {
            mem::swap(&mut pv0, &mut pv1);
        }
        draw_flat_top_triangle(frame_buffer, *pv0, *pv1, *pv2, COLORS[i % COLORS.len()]);
    } else if pv1.y == pv2.y {
        // natural flat bottom

        if pv2.x < pv1.x {
            mem::swap(&mut pv2, &mut pv1);
        }
        draw_flat_bottom_triangle(frame_buffer, *pv0, *pv1, *pv2, COLORS[i % COLORS.len()]);
    } else {
        // general triangle

        // find splitting vertex
        let alpha_split = (pv1.y - pv0.y) / (pv2.y - pv0.y);
        let v_inter = Vec2::new(
            pv0.x + (pv2.x - pv0.x) * alpha_split,
            pv0.y + (pv2.y - pv0.y) * alpha_split,
        );

        if pv1.x < v_inter.x {
            // major right
            draw_flat_bottom_triangle(frame_buffer, *pv0, *pv1, v_inter, COLORS[i % COLORS.len()]);
            draw_flat_top_triangle(frame_buffer, *pv1, v_inter, *pv2, COLORS[i % COLORS.len()]);
        } else {
            draw_flat_bottom_triangle(frame_buffer, *pv0, v_inter, *pv1, COLORS[i % COLORS.len()]);
            draw_flat_top_triangle(frame_buffer, v_inter, *pv1, *pv2, COLORS[i % COLORS.len()]);
        }
    }
}

fn draw_flat_top_triangle(frame_buffer: &mut [Vec<u8>], v0: Vec2, v1: Vec2, v2: Vec2, char: u8) {
    let m0: f32 = (v2.x - v0.x) / (v2.y - v0.y);
    let m1: f32 = (v2.x - v1.x) / (v2.y - v1.y);

    let y_start: usize = (v0.y - 0.5).ceil() as usize;
    let y_end: usize = (v2.y - 0.5).ceil() as usize;

    (y_start..y_end).for_each(|y| {
        let px0: f32 = m0 * (y as f32 + 0.5 - v0.y) + v0.x;
        let px1: f32 = m1 * (y as f32 + 0.5 - v1.y) + v1.x;

        let x_start: usize = (px0 - 0.5).ceil() as usize;
        let x_end: usize = (px1 - 0.5).ceil() as usize;

        for x in x_start..x_end {
            frame_buffer[y][x] = char;
        }
    });
}

fn draw_flat_bottom_triangle(frame_buffer: &mut [Vec<u8>], v0: Vec2, v1: Vec2, v2: Vec2, char: u8) {
    let m0: f32 = (v1.x - v0.x) / (v1.y - v0.y);
    let m1: f32 = (v2.x - v0.x) / (v2.y - v0.y);

    let y_start: usize = (v0.y - 0.5).ceil() as usize;
    let y_end: usize = (v2.y - 0.5).ceil() as usize;

    (y_start..y_end).for_each(|y| {
        let px0: f32 = m0 * (y as f32 + 0.5 - v0.y) + v0.x;
        let px1: f32 = m1 * (y as f32 + 0.5 - v0.y) + v0.x;

        let x_start: usize = (px0 - 0.5).ceil() as usize;
        let x_end: usize = (px1 - 0.5).ceil() as usize;

        for x in x_start..x_end {
            frame_buffer[y][x] = char;
        }
    })
}
