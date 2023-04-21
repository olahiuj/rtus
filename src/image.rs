use std::{
    fs::File,
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
    pub width: usize,  // x
    pub height: usize, // y
    max_color: u32,
    pixels: Vec<Vec<Color>>,
}

impl Ppm {
    const DEFAULT_WIDTH: usize = 400;
    const DEFAULT_ASPECT_RATIO: f32 = 16. / 9.;

    pub fn new() -> Ppm {
        let width = Ppm::DEFAULT_WIDTH;
        let height = (Ppm::DEFAULT_WIDTH as f32 / Ppm::DEFAULT_ASPECT_RATIO) as usize;
        Ppm {
            width,
            height,
            max_color: 255,
            pixels: vec![vec![Color::default(); width]; height],
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
                let color = self.pixels[j][i];
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
                let color = self.pixels[j][i];
                println!("{color}");
            }
        }
    }
}
