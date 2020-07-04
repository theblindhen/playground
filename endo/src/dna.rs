use im::vector::Vector;
use std::fmt;

#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Base {
    I,
    C,
    F,
    P,
}

impl From<char> for Base {
    fn from(c: char) -> Self {
        match c {
            'I' => Base::I,
            'C' => Base::C,
            'F' => Base::F,
            'P' => Base::P,
            _ => panic!("Non-ICFP base '{}'", c),
        }
    }
}

impl From<Base> for char {
    fn from(b: Base) -> Self {
        match b {
            Base::I => 'I',
            Base::C => 'C',
            Base::F => 'F',
            Base::P => 'P',
        }
    }
}

#[derive(Default, Clone, PartialEq, Eq)]
pub struct DNA {
    v: Vector<Base>,
}

impl From<&str> for DNA {
    fn from(s: &str) -> Self {
        let mut v = Vector::new();
        for c in s.chars() {
            v.push_back(c.into());
        }
        DNA { v }
    }
}

impl fmt::Debug for DNA {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for &b in &self.v {
            let c: char = b.into();
            write!(f, "{}", c)?;
        }
        Ok(())
    }
}

impl DNA {
    pub fn len(&self) -> usize {
        self.v.len()
    }

    pub fn pop(&mut self) -> Option<Base> {
        self.v.pop_front()
    }

    pub fn peek(&mut self) -> Option<Base> {
        self.v.front().map(|b| *b)
    }

    pub fn drop(&mut self, count: usize) {
        self.v = self.v.skip(count.min(self.v.len()));
    }

    pub fn prepend(&mut self, b: Base) {
        self.v.push_front(b);
    }

    pub fn append(&mut self, b: Base) {
        self.v.push_back(b);
    }

    pub fn concat(&mut self, rhs: Self) {
        self.v.append(rhs.v);
    }

    /// Indexes are 0-based, and end is not inclusive
    pub fn subseq(&self, start: usize, end: usize) -> Self {
        let end = end.min(self.v.len());
        let start = start.min(end);
        DNA {
            v: self.v.clone().slice(start..end),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_drop() {
        let mut dna: DNA = "ICFP".into();
        dna.drop(2);
        assert_eq!(dna, "FP".into());
        dna.drop(0);
        assert_eq!(dna, "FP".into());
        dna.drop(3);
        assert_eq!(dna, "".into());
    }

    #[test]
    fn test_subseq() {
        let mut dna: DNA = "ICFP".into();
        assert_eq!(dna.subseq(0, 1), "I".into());
        assert_eq!(dna.subseq(1, 3), "CF".into());
        assert_eq!(dna.subseq(3, 5), "P".into());
        assert_eq!(dna.subseq(5, 3), "".into());
        assert_eq!(dna.subseq(3, 3), "".into());
        assert_eq!(dna.subseq(3, 0), "".into());
    }
}
