use std::sync::RwLock;

pub struct Model {
    pub score: RwLock<usize>,
}

const START_SCORE: usize = 0;

impl Model {
    pub fn new() -> Self {
        Self {
            score: RwLock::new(START_SCORE),
        }
    }

    pub fn increment_score(&self) {
        let mut x = self.score.write().unwrap();
        *x += 1;
        // Prevent integer overflow
        if *x == std::usize::MAX {
            *x = 0;
        }
    }

    pub fn get_score(&self) -> usize {
        *self.score.read().unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn scoring() {
        let model = Model::new();
        assert_eq!(model.get_score(), START_SCORE);
        model.increment_score();
        assert_eq!(model.get_score(), START_SCORE + 1);
    }
}
