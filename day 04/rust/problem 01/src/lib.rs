use log::{info, debug};
use std::fmt;
use std::str::Lines;

const BOARD_DIMENSIONS: usize = 5;

#[derive(Debug, Copy, Clone)]
struct BingoNumber {
    number: usize,
    marked: bool,
}

impl BingoNumber {
    fn new(number: usize) -> Self {
        BingoNumber{ number: number, marked: false }
    }

    fn mark_if_hit(&self, by: usize) -> Self {
        if self.number == by {
            BingoNumber{ number: self.number, marked: true }
        } else {
            *self
        }
    }

    fn is_marked(&self) -> bool {
        self.marked
    }

    fn score(&self) -> usize {
        match self.is_marked() {
            false => self.number,
            _ => 0,
        }
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
    id: usize,
    numbers: Vec<Vec<BingoNumber>>,
}

impl Board {
    fn new(input: Vec<&str>, id: usize) -> Self {
        let mut numbers: Vec<Vec<BingoNumber>> = Vec::new();
        
        for line in input {
            let mut line_numbers: Vec<BingoNumber> = Vec::new();
            for num in line.split_whitespace() {
                let num_u8 : usize = match num.parse::<usize>() {
                    Ok(n) => n,
                    Err(_) => { panic!("Could not parse {:?} as u8", num) }
                };

                line_numbers.push(BingoNumber::new(num_u8));
            }
            numbers.push(line_numbers);
        }

        Board { numbers: numbers, id: id }
    }

    fn mark(&self, number: usize) -> Self {
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

    fn get_score(&self) -> usize {
        self.numbers.iter().fold(0, |sum, list| {
            sum + list.iter().fold(0, |s, num| s + num.score())
        })
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
   numbers: Vec<usize>,
   boards: Vec<Board>,
   last_number: usize,
   strategy: Strategy,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Strategy {
    First,
    Last,
}

impl BingoGame {
    fn new(numbers: Vec<usize>, boards: Vec<Board>, strategy: Strategy) -> Self {
        let mut stacked_numbers = numbers;
        stacked_numbers.reverse();
        BingoGame { numbers: stacked_numbers, boards: boards, last_number: 0, strategy: strategy }
    }

    fn find_winning_board(&mut self) -> Result<&Board, ()> {
        while self.numbers.len() > 0 {
            self.step();
            let mut index = 0;
            let mut winner: Option<&Board> = None;
            for board in &self.boards {
                if board.has_won() {
                    winner = Some(&board.clone());
                }
                index = index + 1;
            }
            
            if winner.is_some() {
                let won = winner.unwrap();
                if self.strategy == Strategy::Last {
                    if self.boards.len() > 1 {
                        self.boards.remove(index);
                        continue;
                    }
                }
                return Ok(won);
            }
        }

        Err(())
    }

    fn run(&mut self) -> Result<(), ()> {
        info!("Starting the game!");
        
        let winner = match self.find_winning_board() {
            Ok(board) => board.clone(),
            Err(_) => return Err(())
        };

        info!("we have a winner! it is Board {}:\n{}", winner.id, winner);
        
        self.calculate_score(&winner);

        Ok(())
    }

    fn calculate_score(&self, winner: &Board) {
        debug!("Calculating winning score for board \n{}", winner);
        let board_score = winner.get_score();
        let result = board_score * self.last_number;
        info!("And the winning score = {} * {} = {}", board_score, self.last_number, result);
    }

    fn step(&mut self) {
        let number = self.numbers.pop().unwrap();
        info!("We drew number {}, check your boards!", &number);
        self.boards = self.boards.iter().map(|b| b.mark(number)).collect();
        
        debug!("Boards:");
        for board in &self.boards {
            debug!("{}\n", board);
        }
        self.last_number = number;
    }
}

fn get_numbers(list: &str) -> Vec<usize> {
    list.split(",").map(str::parse::<usize>).map(std::result::Result::unwrap).collect()
}

fn get_boards(lines: Lines) -> Vec<Board> {
    let mut boards: Vec<Board> = Vec::new();

    let mut board_lines: Vec<&str> = lines.collect::<Vec<&str>>();
    board_lines.retain(|&l| l.len() > 0);

    let mut board_id = 1;
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

pub fn play_bingo(input: String, strategy: Strategy) -> Result<(), ()> {
    let mut lines = input.lines();
    let numbers = get_numbers(lines.next().unwrap());
    let boards = get_boards(lines);

    let mut game = BingoGame::new(numbers, boards, strategy);
    
    game.run()
}