//! Drawing primitives: Bresenham line, rect, ellipse, and flood fill.

use crate::canvas::Canvas;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self { Self { x, y } }
    pub fn from_u32(x: u32, y: u32) -> Self { Self { x: x as i32, y: y as i32 } }
}

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
        if x0 == x1 && y0 == y1 { break; }
        let e2 = 2 * err;
        if e2 >= dy { err += dy; x0 += sx; }
        if e2 <= dx { err += dx; y0 += sy; }
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
        // Draw every other pixel for dashed effect
        if step % 2 == 0 {
            canvas.set_pixel(x0 as u32, y0 as u32, color);
        }
        if x0 == x1 && y0 == y1 { break; }
        let e2 = 2 * err;
        if e2 >= dy { err += dy; x0 += sx; }
        if e2 <= dx { err += dx; y0 += sy; }
        step += 1;
    }
}

/// Draw a dashed rectangle outline for preview rendering.
pub fn draw_dashed_rect(canvas: &mut Canvas, p0: Point, p1: Point, color: crate::egui::Color32) {
    let x0 = p0.x.min(p1.x); let x1 = p0.x.max(p1.x);
    let y0 = p0.y.min(p1.y); let y1 = p0.y.max(p1.y);
    draw_dashed_line(canvas, Point::new(x0, y0), Point::new(x1, y0), color);
    draw_dashed_line(canvas, Point::new(x0, y1), Point::new(x1, y1), color);
    draw_dashed_line(canvas, Point::new(x0, y0), Point::new(x0, y1), color);
    draw_dashed_line(canvas, Point::new(x1, y0), Point::new(x1, y1), color);
}

/// Draw a dashed ellipse outline for preview rendering.
pub fn draw_dashed_ellipse(canvas: &mut Canvas, center: Point, rx: i32, ry: i32, color: crate::egui::Color32) {
    // Use the same ellipse algorithm but draw only every other pixel
    let _x = 0_i64;
    let mut y = -(ry as i64);
    let mut px = 0_i64;
    let mut py = (ry * ry) as i64;
    let mut px2 = 2 * rx as i64 * rx as i64;
    let mut py2 = 2 * ry as i64 * ry as i64;
    let mut p = (ry as i64 * ry as i64) - (rx as i64 * rx as i64 * ry as i64) + (rx as i64 * rx as i64 / 4);
    let mut step = 0i64;
    while px < px2 {
        if step % 2 == 0 {
            let cx = center.x + px as i32;
            let cy = center.y - py as i32;
            canvas.set_pixel(cx.max(0) as u32, cy.max(0) as u32, color);
            canvas.set_pixel(cx.max(0) as u32, (-cy).max(0) as u32, color);
            canvas.set_pixel((-cx).max(0) as u32, cy.max(0) as u32, color);
            canvas.set_pixel((-cx).max(0) as u32, (-cy).max(0) as u32, color);
        }
        if p < 0 { px += py2; px2 += py2; p += py2; }
        else { px += py2; px2 += py2; py -= px2; py2 -= px2; p += py2 - px2; }
        step += 1;
    }
    p = (ry as i64 * ry as i64) + (rx as i64 * rx as i64 / 4);
    step = 0;
    while y < 0 {
        if step % 2 == 0 {
            let cx = center.x + px as i32;
            let cy = center.y - py as i32;
            canvas.set_pixel(cx.max(0) as u32, cy.max(0) as u32, color);
            canvas.set_pixel(cx.max(0) as u32, (-cy).max(0) as u32, color);
            canvas.set_pixel((-cx).max(0) as u32, cy.max(0) as u32, color);
            canvas.set_pixel((-cx).max(0) as u32, (-cy).max(0) as u32, color);
        }
        if p > 0 { py -= px2; py2 -= px2; p -= px2; }
        else { px += py2; px2 += py2; py -= px2; py2 -= px2; p += py2 - px2; }
        y += 1;
        step += 1;
    }
}

pub fn draw_rect(canvas: &mut Canvas, p0: Point, p1: Point, color: crate::egui::Color32) {
    let x0 = p0.x.min(p1.x); let x1 = p0.x.max(p1.x);
    let y0 = p0.y.min(p1.y); let y1 = p0.y.max(p1.y);
    draw_line(canvas, Point::new(x0, y0), Point::new(x1, y0), color);
    draw_line(canvas, Point::new(x0, y1), Point::new(x1, y1), color);
    draw_line(canvas, Point::new(x0, y0), Point::new(x0, y1), color);
    draw_line(canvas, Point::new(x1, y0), Point::new(x1, y1), color);
}

pub fn fill_rect(canvas: &mut Canvas, p0: Point, p1: Point, color: crate::egui::Color32) {
    let x0 = p0.x.min(p1.x); let x1 = p0.x.max(p1.x);
    let y0 = p0.y.min(p1.y); let y1 = p0.y.max(p1.y);
    let w = canvas.width() as i32; let h = canvas.height() as i32;
    for y in y0..=y1 {
        for x in x0..=x1 {
            if x >= 0 && x < w as i32 && y >= 0 && y < h as i32 {
                canvas.set_pixel(x as u32, y as u32, color);
            }
        }
    }
}

pub fn draw_ellipse(canvas: &mut Canvas, center: Point, rx: i32, ry: i32, color: crate::egui::Color32) {
    let _x = 0_i64;
    let mut y = -(ry as i64);
    let mut px = 0_i64;
    let mut py = (ry * ry) as i64;
    let mut px2 = 2 * rx as i64 * rx as i64;
    let mut py2 = 2 * ry as i64 * ry as i64;
    let mut p = (ry as i64 * ry as i64) - (rx as i64 * rx as i64 * ry as i64) + (rx as i64 * rx as i64 / 4);
    while px < px2 {
        let cx = center.x + px as i32;
        let cy = center.y - py as i32;
        canvas.set_pixel(cx.max(0) as u32, cy.max(0) as u32, color);
        canvas.set_pixel(cx.max(0) as u32, (-cy).max(0) as u32, color);
        canvas.set_pixel((-cx).max(0) as u32, cy.max(0) as u32, color);
        canvas.set_pixel((-cx).max(0) as u32, (-cy).max(0) as u32, color);
        if p < 0 { px += py2; px2 += py2; p += py2; }
        else { px += py2; px2 += py2; py -= px2; py2 -= px2; p += py2 - px2; }
    }
    p = (ry as i64 * ry as i64) + (rx as i64 * rx as i64 / 4);
    while y < 0 {
        let cx = center.x + px as i32;
        let cy = center.y - py as i32;
        canvas.set_pixel(cx.max(0) as u32, cy.max(0) as u32, color);
        canvas.set_pixel(cx.max(0) as u32, (-cy).max(0) as u32, color);
        canvas.set_pixel((-cx).max(0) as u32, cy.max(0) as u32, color);
        canvas.set_pixel((-cx).max(0) as u32, (-cy).max(0) as u32, color);
        if p > 0 { py -= px2; py2 -= px2; p -= px2; }
        else { px += py2; px2 += py2; py -= px2; py2 -= px2; p += py2 - px2; }
        y += 1;
    }
}

pub fn fill_ellipse(canvas: &mut Canvas, center: Point, rx: i32, ry: i32, color: crate::egui::Color32) {
    let w = canvas.width() as i32; let h = canvas.height() as i32;
    for y in -ry..=ry {
        if center.y + y < 0 || center.y + y >= h as i32 { continue; }
        if rx == 0 {
            canvas.set_pixel(center.x as u32, (center.y + y).max(0) as u32, color);
            continue;
        }
        let x_max = ((rx as f64 * rx as f64 * (ry * ry - y * y) as f64 / (ry * ry) as f64).sqrt() as i32).max(0);
        for x in -x_max..=x_max {
            let px = center.x + x; let py = center.y + y;
            if px >= 0 && px < w as i32 && py >= 0 && py < h as i32 {
                canvas.set_pixel(px as u32, py as u32, color);
            }
        }
    }
}

pub fn flood_fill(canvas: &mut Canvas, start: Point, new_color: crate::egui::Color32, tolerance: u8) {
    let target_color = match canvas.get_pixel(start.x as u32, start.y as u32) { Some(c) => c, None => return };
    if color_distance(target_color, new_color) <= tolerance as u32 { return; }
    let mut stack = std::collections::VecDeque::new();
    stack.push_back(start);
    let w = canvas.width(); let h = canvas.height();
    while let Some(p) = stack.pop_front() {
        if p.x < 0 || p.x >= w as i32 || p.y < 0 || p.y >= h as i32 { continue; }
        let current = match canvas.get_pixel(p.x as u32, p.y as u32) { Some(c) => c, None => continue };
        if color_distance(current, target_color) > tolerance as u32 { continue; }
        canvas.set_pixel(p.x as u32, p.y as u32, new_color);
        stack.push_back(Point::new(p.x + 1, p.y));
        stack.push_back(Point::new(p.x - 1, p.y));
        stack.push_back(Point::new(p.x, p.y + 1));
        stack.push_back(Point::new(p.x, p.y - 1));
    }
}

fn color_distance(a: crate::egui::Color32, b: crate::egui::Color32) -> u32 {
    (a[0] as i32 - b[0] as i32).unsigned_abs() as u32
        + (a[1] as i32 - b[1] as i32).unsigned_abs() as u32
        + (a[2] as i32 - b[2] as i32).unsigned_abs() as u32
        + (a[3] as i32 - b[3] as i32).unsigned_abs() as u32
}
