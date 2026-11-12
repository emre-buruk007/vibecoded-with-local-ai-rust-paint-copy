//! Core application: PaintApp struct, constructor, and state management.

use crate::canvas::Canvas;
use crate::draw::DrawState;
use crate::history::UndoStack;
use crate::tool::Tool;
use crate::egui;

/// Layout information for the canvas widget.
#[derive(Debug)]
pub struct CanvasLayout {
    pub rect: egui::Rect,
    pub scale: f32,
}

/// The main paint application state.
pub struct PaintApp {
    pub tool: Tool,
    pub color: egui::Color32,
    pub brush_size: f32,
    pub fill_shape: bool,
    pub canvas: Canvas,
    pub canvas_layout: CanvasLayout,
    pub draw_state: DrawState,
    pub pointer_pos: Option<egui::Pos2>,
    pub primary_just_pressed: bool,
    pub primary_just_released: bool,
    pub undo_stack: UndoStack,
    pub show_resize: bool,
    pub resize_w: String,
    pub resize_h: String,
    pub show_about: bool,
}

impl PaintApp {
    pub fn new() -> Self {
        PaintApp {
            tool: Tool::Pencil,
            color: egui::Color32::BLACK,
            brush_size: 5.0_f32,
            fill_shape: false,
            canvas: Canvas::new(),
            canvas_layout: CanvasLayout {
                rect: egui::Rect::NOTHING,
                scale: 1.0,
            },
            draw_state: DrawState::None,
            pointer_pos: None,
            primary_just_pressed: false,
            primary_just_released: false,
            undo_stack: UndoStack::new(50),
            show_resize: false,
            resize_w: Canvas::CW.to_string(),
            resize_h: Canvas::CH.to_string(),
            show_about: false,
        }
    }
}

impl PaintApp {
    pub fn save_state(&mut self) {
        let data = self.canvas.clone_data();
        self.undo_stack.push(data);
    }

    pub fn undo(&mut self) {
        if let Some(prev) = self.undo_stack.pop(self.canvas.data()) {
            self.undo_stack.push(self.canvas.clone_data());
            self.canvas.load_from_pixels(&prev);
        }
    }

    pub fn redo(&mut self) {
        let current = self.canvas.clone_data();
        if let Some(next) = self.undo_stack.redo_pop(&current) {
            self.canvas.load_from_pixels(&next);
        }
    }

    pub fn clear_canvas(&mut self) {
        self.save_state();
        self.canvas.fill(egui::Color32::WHITE);
    }

    pub fn canvas_to_color_image(&self) -> egui::ColorImage {
        let mut img = egui::ColorImage::new([Canvas::CW as usize, Canvas::CH as usize], egui::Color32::WHITE);
        let data = self.canvas.data();
        for idx in 0..img.pixels.len() {
            let i = idx * 4;
            if i + 3 < data.len() {
                img.pixels[idx] = egui::Color32::from_rgba_unmultiplied(data[i], data[i+1], data[i+2], data[i+3]);
            }
        }
        img
    }

    pub fn screen_to_canvas(&self, screen_pos: egui::Pos2) -> Option<crate::shapes::Point> {
        let dx = screen_pos.x - self.canvas_layout.rect.min.x;
        let dy = screen_pos.y - self.canvas_layout.rect.min.y;
        if dx < 0.0 || dy < 0.0 || dx > self.canvas_layout.rect.width() || dy > self.canvas_layout.rect.height() {
            return None;
        }
        Some(crate::shapes::Point::new((dx / self.canvas_layout.scale) as i32, (dy / self.canvas_layout.scale) as i32))
    }
}

impl PaintApp {
    /// Main render entry point: called from egui::App::update.
    pub fn render(&mut self, ctx: &egui::Context) {
        self.render_menu_bar(ctx);
        self.render_toolbar(ctx);
        self.render_layers_panel(ctx);
        self.render_canvas(ctx);
        self.show_resize_dialog(ctx);
        self.show_about_dialog(ctx);
        self.on_key(ctx);
        ctx.request_repaint();
    }
}

impl eframe::App for PaintApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.render(ctx);
    }
}