#[derive(Clone, Copy, Debug)]
pub(crate) struct Player {
    number: usize,
    position: usize,
    score: usize,
}

impl Player {
    pub(crate) fn new(number: usize, position: usize) -> Self {
        let score = 0;
        Self {
            number,
            position,
            score,
        }
    }

    pub(crate) fn advance_to(&mut self, position: usize) {
        self.position = position;
    }

    pub(crate) fn current_position(&self) -> usize {
        self.position
    }

    /// Get a reference to the player's position.
    pub(crate) fn number(&self) -> usize {
        self.number
    }

    pub(crate) fn recalculate_score(&mut self) {
        self.score += self.position
    }

    pub(crate) fn score(&self) -> usize {
        self.score
    }
}
