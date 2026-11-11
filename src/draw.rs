//! DrawState: tracks the current drawing state (e.g., start position, previous point).

use eframe::egui;

/// Represents the current drawing state.
#[derive(Debug, Clone, PartialEq)]
pub enum DrawState {
    /// Not currently drawing.
    None,
    /// Currently drawing. Stores the starting screen position.
    Drawing {
        start: egui::Pos2,
    },
}

impl DrawState {
    pub fn new(start: egui::Pos2) -> Self {
        DrawState::Drawing { start }
    }

    pub fn start(&self) -> egui::Pos2 {
        match self {
            DrawState::None => panic!("DrawState::None has no start"),
            DrawState::Drawing { start } => *start,
        }
    }

    pub fn update_prev(&mut self, _pos: egui::Pos2) {
        // For tools that need previous point tracking, update here
        match self {
            DrawState::Drawing { .. } => {}
            DrawState::None => {}
        }
    }
}
