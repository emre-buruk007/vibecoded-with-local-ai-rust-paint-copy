//! Line drawing primitives: Point struct and Bresenham line algorithms.

use crate::canvas::Canvas;

/// A 2D point with integer coordinates.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
    pub fn from_u32(x: u32, y: u32) -> Self {
        Self { x: x as i32, y: y as i32 }
    }
}

/// Draw a solid line between two points using Bresenham's algorithm.
pub fn draw_line(canvas: &mut Canvas, p0: Point, p1: Point, color: crate::egui::Color32) {
    let (mut x0, mut y0) = (p0.x, p0.y);
    let (x1, y1) = (p1.x, p1.y);
    let dx = (x1 - x0).unsigned_abs() as i32;
    let dy = (y1 - y0).unsigned_abs() as i32;
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy;
    loop {
        canvas.set_pixel(x0 as u32, y0 as u32, color);
        if x0 == x1 && y0 == y1 {
            break;
        }
        let e2 = 2 * err;
        if e2 >= dy {
            err += dy;
            x0 += sx;
        }
        if e2 <= dx {
            err += dx;
            y0 += sy;
        }
    }
}

/// Draw a dashed (dotted) line for preview rendering.
/// Every other pixel is drawn, creating a dotted preview effect.
pub fn draw_dashed_line(canvas: &mut Canvas, p0: Point, p1: Point, color: crate::egui::Color32) {
    let (mut x0, mut y0) = (p0.x, p0.y);
    let (x1, y1) = (p1.x, p1.y);
    let dx = (x1 - x0).unsigned_abs() as i32;
    let dy = (y1 - y0).unsigned_abs() as i32;
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy;
    let mut step = 0i32;
    loop {
        if step % 2 == 0 {
            canvas.set_pixel(x0 as u32, y0 as u32, color);
        }
        if x0 == x1 && y0 == y1 {
            break;
        }
        let e2 = 2 * err;
        if e2 >= dy {
            err += dy;
            x0 += sx;
        }
        if e2 <= dx {
            err += dx;
            y0 += sy;
        }
        step += 1;
    }
}
