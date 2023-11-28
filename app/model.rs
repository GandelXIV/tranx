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

    pub fn increment_score(&self) {
        let mut x = self.score.write().unwrap();
        *x += 1;
    }

    pub fn get_score(&self) -> usize {
        *self.score.read().unwrap()
    }
}
