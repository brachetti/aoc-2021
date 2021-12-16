use log::{debug, info};
use std::fmt;
use std::fmt::Write as FmtWrite;
use std::num::ParseIntError;
use std::str::FromStr;

const DIM: usize = 1000;

#[derive(Copy, Clone, Debug)]
struct Point2D {
    x: isize,
    y: isize,
}

impl Point2D {
    fn new(x: isize, y: isize) -> Self {
        Point2D { x, y }
    }

    // fn hits(&self) -> String {
    //     match self.hit_counter {
    //         0 => String::from(" . "),
    //         _ => format!(" {} ", cmp::min(self.hit_counter, 9)),
    //     }
    // }
}

impl FromStr for Point2D {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords: Vec<&str> = s
            .trim_matches(|p| p == '(' || p == ')' || p == ' ')
            .split(',')
            .collect();

        let x_fromstr = coords[0].parse::<isize>()?;
        let y_fromstr = coords[1].parse::<isize>()?;

        Ok(Point2D::new(x_fromstr, y_fromstr))
    }
}

impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:01},{:01} ", self.x, self.y)
    }
}

#[derive(Copy, Clone, Debug)]
struct Line {
    from: Point2D,
    to: Point2D,
}

impl Line {
    fn new(from: Point2D, to: Point2D) -> Self {
        Line { from, to }
    }

    fn is_simple(&self) -> bool {
        self.is_vertical() || self.is_horizontal()
    }

    fn is_horizontal(&self) -> bool {
        self.from.y == self.to.y
    }

    fn is_vertical(&self) -> bool {
        self.from.x == self.to.x
    }

    fn is_diagonal(&self) -> bool {
        !self.is_simple()
    }

    fn get_all_points(&self) -> Vec<Point2D> {
        let mut result = Vec::new();
        if self.is_horizontal() {
            let y = self.from.y;
            let lower_bound = isize::min(self.from.x, self.to.x);
            let upper_bound = isize::max(self.from.x, self.to.x);
            for x in lower_bound..=upper_bound {
                result.push(Point2D::new(x, y));
            }
        } else if self.is_vertical() {
            let x = self.from.x;
            let lower_bound = isize::min(self.from.y, self.to.y);
            let upper_bound = isize::max(self.from.y, self.to.y);
            for y in lower_bound..=upper_bound {
                result.push(Point2D::new(x, y));
            }
        } else if self.is_diagonal() {
            debug!("Diagonal! vector {}", self);
            let lower_bound = if self.from.x <= self.to.x {
                self.from
            } else {
                self.to
            };
            let upper_bound = if self.from.x <= self.to.x {
                self.to
            } else {
                self.from
            };
            let y_step_up = lower_bound.y <= upper_bound.y;
            let mut y = lower_bound.y;
            for x in lower_bound.x..=upper_bound.x {
                result.push(Point2D::new(x, y));
                if y_step_up {
                    y = y + 1;
                } else {
                    if y == 0 {
                        break
                    }
                    if y > 0 {
                        y = y - 1;
                    }
                }
            }
        }

        result
    }
}

impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} -> {}", self.from, self.to)
    }
}

impl FromStr for Line {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords: Vec<&str> = s.trim_matches(|p| p == ' ').split("->").collect();

        let point_from: Point2D = coords[0].parse()?;
        let point_to: Point2D = coords[1].parse()?;

        Ok(Line::new(point_from, point_to))
    }
}

impl From<(Point2D, Point2D)> for Line {
    fn from(points: (Point2D, Point2D)) -> Self {
        let a: Point2D = points.0;
        let b: Point2D = points.1;
        Line::new(a, b)
    }
}

impl From<[isize; 4]> for Line {
    fn from(points: [isize; 4]) -> Self {
        Line::new(
            Point2D::new(points[0], points[1]),
            Point2D::new(points[2], points[3]),
        )
    }
}

#[derive(Copy, Clone, Debug)]
struct Board {
    items: [u8; DIM * DIM],
}

impl Board {
    fn new() -> Self {
        Board {
            items: [0; DIM * DIM],
        }
    }

    fn position(&self, row: usize, col: usize) -> usize {
        DIM * row + col
    }

    fn apply(&mut self, line: Line) {
        debug!("Applying {}:", line);
        let get_all_points = line.get_all_points();
        for point in get_all_points {
            debug!("- {}", point);
            let pos = self.position(
                usize::try_from(point.y).unwrap(),
                usize::try_from(point.x).unwrap(),
            );
            self.items[pos] = self.items[pos] + 1;
        }

        debug!("{}", self);
    }

    fn count_crossings(&self) -> usize {
        self.items.iter().filter(|&n| *n > 1).count()
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output = String::new();
        writeln!(&mut output)?;
        for line in 0..DIM {
            for cols in 0..DIM {
                let item = self.items[self.position(line, cols)];
                let item_as_str = match item {
                    0 => String::from("."),
                    _ => format!("{}", u8::min(item, 9)),
                };
                write!(&mut output, "{}", item_as_str)?;
            }
            writeln!(&mut output)?;
        }
        writeln!(&mut output)?;

        write!(f, "{}", output)
        // write!(f, "{}", 2)
    }
}

#[derive(Clone, Debug)]
pub struct Simulation {
    lines: Vec<Line>,
    board: Board,
}

impl Simulation {
    pub fn new(input: String) -> Simulation {
        Simulation {
            lines: input
                .lines()
                .map(Line::from_str)
                .map(|res| res.unwrap())
                .collect(),
            board: Board::new(),
        }
    }

    fn steps(&mut self) {
        loop {
            if let Some(line) = self.lines.pop() {
                self.step(line);
            } else {
                break;
            }
        }
    }

    fn step(&mut self, line: Line) {
        debug!("step ...");
        self.board.apply(line);
        debug!("step ... done");
    }

    pub fn run(&mut self) -> Result<(), ()> {
        info!("Running simulation now");

        self.steps();
        info!("Board after applying vectors:\n{}", self.board);

        info!(
            "{} points where vectors cross",
            self.board.count_crossings()
        );

        info!("Simulation is done");
        Ok(())
    }
}

impl fmt::Display for Simulation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let steps_left = self.lines.len();
        let status = match steps_left {
            0 => format!("Simulation ended, see result below"),
            a => format!("{} steps left", a),
        };

        if steps_left > 0 {
            debug!("{:?}", self.board);
        }

        write!(f, "{}", status)
    }
}
