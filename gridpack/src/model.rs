use crate::bits2d::Bits2D;
use std::fmt;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Shape(Bits2D);

impl fmt::Display for Shape {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data = &self.0;
        for line in 1..=data.length2() {
            let y = data.length2() - line;
            for x in 0..data.length1() {
                write!(f, "{}", if data[(x, y)] { '*' } else { ' ' })?
            }
            writeln!(f, "")?
        }
        Ok(())
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Grid(Bits2D);

impl Grid {
    pub fn new(width: u32, height: u32) -> Self {
        Grid(Bits2D::new(width, height))
    }

    pub fn try_place(&mut self, shape: &Shape, x_offset: u32, y_offset: u32) -> bool {
        // Bounds check
        if shape.0.length1() + x_offset > self.0.length1() {
            return false;
        }
        if shape.0.length2() + y_offset > self.0.length2() {
            return false;
        }
        // Check availability. Don't mutate anything here, allowing us to back out.
        for y in 0..shape.0.length2() {
            for x in 0..shape.0.length1() {
                // already occupied?
                if shape.0[(x, y)] && self.0[(x_offset + x, y_offset + y)] {
                    return false;
                }
            }
        }
        // Make the change.
        for y in 0..shape.0.length2() {
            for x in 0..shape.0.length1() {
                self.0.assign(x_offset + x, y_offset + y, shape.0[(x, y)]);
            }
        }
        true
    }

    pub fn must_place(&mut self, shape: &Shape, x_offset: u32, y_offset: u32) {
        if !self.try_place(shape, x_offset, y_offset) {
            panic!("Tried to place on occupied field")
        }
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data = &self.0;
        write!(f, " ")?;
        for x in 0..data.length1() {
            write!(f, "-")?
        }
        writeln!(f, " ")?;

        for line in 1..=data.length2() {
            let y = data.length2() - line;
            write!(f, "|")?;
            for x in 0..data.length1() {
                write!(f, "{}", if data[(x, y)] { '*' } else { ' ' })?
            }
            writeln!(f, "|")?
        }

        write!(f, " ")?;
        for x in 0..data.length1() {
            write!(f, "-")?
        }
        writeln!(f, " ")?;

        Ok(())
    }
}

pub mod shapes {
    use super::*;

    pub fn stair_step() -> Shape {
        let mut data = Bits2D::new(3, 2);
        data.set(0, 0);
        data.set(1, 0);
        data.set(1, 1);
        data.set(2, 1);
        Shape(data)
    }

    pub fn ell() -> Shape {
        let mut data = Bits2D::new(2, 3);
        data.set(0, 0);
        data.set(1, 0);
        data.set(0, 1);
        data.set(0, 2);
        Shape(data)
    }
}
