use std::{
    cmp::{max, min},
    result::Result,
};

#[derive(Copy, Clone, Debug)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Point { x, y }
    }

    fn is_neighbour_of(&self, other: &Point) -> bool {
        let x = max(self.x, other.x) - min(self.x, other.x);
        let y = max(self.y, other.y) - min(self.y, other.y);

        (x == 0 && y == 1) || (x == 1 && y == 0)
    }
}

#[derive(Clone, Debug)]
struct Location {
    value: u8,
    is_lowest: bool,
    point: Point,
}

impl Location {
    fn new(value: u8, x: usize, y: usize) -> Self {
        let point = Point::new(x, y);
        Location {
            value,
            point,
            is_lowest: false,
        }
    }

    fn with_values(location: &Location, neighbour_values: Vec<u8>) -> Self {
        let value = location.value;
        let point = location.point;
        let is_lowest = Self::is_lowest(value, neighbour_values);
        Location {
            value,
            point,
            is_lowest,
        }
    }

    fn find_neighbours(&self, list: &Vec<Location>) -> Vec<u8> {
        let mut neighbours = Vec::new();
        for location in list {
            if location.is_neighbour_of(&self.point) {
                neighbours.push(location);
            }
            if neighbours.len() == 4 {
                break;
            }
        }
        neighbours.iter().map(|location| location.value).collect()
    }

    fn is_neighbour_of(&self, point: &Point) -> bool {
        self.point.is_neighbour_of(point)
    }

    fn is_lowest(value: u8, neighbour_values: Vec<u8>) -> bool {
        for other in &neighbour_values {
            if value >= *other {
                return false
            }
        }
        true
    }
}

#[derive(Clone, Debug)]
pub struct Simulation {
    locations: Vec<Location>,
}

impl Simulation {
    pub fn new(original_input: String) -> Self {
        let mut input = Vec::new();
        let mut x = 0;
        let mut y = 0;
        for line in original_input.lines() {
            for value in line.as_bytes() {
                input.push(Location::new(*value, x, y));
                x += 1;
            }
            y += 1;
        }
        Simulation { locations: input }
    }

    fn init_locations(&mut self) {
        self.locations = self.locations
            .iter()
            .map(|l| {
                let values = l.find_neighbours(&self.locations);
                Location::with_values(l, values)
            })
            .collect()
    }

    pub fn run(&mut self) -> Result<(), ()> {
        self.init_locations();
        Ok(())
    }
}
