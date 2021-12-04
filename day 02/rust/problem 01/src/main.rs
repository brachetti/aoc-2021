use std::env;
use std::fs;
use std::str::FromStr;
use std::str::Utf8Error;
use std::iter::FromIterator;

#[derive(Debug)]
enum Direction {
    Up, Down, Forward
}

#[derive(Debug)]
struct Command {
    direction: Direction,
    units: usize
}

impl FromStr for Command {
    type Err = Utf8Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // let pair = s.split(" ").collect::<Vec<_>>();
        // match pair {
        //     ["forward", unit] => Command { direction: Direction::Forward }
        // }

        let parts = s.split(';').collect::<Vec<&str>>();
        let (direction, unit) = (parts[0], parts[1]);
        let result = match direction {
            "forward" => Command { direction: Direction::Forward, units: usize::from_str(unit).unwrap() },
            "up" => Command { direction: Direction::Up, units: usize::from_str(unit).unwrap() },
            "down" => Command { direction: Direction::Down, units: usize::from_str(unit).unwrap() },
        };

        Ok(result)
    }
}

#[derive(Debug)]
struct Commands(Vec<Command>);

impl Commands {
    fn new() -> Commands {
        Commands(Vec::new())
    }

    fn add(&mut self, elem: Command) {
        self.0.push(elem);
    }
}

fn get_input() -> String {
    let args: Vec<String> = env::args().collect();
    let input = args.get(1).expect("Missing commandline argument!");

     fs::read_to_string(input)
        .expect("Could not read from file!")
}

fn get_commands() -> Vec<Command> {
    let contents = get_input();

    let raw_commands: Vec<&str> = contents
        .split("\n")
        .collect();

    let commands: Commands = raw_commands
        .map(Command::from_str);

    println!("Puzzle input: {:?}", raw_commands);

    Vec::new()
}

fn main() {
    let commands : Vec<Command> = get_commands();

    println!("Commands:\n{:?}", commands);
}
