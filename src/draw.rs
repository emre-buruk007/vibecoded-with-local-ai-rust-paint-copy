//! DrawState: tracks the current drawing state (e.g., start position, previous point).

use crate::shapes::Point;

/// Represents the current drawing state.
#[derive(Debug, Clone, PartialEq)]
pub enum DrawState {
    /// Not currently drawing.
    None,
    /// Currently drawing. Stores the starting canvas coordinates.
    Drawing {
        start: Point,
    },
}

impl DrawState {
    pub fn new(start: Point) -> Self {
        DrawState::Drawing { start }
    }

    pub fn start(&self) -> Point {
        match self {
            DrawState::None => panic!("DrawState::None has no start"),
            DrawState::Drawing { start } => *start,
        }
    }

    pub fn update_prev(&mut self, _pos: Point) {
        // For tools that need previous point tracking, update here
        match self {
            DrawState::Drawing { .. } => {}
            DrawState::None => {}
        }
    }
}
