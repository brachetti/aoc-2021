#[macro_use]
extern crate log;

fn get_input() -> String {
    let args: Vec<String> = std::env::args().collect();
    let input = args.get(1).expect("Missing commandline argument!");

     std::fs::read_to_string(input)
        .expect("Could not read from file!")
}

fn main() -> Result<(), ()> {
    env_logger::init();

    debug!("starting up");
    let input = get_input();

    aoc_2021_120401::play_bingo(input)
}
