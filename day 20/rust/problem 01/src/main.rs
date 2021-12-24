#[macro_use]
extern crate log;
extern crate num_traits;
extern crate quadtree_rs;

use aoc_2021_122001::Simulation;

fn get_input() -> String {
    let args: Vec<String> = std::env::args().collect();
    let input = args.get(1).expect("Missing commandline argument!");

    std::fs::read_to_string(input).expect("Could not read from file!")
}

fn main() -> Result<(), ()> {
    env_logger::init();

    debug!("starting up");
    let input = get_input();
    // debug!("input: {:?}", input);

    let mut simulation = Simulation::new(input);

    simulation.run()
}
