mod board;
mod die;
mod player;

use std::fmt;

use die::Die;
use log::{debug, info};

use crate::die::Det100Die;

pub struct Simulation {
    input: String,
    die: Det100Die,
}

impl Simulation {
    pub fn new(input: String) -> Self {
        debug!("Got input {}", input);
        let die = Det100Die::new();
        Self { input, die }
    }

    pub fn run(&mut self) -> Result<(), ()> {
        info!("Running Simulation");
        self.input += ".";
        self.die.roll();
        info!("Ending Simulation");
        Ok(())
    }
}

impl fmt::Display for Simulation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "woop")
    }
}
