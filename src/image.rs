use std::{
    fs::{self, File},
    io::{Error, Write},
    path::Path,
};

use crate::color::Color;

pub trait Image {
    fn to_file(&self, fname: &str) -> Result<(), Error>;

    fn to_stdout(&self);
}

#[derive(Debug)]
pub struct Ppm {
    pub width: u32,  // x
    pub height: u32, // y
    max_color: u32,
    pixels: Vec<Vec<Color>>,
}

impl Ppm {
    pub fn new() -> Ppm {
        Ppm {
            width: 200,
            height: 100,
            max_color: 255,
            pixels: vec![vec![Color::default(); 200]; 100],
        }
    }

    pub fn plot(&mut self, x: usize, y: usize, color: Color) {
        self.pixels[x][y] = color
    }
}

impl Image for Ppm {
    fn to_file(&self, fname: &str) -> Result<(), Error> {
        let mut file = File::create(Path::new(fname))?;

        write!(
            file,
            "P3\n{:?} {:?}\n{:?}\n",
            self.width, self.height, self.max_color
        )?;

        for j in 1..=self.height {
            let j = self.height - j;
            for i in 0..self.width {
                let color = self.pixels[j as usize][i as usize];
                writeln!(file, "{color}")?;
            }
        }
        Ok(())
    }

    fn to_stdout(&self) {
        print!(
            "P3\n{:?} {:?}\n{:?}\n",
            self.width, self.height, self.max_color
        );

        for j in 1..=self.height {
            let j = self.height - j;
            for i in 0..self.width {
                let color = self.pixels[j as usize][i as usize];
                println!("{color}");
            }
        }
    }
}
