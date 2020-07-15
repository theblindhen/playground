use im::vector::Vector;
use std::fmt;
use std::convert::{TryFrom, TryInto};

#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Base {
    I,
    C,
    F,
    P,
}

impl TryFrom<char> for Base {
    type Error = ();
    fn try_from(c: char) -> Result<Self, ()> {
        match c {
            'I' => Ok(Base::I),
            'C' => Ok(Base::C),
            'F' => Ok(Base::F),
            'P' => Ok(Base::P),
            _ => Err(()),
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
            // Ignore unknown characters
            if let Ok(b) = c.try_into() {
                v.push_back(b);
            }
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

    pub fn at(&self, index: usize) -> Option<Base> {
        self.v.get(index).map(|b| *b)
    }

    pub fn find_first(&self, needle: &DNA,  from: usize) -> Option<usize> {
        let mut needle = needle.v.focus();
        let mut haystack = self.v.focus();
        let mut needle_pos = 0;
        let mut haystack_pos = from;
        while haystack_pos <= haystack.len() { // Note: `<=` so we can handle EOF
            match needle.get(needle_pos) {
                Some(needle_base) => {
                    if Some(needle_base) == haystack.get(haystack_pos) {
                        haystack_pos += 1;
                        needle_pos += 1;
                    } else {
                        haystack_pos += 1;
                        haystack_pos -= needle_pos;
                        needle_pos = 0;
                    }
                }
                None => return Some(haystack_pos)
            }
        }
        None
    }

    pub fn assign(&mut self,  other: DNA) {
        self.v = other.v
    }
}

impl IntoIterator for DNA {
    type Item = Base;
    type IntoIter = im::vector::ConsumingIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.v.into_iter()
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

    #[test]
    fn test_find_first() {
        let mut dna: DNA = "I IC ICF ICF".into();
        assert_eq!(dna.find_first(&"".into(), 0), Some(0));

        assert_eq!(dna.find_first(&"C".into(), 0), Some(3));
        assert_eq!(dna.find_first(&"C".into(), 2), Some(3));
        assert_eq!(dna.find_first(&"C".into(), 3), Some(5));
        assert_eq!(dna.find_first(&"P".into(), 0), None);

        assert_eq!(dna.find_first(&"IC".into(), 0), Some(3));
        assert_eq!(dna.find_first(&"IC".into(), 1), Some(3));
        assert_eq!(dna.find_first(&"IC".into(), 2), Some(5));

        assert_eq!(dna.find_first(&"ICF".into(), 0), Some(6));
        assert_eq!(dna.find_first(&"ICF".into(), 4), Some(9));
        assert_eq!(dna.find_first(&"F".into(), 6), Some(9));
        assert_eq!(dna.find_first(&"CF".into(), 6), Some(9));
        assert_eq!(dna.find_first(&"FICF".into(), 0), Some(9));
        assert_eq!(dna.find_first(&"F".into(), 8), Some(9));
        assert_eq!(dna.find_first(&"F".into(), 9), None);
        assert_eq!(dna.find_first(&"F".into(), 10), None);
    }
}
