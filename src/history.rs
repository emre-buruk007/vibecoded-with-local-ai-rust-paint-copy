use std::collections::VecDeque;

pub struct UndoStack {
    undo_history: VecDeque<Vec<u8>>,
    redo_history: VecDeque<Vec<u8>>,
    max_steps: usize,
}

impl UndoStack {
    pub fn new(max: usize) -> Self {
        Self {
            undo_history: VecDeque::with_capacity(max + 1),
            redo_history: VecDeque::with_capacity(max + 1),
            max_steps: max,
        }
    }

    /// Push a new state. Clears redo history.
    pub fn push(&mut self, data: Vec<u8>) {
        self.redo_history.clear();
        self.undo_history.push_back(data);
        if self.undo_history.len() > self.max_steps {
            self.undo_history.pop_front();
        }
    }

    /// Pop the last undo state. Pushes current state to redo.
    pub fn pop(&mut self, current: &[u8]) -> Option<Vec<u8>> {
        if let Some(state) = self.undo_history.pop_back() {
            self.redo_history.push_back(current.to_vec());
            Some(state)
        } else {
            None
        }
    }

    /// Pop the last redo state. Pushes current to undo.
    pub fn redo_pop(&mut self, current: &[u8]) -> Option<Vec<u8>> {
        if let Some(state) = self.redo_history.pop_back() {
            self.undo_history.push_back(current.to_vec());
            Some(state)
        } else {
            None
        }
    }

    pub fn undo_len(&self) -> usize {
        self.undo_history.len()
    }

    pub fn redo_len(&self) -> usize {
        self.redo_history.len()
    }

    pub fn is_empty(&self) -> bool {
        self.undo_history.is_empty()
    }
}
