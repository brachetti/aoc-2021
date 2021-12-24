mod algorithm;
mod image;

use algorithm::IEAlgorithm;
use image::Image;
use std::{fmt, result::Result};

#[derive(Debug)]
pub struct Simulation {
    algorithm: IEAlgorithm,
    image: Image,
}

impl Simulation {
    pub fn new(input: String) -> Self {
        let algo_input = input.lines().take(1).collect();
        let algorithm = IEAlgorithm::new(algo_input);
        let image_input = input.lines().skip(1).collect();
        let image = Image::with_starting_image(image_input);

        Self { algorithm, image }
    }

    pub fn run(&mut self) -> Result<(), ()> {
        Ok(())
    }
}

impl fmt::Display for Simulation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}", self.image)
    }
}
