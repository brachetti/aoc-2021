use std::fmt;
use std::str::FromStr;

use log::{info, debug};

const FISH_RESTART_TIMER: u8 = 6;
const FISH_SPAWN_TIMER: u8 = 8;
const RUN_FOR_DAYS: u16 = 256;

#[derive(Clone, Debug)]
pub struct Simulation {
    days: u16,
    fishes: Vec<u8>,
}

impl Simulation {
    pub fn new(input: String) -> Self {
        Simulation {
            days: 0,
            fishes: input.split_terminator(",").map(str::trim).map(u8::from_str).map(|res| res.unwrap()).collect(),
        }
    }

    pub fn run(&mut self) -> Result<(), ()> {
        info!("Running Simulation");
        for day in 0..RUN_FOR_DAYS {
            self.days = day;
            debug!("{}", self);
            self.age_one_day();
            info!("Day {}", day);
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
        let mut new_fishes: Vec<u8> = Vec::new();
        self.fishes = self.fishes.iter().map(|fish| {
            if self.spawns_new_fish(fish.clone()) {
                new_fishes.push(self.spawn_new());
            }
            self.age(*fish)
        }).collect();
        self.fishes.append(&mut new_fishes);
    }

    fn spawn_new(&self) -> u8 {
        FISH_SPAWN_TIMER
    }

    fn spawns_new_fish(&self, num: u8) -> bool {
        num == 0
    }

    fn age(&self, fish: u8) -> u8{
        if self.spawns_new_fish(fish) {
            FISH_RESTART_TIMER
        } else {
            fish - 1
        }
    }

    fn fmt_fishes(&self) -> String {
        let res: Vec<String> = self.fishes.iter().map(u8::to_string).collect();
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