use std::{
    fmt::{self, Display},
    ops::{Index, IndexMut},
};

use noise::NoiseFn;
use rand::prelude::*;

pub struct Board {
    height: usize,
    width: usize,
    pixels: Vec<bool>,
}

impl Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for cell in &self.pixels {
            if *cell {
                write!(f, "#")?;
            } else {
                write!(f, " ")?;
            }
        }

        Ok(())
    }
}

impl Index<(usize, usize)> for Board {
    type Output = bool;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        self.pixels.get(y * self.width + x).unwrap_or(&false)
    }
}

impl IndexMut<(usize, usize)> for Board {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        self.pixels.get_mut(y * self.width + x).unwrap()
    }
}

impl Board {
    pub fn with_dimensions(x: usize, y: usize) -> Self {
        Self {
            height: y,
            width: x,
            pixels: vec![false; x * y],
        }
    }

    pub fn generate(&mut self) {
        let p = noise::Perlin::new();
        let mut rng = rand::thread_rng();
        let (x_offset, y_offset, z_offset): (f64, f64, f64) = rng.gen();

        for x in 0..self.width {
            let fractional_x = x as f64 / self.width as f64 * 10. + x_offset;
            for y in 0..self.height {
                let fractional_y = y as f64 / self.height as f64 * 10. + y_offset;

                self[(x, y)] = p.get([fractional_x, fractional_y, z_offset]) > 0.75;
            }
        }
    }

    fn neighbors(&self, x: usize, y: usize) -> u8 {
        let mut alive = 0;

        let (left, top, right, bottom) =
            (x == 0, y == 0, x == self.width - 1, y == self.height - 1);

        if !left && !top && self[(x - 1, y - 1)] {
            alive += 1;
        }

        if !top && self[(x, y - 1)] {
            alive += 1;
        }

        if !right && !top && self[(x + 1, y - 1)] {
            alive += 1;
        }

        if !right && self[(x + 1, y)] {
            alive += 1;
        }

        if !right && !bottom && self[(x + 1, y + 1)] {
            alive += 1;
        }

        if !bottom && self[(x, y + 1)] {
            alive += 1;
        }

        if !left && !bottom && self[(x - 1, y + 1)] {
            alive += 1;
        }

        if !left && self[(x - 1, y)] {
            alive += 1;
        }

        alive
    }

    pub fn update(&mut self) {
        for x in 0..self.width {
            for y in 0..self.height {
                self[(x, y)] = match (self[(x, y)], self.neighbors(x, y)) {
                    (true, 2) | (_, 3) => true,
                    _ => false,
                };
            }
        }
    }
}
