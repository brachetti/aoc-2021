use std::{collections::BTreeMap, fmt::{self, Debug}};
// extern crate conv;
// use conv::*;

use log::{info, debug};

type Test = usize;
type Counter = usize;

struct Calculation(BTreeMap<Test, Counter>);

impl Calculation {
    fn with_simple_diff(numbers: &Vec<usize>) -> Self {
        let mut collection = BTreeMap::new();
        let (min, max) = Self::min_max(&numbers);
        for num in min..max {
            // no need to de-dup
            let mut result: usize = 0;
            for elem in numbers {
                result += Self::difference(*elem, num);
            }
            collection.insert(num, result);
            debug!("simple calc: {} -> {}", num, result);
        }

        Calculation { 0: collection }
    }

    fn with_linear_diff(numbers: &Vec<usize>) -> Self {
        let mut collection = BTreeMap::new();
        let (min, max) = Self::min_max(&numbers);
        for num in min..max {
            // no need to de-dup
            let mut result: usize = 0;
            for elem in numbers {
                result += Self::linear_difference(*elem, num);
            }
            collection.insert(num, result);
        }

        Calculation { 0: collection }
    }

    fn minimum(&self) -> (Test, Counter) {
        let entry = self.0.iter().min_by(|a, b| a.1.cmp(b.1)).unwrap();
        (*entry.0, *entry.1)
    }

    fn min_max(numbers: &Vec<usize>) -> (Test, Test) {
        (numbers[0], numbers[numbers.len() - 1])
    }

    fn difference(a: usize, b: usize) -> usize {
        if a > b {
            a - b
        } else {
            b - a
        }
    }

    fn linear_difference(a: usize, b: usize) -> usize {
        let min = usize::min(a, b);
        let max = usize::max(a, b);
        let mut counter = 0;
        for i in min..=max {
            counter += Self::difference(min, i);
        }
        counter
    }
}

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
        let calc_simple = Calculation::with_simple_diff(&self.numbers);
        let (s_test, s_counter) = calc_simple.minimum();
        info!("Minimum with simple difference {} -> {}", s_test, s_counter);
        let calc_linear = Calculation::with_linear_diff(&self.numbers);
        let (l_test, l_counter) = calc_linear.minimum();
        info!("Minimum with linear difference {} -> {}", l_test, l_counter);
        info!("Finished calculation");
    }

    // fn median(&self) -> usize {
    //     let mid = self.numbers.len() / 2;
    //     self.numbers[mid]
    // }

    // fn mean(&self) -> usize {
    //     let sum: usize = self.numbers.iter().sum();
    //     // f64::from(sum) / self.numbers.len();
    //     let sum64 = f64::value_from(sum);
    //     let len64 = f64::value_from(self.numbers.len());
    //     if sum64.is_err() && len64.is_err() {
    //         panic!("could not convert");
    //     }
    //     (f64::round(sum64.unwrap() / len64.unwrap())) as usize
    // }
}

impl fmt::Display for Simulation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.numbers)
    }
}

#[test]
fn test_linear_difference() -> std::result::Result<(), String> {
    let tests = [
        (16, 5, 66),
        (1, 5, 10),
        (2, 5, 6),
        (0, 5, 15),
        (4, 5, 1),
        (7, 5, 3),
        (14, 5, 45),
    ];

    for test in tests {
        let (a, b, expected) = test;
        let result = Calculation::linear_difference(a, b);
        if result == expected {
            continue;
        }
        return Err(format!(
            "lin_diff of {} and {} is {}, expected {}",
            a, b, result, expected
        ));
    }

    Ok(())
}
