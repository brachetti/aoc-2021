#[macro_use]
extern crate log;

use aoc_2021_120401::{play_bingo, Strategy};

fn get_input() -> (String, Strategy) {
    let args: Vec<String> = std::env::args().collect();
    let input = args.get(1).expect("Missing commandline argument!");

     let game_input = std::fs::read_to_string(input)
        .expect("Could not read from file!");

    let strategy = match args.get(2).unwrap_or(&String::from("first")).as_str() {
        "first" => Strategy::First,
        "last" => Strategy::Last,
        _ => Strategy::First,
    };

    (game_input, strategy)
}

fn main() -> Result<(), ()> {
    env_logger::init();

    debug!("starting up");
    let (input, strategy) = get_input();

    play_bingo(input, strategy)
}
