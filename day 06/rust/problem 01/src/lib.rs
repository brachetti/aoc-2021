use std::fmt;
use std::num::ParseIntError;
use std::str::FromStr;

use log::{info, debug};

const FISH_RESTART_TIMER: u8 = 6;
const FISH_SPAWN_TIMER: u8 = 8;
const RUN_FOR_DAYS: u16 = 80;

#[derive(Copy, Clone, Debug)]
struct Fish {
    timer: u8,
}


impl Fish {
    fn new(timer: u8) -> Self {
        Fish { timer }
    }

    fn spawn_new() -> Self {
        Fish::new(FISH_SPAWN_TIMER)
    }

    fn spawns_new_fish(&self) -> bool {
        self.timer == 0
    }

    fn age(&mut self) {
        if self.spawns_new_fish() {
            self.timer = FISH_RESTART_TIMER;
            return
        }
        self.timer = self.timer - 1;
    }
}

impl fmt::Display for Fish {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.timer)
    }
}

impl FromStr for Fish {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        debug!("Fish from string {:?}", s);
        let value = s.parse::<u8>()?;

        Ok(Fish::new(value))
    }
}

#[derive(Clone, Debug)]
pub struct Simulation {
    days: u16,
    fishes: Vec<Fish>,
}

impl Simulation {
    pub fn new(input: String) -> Self {
        Simulation {
            days: 0,
            fishes: input.split_terminator(",").map(str::trim).map(Fish::from_str).map(|res| res.unwrap()).collect(),
        }
    }

    pub fn run(&mut self) -> Result<(), ()> {
        info!("Running Simulation");
        for day in 0..RUN_FOR_DAYS {
            self.days = day;
            debug!("{}", self);
            self.age_one_day();
        }
        self.days = RUN_FOR_DAYS;
        debug!("{}", self);
        self.print_summary();
        Ok(())
    }

    fn print_summary(&self) {
        info!("After {} days, there are {} fishes!", RUN_FOR_DAYS, self.fishes.len());
    }

    fn age_one_day(&mut self) {
        let mut new_fishes: Vec<Fish> = Vec::new();
        for mut fish in self.fishes.iter_mut() {
            if fish.spawns_new_fish() {
                new_fishes.push(Fish::spawn_new());
            }
            fish.age();
        }
        self.fishes.append(&mut new_fishes);
    }

    fn fmt_fishes(&self) -> String {
        let res: Vec<String> = self.fishes.iter().map(Fish::to_string).collect();
        res.join(",")
    }
}

impl fmt::Display for Simulation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.days == 0 {
            write!(f, "Initial state: {}", self.fmt_fishes())
        } else {
            write!(f, "After {:>2} days: {}", self.days, self.fmt_fishes())
        }
    }
}