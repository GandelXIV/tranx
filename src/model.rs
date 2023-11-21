use std::sync::RwLock;

pub struct Model {
    pub score: RwLock<usize>,
}

impl Model {
    pub fn new() -> Self {
        Self {
            score: RwLock::new(0),
        }
    }
}
