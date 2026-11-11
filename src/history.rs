use std::collections::VecDeque;

pub struct UndoStack {
    history: VecDeque<Vec<u8>>,
    max_steps: usize,
}

impl UndoStack {
    pub fn new(max: usize) -> Self {
        Self { history: VecDeque::with_capacity(max + 1), max_steps: max }
    }

    pub fn push(&mut self, data: Vec<u8>) {
        self.history.clear();
        self.history.push_back(data);
        if self.history.len() > self.max_steps {
            self.history.pop_front();
        }
    }

    pub fn pop(&mut self) -> Option<Vec<u8>> {
        self.history.pop_back()
    }

    pub fn len(&self) -> usize {
        self.history.len()
    }

    pub fn is_empty(&self) -> bool {
        self.history.is_empty()
    }
}
