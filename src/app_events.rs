//! Event handling for the PaintApp: mouse, keyboard, and pointer events.

use crate::app::PaintApp;
use crate::tool::Tool;
use crate::draw::DrawState;
use crate::shapes;
use crate::shapes::Point;
use crate::egui;

impl PaintApp {
    pub fn on_mouse_press(&mut self, ctx: &egui::Context) {
        if let Some(pt) = self.screen_to_canvas(self.pointer_pos.unwrap()) {
            match self.tool {
                Tool::Pencil | Tool::Eraser => {
                    self.save_state();
                    self.draw_state = DrawState::Drawing { start: pt };
                    let color = if self.tool == Tool::Eraser {
                        egui::Color32::WHITE
                    } else {
                        self.color
                    };
                    for dy in 0..=self.brush_size as i32 {
                        for dx in 0..=self.brush_size as i32 {
                            if dx * dx + dy * dy <= self.brush_size as i32 * self.brush_size as i32 {
                                let x = pt.x + dx - self.brush_size as i32 / 2;
                                let y = pt.y + dy - self.brush_size as i32 / 2;
                                if x >= 0 && y >= 0 && x < self.canvas.width() as i32 && y < self.canvas.height() as i32 {
                                    self.canvas.set_pixel(x as u32, y as u32, color);
                                }
                            }
                        }
                    }
                    ctx.request_repaint();
                }
                Tool::Fill => {
                    self.save_state();
                    shapes::flood_fill(&mut self.canvas, pt, self.color, 32);
                    ctx.request_repaint();
                }
                Tool::Eyedropper => {
                    if let Some(pix) = self.canvas.get_pixel(pt.x as u32, pt.y as u32) {
                        self.color = egui::Color32::from_rgba_unmultiplied(pix[0], pix[1], pix[2], pix[3]);
                    }
                    ctx.request_repaint();
                }
                Tool::Line | Tool::Rectangle | Tool::Ellipse => {
                    self.save_state();
                    self.draw_state = DrawState::Drawing { start: pt };
                }
            }
        }
    }

    pub fn on_mouse_drag(&mut self, ctx: &egui::Context) {
        if let DrawState::Drawing { .. } = self.draw_state {
            if let Some(pt) = self.screen_to_canvas(self.pointer_pos.unwrap()) {
                match self.tool {
                    Tool::Pencil | Tool::Eraser => {
                        let color = if self.tool == Tool::Eraser {
                            egui::Color32::WHITE
                        } else {
                            self.color
                        };
                        for dy in 0..=self.brush_size as i32 {
                            for dx in 0..=self.brush_size as i32 {
                                if dx * dx + dy * dy <= self.brush_size as i32 * self.brush_size as i32 {
                                    let x = pt.x + dx - self.brush_size as i32 / 2;
                                    let y = pt.y + dy - self.brush_size as i32 / 2;
                                    if x >= 0 && y >= 0 && x < self.canvas.width() as i32 && y < self.canvas.height() as i32 {
                                        self.canvas.set_pixel_unchecked(x as u32, y as u32, color);
                                    }
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
            ctx.request_repaint();
        }
    }

    pub fn on_mouse_release(&mut self, ctx: &egui::Context) {
        self.pointer_pos = ctx.pointer_hover_pos();
        if let DrawState::Drawing { start: start_pt } = self.draw_state {
            if let Some(end_pt) = self.screen_to_canvas(self.pointer_pos.unwrap()) {
                let tool_color = self.color;
                let dist = ((end_pt.x as f32 - start_pt.x as f32).powi(2)
                    + (end_pt.y as f32 - start_pt.y as f32).powi(2)).sqrt();
                if dist > 1.0 {
                    match self.tool {
                        Tool::Line => {
                            shapes::draw_line(&mut self.canvas, start_pt, end_pt, tool_color);
                        }
                        Tool::Rectangle => {
                            if self.fill_shape {
                                shapes::fill_rect(&mut self.canvas, start_pt, end_pt, tool_color);
                            } else {
                                shapes::draw_rect(&mut self.canvas, start_pt, end_pt, tool_color);
                            }
                        }
                        Tool::Ellipse => {
                            let cx = (start_pt.x + end_pt.x) / 2;
                            let cy = (start_pt.y + end_pt.y) / 2;
                            let rx = (end_pt.x - start_pt.x).abs() / 2;
                            let ry = (end_pt.y - start_pt.y).abs() / 2;
                            let center = Point::new(cx, cy);
                            if self.fill_shape {
                                shapes::fill_ellipse(&mut self.canvas, center, rx, ry, tool_color);
                            } else {
                                shapes::draw_ellipse(&mut self.canvas, center, rx, ry, tool_color);
                            }
                        }
                        _ => {}
                    }
                }
            }
            self.draw_state = DrawState::None;
            ctx.request_repaint();
        }
    }

    pub fn on_key(&mut self, ctx: &egui::Context) {
        ctx.input(|i| {
            if i.modifiers.ctrl {
                if i.key_pressed(egui::Key::Z) && !i.modifiers.shift {
                    self.undo();
                    ctx.request_repaint();
                } else if i.key_pressed(egui::Key::Z) && i.modifiers.shift {
                    self.redo();
                    ctx.request_repaint();
                }
            }
            if i.key_pressed(egui::Key::Escape) {
                self.draw_state = DrawState::None;
                ctx.request_repaint();
            }
        });
    }

    pub fn on_pointer(&mut self, ctx: &egui::Context) {
        self.pointer_pos = ctx.pointer_hover_pos();
        if let DrawState::Drawing { .. } = self.draw_state {
            ctx.request_repaint();
        }
    }
}