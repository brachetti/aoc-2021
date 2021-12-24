use std::fmt;

use quadtree_rs::Quadtree;

#[derive(Clone, Copy, Debug)]
struct Dark;
#[derive(Clone, Copy, Debug)]
struct Light;

#[derive(Clone, Copy, Debug)]
enum Pixel {
    Dark,
    Light,
}

#[derive(Debug)]
pub(crate) struct Image {
    internal: Quadtree<usize, Pixel>,
    current_depth: usize,
    input_image: String,
}

impl Image {
    fn new(depth: usize) -> Self {
        let current_depth = depth;
        let internal = Quadtree::new(current_depth);
        let input_image = String::new();

        Self {
            internal,
            current_depth,
            input_image,
        }
    }

    pub(crate) fn with_starting_image(input: String) -> Self {
        let len: usize = input.lines().count();
        let mut image = Self::new(len);
        image.init_image(input.lines().collect());

        image
    }

    fn init_image(&mut self, input: Vec<&str>) {
        for (x, line) in input.iter().enumerate() {
            for (y, character) in line.chars().enumerate() {
                if character == '#' {
                    self.internal.insert_pt((x, y).into(), Pixel::Light);
                }
            }
        }
    }
}

impl fmt::Display for Image {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut o = String::from("\n");
        let width = self.internal.width();
        for location in self.internal.values() {}
        writeln!(f, "{}", o)
    }
}
