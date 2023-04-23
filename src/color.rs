use crate::vec::Vec;
use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
pub struct Color {
    coeff: [u32; 3],
}

impl Color {
    pub fn r(&self) -> u32 {
        self.at(0)
    }

    pub fn g(&self) -> u32 {
        self.at(1)
    }

    pub fn b(&self) -> u32 {
        self.at(2)
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
            (255.99 * value.x().sqrt()) as u32,
            (255.99 * value.y().sqrt()) as u32,
            (255.99 * value.z().sqrt()) as u32,
        ])
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {:?} {:?}", self.r(), self.g(), self.b())
    }
}
