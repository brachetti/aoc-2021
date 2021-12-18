use std::collections::BTreeMap;
use std::fmt;
use std::str::FromStr;

use log::{info, debug};

const FISH_RESTART_TIMER: u8 = 6;
const FISH_SPAWN_TIMER: u8 = 8;
const RUN_FOR_DAYS: u16 = 256;

#[derive(Clone, Debug)]
pub struct Simulation {
    days: u16,
    fishes: BTreeMap<u8, usize>,
}

impl Simulation {
    pub fn new(input: String) -> Self {
        let mut fishes = BTreeMap::new();
        for mut item in input.split_terminator(",") {
            item = item.trim();
            let num = u8::from_str(item).unwrap();
            fishes.entry(num).and_modify(|f| { *f += 1 }).or_insert(1);
        }
        Simulation {
            days: 0,
            fishes,
        }
    }

    pub fn run(&mut self) -> Result<(), ()> {
        info!("Running Simulation");
        for day in 0..RUN_FOR_DAYS {
            self.days = day;
            debug!("{}", self);
            self.age_one_day();
            // info!("Day {}", day);
        }
        self.days = RUN_FOR_DAYS;
        debug!("{}", self);
        self.print_summary();
        Ok(())
    }

    fn print_summary(&self) {
        info!("After {} days, there are {} fishes!", RUN_FOR_DAYS, self.amount_fishes());
    }

    fn amount_fishes(&self) -> usize {
        self.fishes.iter().fold(0, |sum, entry| {
            sum + *entry.1
        })
    }

    fn age_one_day(&mut self) {
        let mut new_fishes: BTreeMap<u8, usize> = BTreeMap::new();
        for (fish, count) in self.fishes.iter() {
            if self.spawns_new_fish(*fish) {
                new_fishes.entry(FISH_RESTART_TIMER).or_insert(*count);
                new_fishes
                    .entry(FISH_SPAWN_TIMER)
                    .and_modify(|f| { *f += *count })
                    .or_insert(*count);
            } else {
                new_fishes
                    .entry(*fish - 1)
                    .and_modify(|f| { *f += *count })
                    .or_insert(*count);
            }
        }
        self.fishes = new_fishes;
    }

    
    fn spawns_new_fish(&self, num: u8) -> bool {
        num == 0
    }

    fn fmt_fishes(&self) -> String {
        let res: Vec<String> = self.fishes.iter().map(|(fishes, count)| format!("{}*{}", count, fishes)).collect();
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