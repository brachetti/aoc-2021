#[macro_use]
extern crate log;

use aoc_2021_120701::Simulation;

fn get_input() -> String {
    let args: Vec<String> = std::env::args().collect();
    let input = args.get(1).expect("Missing commandline argument!");

    std::fs::read_to_string(input).expect("Could not read from file!")
}

fn main() {
    env_logger::init();
    debug!("starting up");
    let input = get_input();
    debug!("input: {}", input);

    let simulation = Simulation::new(input);

    simulation.run()
}
