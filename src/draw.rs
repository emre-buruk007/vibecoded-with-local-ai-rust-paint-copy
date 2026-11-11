//! DrawState: tracks the current drawing state (e.g., start position, previous point).

use eframe::egui;

/// Represents the current drawing state.
#[derive(Debug, Clone, PartialEq)]
pub enum DrawState {
    /// Not currently drawing.
    None,
    /// Currently drawing. Stores the starting screen position.
    Drawing {
        start_pos: egui::Pos2,
    },
}

impl DrawState {
    pub fn new(start_pos: egui::Pos2) -> Self {
        DrawState::Drawing { start_pos }
    }

    pub fn start_pos(&self) -> egui::Pos2 {
        match self {
            DrawState::None => panic!("DrawState::None has no start_pos"),
            DrawState::Drawing { start_pos } => *start_pos,
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
