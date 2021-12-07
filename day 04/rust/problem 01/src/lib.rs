use log::{info, debug};
use std::fmt;
use std::str::Lines;

const BOARD_DIMENSIONS: usize = 5;

#[derive(Debug, Copy, Clone)]
struct BingoNumber {
    number: u8,
    marked: bool,
}

impl BingoNumber {
    fn new(number: u8) -> Self {
        BingoNumber{ number: number, marked: false }
    }

    fn mark_if_hit(&self, by: u8) -> Self {
        if self.number == by {
            BingoNumber{ number: self.number, marked: true }
        } else {
            *self
        }
    }

    fn is_marked(&self) -> bool {
        self.marked
    }
}

impl fmt::Display for BingoNumber {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let marker = if self.is_marked() { "*" } else { " " };
        write!(f, "{:02}{} ", self.number, marker)
    }
}

impl Default for BingoNumber {
    fn default() -> Self {
        BingoNumber::new(0)
    }
}

#[derive(Debug, Clone)]
struct Board {
    id: u8,
    numbers: Vec<Vec<BingoNumber>>,
}

impl Board {
    fn new(input: Vec<&str>, id: u8) -> Self {
        let mut numbers: Vec<Vec<BingoNumber>> = Vec::new();
        
        for line in input {
            let mut line_numbers: Vec<BingoNumber> = Vec::new();
            for num in line.split_whitespace() {
                let num_u8 : u8 = match num.parse::<u8>() {
                    Ok(n) => n,
                    Err(_) => { panic!("Could not parse {:?} as u8", num) }
                };

                line_numbers.push(BingoNumber::new(num_u8));
            }
            numbers.push(line_numbers);
        }

        Board { numbers: numbers, id: id }
    }

    fn mark(&self, number: u8) -> Self {
        let mut new_numbers: Vec<Vec<BingoNumber>> = Vec::new();
        for line in &self.numbers {
            let new_line: Vec<BingoNumber> = line.iter().map(|&n| n.mark_if_hit(number)).collect();
            new_numbers.push(new_line);
        }
        
        Board { numbers: new_numbers, id: self.id }
    }

    fn any_line_fully_marked(&self) -> bool {
        self.numbers.iter().any(|line| line.iter().all(|n| n.is_marked()))
    }

    fn any_column_fully_marked(&self) -> bool {
        for col in 0..BOARD_DIMENSIONS {
            if self.numbers.iter().map(|row| row[col]).all(|num: BingoNumber| num.is_marked()) {
                return true
            }
        }
        false
    }

    fn has_won(&self) -> bool {
        self.any_line_fully_marked() 
        || self.any_column_fully_marked()
        || false
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Board {}:", self.id);
        for line in &self.numbers {
            for number in line {
                match write!(f, "{}", number) {
                    Err(_) => (),
                    _ => continue
                }
            }
            write!(f, "\n");
        }

        Ok(())
    }
}

#[derive(Debug)]
struct BingoGame {
   numbers: Vec<u8>,
   boards: Vec<Board>,
}

impl BingoGame {
    fn new(numbers: Vec<u8>, boards: Vec<Board>) -> Self {
        let mut stacked_numbers = numbers;
        stacked_numbers.reverse();
        BingoGame { numbers: stacked_numbers, boards: boards }
    }

    fn run(&mut self) -> Result<(), ()> {
        info!("Starting the game!");
        let mut winner: Board;

        while self.numbers.len() > 0 {
            self.step();
            let won_boards: Vec<u8> = self.boards.iter().filter(|b| b.has_won()).map(|b| b.id).collect();
            if won_boards.len() > 0 {
                info!("We have {} winner(s)!\n{:?}", won_boards.len(), won_boards);
                break;
            }
            // break;
        }

        // info!("we have a winner! it is Board {}", winner.id);
        Ok(())
    }

    fn step(&mut self) {
        let number = self.numbers.pop().unwrap();
        info!("We drew number {}, check your boards!", &number);
        self.boards = self.boards.iter().map(|b| b.mark(number)).collect();
        
        debug!("Boards:");
        for board in &self.boards {
            debug!("{}\n", board);
        }
    }
}

fn get_numbers(list: &str) -> Vec<u8> {
    list.split(",").map(str::parse::<u8>).map(std::result::Result::unwrap).collect()
}

fn get_boards(lines: Lines) -> Vec<Board> {
    let mut boards: Vec<Board> = Vec::new();

    let mut board_lines: Vec<&str> = lines.collect::<Vec<&str>>();
    board_lines.retain(|&l| l.len() > 0);

    let mut board_id = 1u8;
    for raw_board in board_lines.chunks(5) {
        let mut board_list: Vec<&str> = Vec::new();
        for line in raw_board {
            board_list.push(line);
        }
        let board = Board::new(board_list, board_id);
        board_id = board_id + 1;
        boards.push(board);
    }

    debug!("Boards:");
    for board in &boards {
        debug!("{}\n", board);
    }

    boards
}

pub fn play_bingo(input: String) -> Result<(), ()> {
    let mut lines = input.lines();
    let numbers = get_numbers(lines.next().unwrap());
    let boards = get_boards(lines);

    let mut game = BingoGame::new(numbers, boards);
    
    game.run()
}