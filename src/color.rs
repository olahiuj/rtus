use std::fmt::Display;

use crate::vec::Vec;

#[derive(Debug, Clone, Copy)]
pub struct Color {
    coeff: [u32; 3],
}

impl Color {
    pub fn new() -> Color {
        Color { coeff: [0, 0, 0] }
    }

    pub fn r_mut(&mut self) -> &mut u32 {
        self.at_mut(0)
    }

    pub fn g_mut(&mut self) -> &mut u32 {
        self.at_mut(1)
    }

    pub fn b_mut(&mut self) -> &mut u32 {
        self.at_mut(2)
    }

    pub fn r(&self) -> u32 {
        self.at(0)
    }

    pub fn g(&self) -> u32 {
        self.at(1)
    }

    pub fn b(&self) -> u32 {
        self.at(2)
    }

    pub fn at_mut(&mut self, index: usize) -> &mut u32 {
        &mut self.coeff[index]
    }

    pub fn at(&self, index: usize) -> u32 {
        self.coeff[index]
    }
}

impl Default for Color {
    fn default() -> Self {
        Color::from([0, 0, 0])
    }
}

impl From<[u32; 3]> for Color {
    fn from(value: [u32; 3]) -> Self {
        Color { coeff: value }
    }
}

impl From<Vec> for Color {
    fn from(value: Vec) -> Self {
        Color::from([
            (255.99 * value.at(0)) as u32,
            (255.99 * value.at(1)) as u32,
            (255.99 * value.at(2)) as u32,
        ])
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {:?} {:?}", self.r(), self.g(), self.b())
    }
}
