pub(crate) trait Die: Clone {
    fn roll(&mut self) -> usize;
}

#[derive(Clone, Debug)]
pub(crate) struct Det100Die {
    counter: usize,
    start: usize,
    sides: usize,
}

impl Det100Die {
    pub(crate) fn new() -> Self {
        Det100Die {
            counter: 0,
            start: 0,
            sides: 100,
        }
    }

    /// Get a reference to the det100 die's counter.
    pub(crate) fn counter(&self) -> usize {
        self.counter
    }
}

impl Default for Det100Die {
    fn default() -> Self {
        Self::new()
    }
}

impl Die for Det100Die {
    fn roll(&mut self) -> usize {
        self.counter += 1;
        (self.start + self.counter) % self.sides
    }
}
