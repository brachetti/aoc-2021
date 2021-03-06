use std::{
    cmp::{max, min},
    collections::{HashMap, HashSet},
    fmt,
    result::Result,
};

use log::{debug, info};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
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
        let x_same = self.x == other.x;
        let y = max(self.y, other.y) - min(self.y, other.y);
        let y_same = self.y == other.y;

        (x_same && y == 1) || (x == 1 && y_same)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Location {
    value: usize,
    is_lowest: bool,
    point: Point,
}

impl Location {
    fn new(value: usize, x: usize, y: usize) -> Self {
        let point = Point::new(x, y);
        Location {
            value,
            point,
            is_lowest: false,
        }
    }

    fn with_values(location: &Location, neighbour_values: Vec<usize>) -> Self {
        let value = location.value;
        let point = location.point;
        let is_lowest = Self::check_lowest(value, neighbour_values);
        Location {
            value,
            point,
            is_lowest,
        }
    }

    fn find_neighbours(&self, list: &[Location]) -> Vec<usize> {
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

    fn find_basin_neighbours(&self, list: &[Location]) -> Vec<Location> {
        let mut neighbours = Vec::new();
        for location in list {
            if self.is_neighbour_in_basin_of(location) {
                neighbours.push(*location);
            }
            if neighbours.len() == 4 {
                break;
            }
        }
        neighbours
    }

    fn is_neighbour_of(&self, point: &Point) -> bool {
        self.point.is_neighbour_of(point)
    }

    fn is_neighbour_in_basin_of(&self, other: &Location) -> bool {
        other.value < 9 && other.is_neighbour_of(&self.point)
    }

    fn check_lowest(value: usize, neighbour_values: Vec<usize>) -> bool {
        for other in &neighbour_values {
            if value >= *other {
                return false;
            }
        }
        true
    }

    fn is_lowest(&self) -> bool {
        self.is_lowest
    }

    fn risk_level(&self) -> usize {
        self.value + 1
    }
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.value == 9 {
            write!(f, " ")
        } else {
            write!(f, "{}", self.value)
        }
    }
}

#[derive(Clone, Debug)]
pub struct Simulation {
    locations: Vec<Location>,
    basins: HashMap<Point, HashSet<Location>>,
    columns: usize,
}

impl Simulation {
    pub fn new(original_input: String) -> Self {
        let mut locations: Vec<Location> = Vec::new();
        let mut x = 0;
        let mut columns = 0;
        let basins = HashMap::new();
        debug!("\n{}", original_input);
        for (y, line) in original_input.lines().enumerate() {
            for val in line.chars() {
                let value: u32 = val.to_digit(10).unwrap();
                let location = Location::new(value as usize, x, y);
                locations.push(location);
                x += 1;
            }
            columns = x;
            x = 0;
        }
        Simulation {
            locations,
            columns,
            basins,
        }
    }

    fn init_locations(&mut self) {
        self.locations = self
            .locations
            .iter()
            .map(|l| {
                let values = l.find_neighbours(&self.locations);
                Location::with_values(l, values)
            })
            // .map(|l| *l)
            .collect()
    }

    fn get_risk_levels(&self) {
        let sum: usize = self
            .locations
            .iter()
            .filter(|l| l.is_lowest())
            .map(|l| l.risk_level())
            .sum();
        info!("Sum of risk levels is {}", sum);
    }

    fn find_basins(&mut self) {
        let mut basins: HashMap<Point, HashSet<Location>> = HashMap::new();
        self.locations
            .iter()
            .filter(|l| l.is_lowest())
            .for_each(|l| {
                let basin = self.add_to_basin(HashSet::new(), vec![*l]);
                basins.insert(l.point, basin);
            });

        self.basins = basins
    }

    /**
     * Iteratively checks all locations for neighbours, that would be part of the basin.
     */
    fn add_to_basin(
        &self,
        mut basin: HashSet<Location>,
        to_check: Vec<Location>,
    ) -> HashSet<Location> {
        // can be at most that many
        let max = to_check.len() * 4;
        let mut additional: Vec<Location> = Vec::with_capacity(max);
        // end-recursive return
        if to_check.is_empty() {
            return basin;
        }
        basin.reserve(to_check.len());
        for location in to_check {
            // succesively push the locations into the known space
            if !basin.insert(location) {
                // location was already known, do not double check
                continue;
            }
            // for that location, find the potential neighbours
            let mut l_neighbours = location.find_basin_neighbours(&self.locations);
            additional.append(&mut l_neighbours);
        }

        self.add_to_basin(basin, additional)
    }

    fn get_basin_sizes(&self) {
        let mut basin_sizes: Vec<usize> = self.basins.iter().map(|(_, set)| set.len()).collect();
        basin_sizes.sort_unstable();
        let product: usize = basin_sizes.iter().rev().take(3).copied().product();
        info!("Product of 3 largest basin sizes levels is {}", product);
    }

    pub fn run(&mut self) -> Result<(), ()> {
        info!("Running Simulation");
        self.init_locations();
        debug!("{}", self);
        self.get_risk_levels();
        self.find_basins();
        self.get_basin_sizes();
        info!("Done running Simulation");
        Ok(())
    }
}

impl fmt::Display for Simulation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut o = String::from("\n");
        let mut column = 0;
        for location in &self.locations {
            o.push_str(&location.to_string());
            column += 1;
            if column == self.columns {
                o.push('\n');
                column = 0;
            }
        }
        writeln!(f, "{}", o)
    }
}
