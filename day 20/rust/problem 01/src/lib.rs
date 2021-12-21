use std::result::Result;

use arrayvec::{ArrayString, ArrayVec};

extern crate arrayvec;

#[derive(Copy, Clone, Debug)]
struct IEAlgorithm {
    description: ArrayString<512>,
}

impl IEAlgorithm {
    fn new(input: String) -> Self {
        let description = ArrayString::from(&input).unwrap();
        Self { description }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Simulation {
    algorithm: IEAlgorithm,
}

impl Simulation {
    pub fn new(input: String) -> Self {
        let lines = input.lines();
        let algo_input = lines.take(1).collect();
        let algorithm = IEAlgorithm::new(algo_input);

        Self { algorithm }
    }

    pub fn run(&mut self) -> Result<(), ()> {
        Ok(())
    }
}
