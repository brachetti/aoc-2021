use std::fmt;

use log::info;

#[derive(Clone, Debug)]
pub struct Simulation {
    numbers: Vec<usize>,
}

impl Simulation {
    pub fn new(input: String) -> Self {
        let mut numbers: Vec<usize> = input
            .split(",")
            .map(str::trim)
            .map(|n| usize::from_str_radix(n, 10))
            .map(|e| e.unwrap())
            .collect();
        numbers.sort();
        Simulation { numbers }
    }

    pub fn run(&self) {
        info!("Starting calculation");
        info!("Median is {}", self.median());
        info!("Fuel consumption is {}", self.fuel_consumption());
        info!("Finished calculation");
    }

    fn fuel_consumption(&self) -> usize {
        let median = self.median();
        let mut result: usize = 0;
        for elem in &self.numbers {
            result += self.difference(*elem, median);
        }
        result
    }

    fn difference(&self, a: usize, b: usize) -> usize {
        if a > b {
            a - b
        } else {
            b - a
        }
    }

    fn median(&self) -> usize {
        let mid = self.numbers.len() / 2;
        self.numbers[mid]
    }
}

impl fmt::Display for Simulation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.numbers)
    }
}
