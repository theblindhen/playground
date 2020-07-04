use crate::dna::{Base, DNA};

pub fn execute(dna: &mut DNA, mut rna_sink: impl FnMut(DNA)) {
    while step(dna, &mut rna_sink).is_ok() {
        // do nothing
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
enum PItem {
    Base(Base),
    Skip(usize),
    Search(DNA),
    Open(),
    Close(),
}

type Pattern = Vec<PItem>;

#[derive(Clone, PartialEq, Eq, Debug)]
enum TItem {
    Base(Base),
    Ref(usize, usize),
    RefLen(usize),
}

type Template = Vec<TItem>;

fn step(dna: &mut DNA, rna_sink: &mut dyn FnMut(DNA)) -> Result<(), ()> {
    let p = pattern(dna, rna_sink)?;
    let t = template(dna, rna_sink)?;
    matchreplace(p, t);
    Ok(())
}

/// May leave `dna` inconsistent when EOF reached
fn pattern(mut dna: &mut DNA, rna_sink: &mut dyn FnMut(DNA)) -> Result<Pattern, ()> {
    let mut p = vec![];
    let mut lvl: usize = 0;
    loop {
        match dna.pop() {
            Some(Base::C) => p.push(PItem::Base(Base::I)),
            Some(Base::F) => p.push(PItem::Base(Base::C)),
            Some(Base::P) => p.push(PItem::Base(Base::F)),
            Some(Base::I) => match dna.pop() {
                Some(Base::C) => p.push(PItem::Base(Base::P)),
                Some(Base::P) => {
                    let n = nat(&mut dna)?;
                    p.push(PItem::Skip(n));
                }
                Some(Base::F) => {
                    dna.pop(); // quirk of the specification
                    let s = consts(&mut dna);
                    p.push(PItem::Search(s));
                }
                Some(Base::I) => match dna.pop() {
                    Some(Base::P) => {
                        lvl += 1;
                        p.push(PItem::Open())
                    }
                    Some(Base::C) | Some(Base::F) => {
                        if lvl == 0 {
                            return Ok(p);
                        } else {
                            lvl = lvl - 1;
                            p.push(PItem::Close());
                        }
                    }
                    Some(Base::I) => {
                        rna_sink(dna.subseq(0, 7));
                        dna.drop(7);
                    }
                    None => break,
                },
                None => break,
            },
            None => break,
        }
    }
    Err(())
}

/// MSB is last
fn nat(dna: &mut DNA) -> Result<usize, ()> {
    let mut shiftcount = 0;
    let mut acc = 0;
    while let Some(b) = dna.pop() {
        match b {
            Base::P => return Ok(acc),
            Base::I | Base::F => (), // `|=` with 0 is a no-op
            Base::C => acc |= (1 << shiftcount),
        }
        shiftcount += 1;
    }
    Err(())
}

fn consts(dna: &mut DNA) -> DNA {
    let mut acc = DNA::default();
    while let Some(b) = dna.pop() {
        match b {
            Base::C => acc.append(Base::I),
            Base::F => acc.append(Base::C),
            Base::P => acc.append(Base::F),
            Base::I => match dna.pop() {
                Some(Base::C) => acc.append(Base::P),
                Some(b) => {
                    // Went too far by two
                    dna.prepend(b);
                    dna.prepend(Base::I);
                    return acc;
                }
                None => {
                    // Went too far by one
                    dna.prepend(Base::I);
                    return acc;
                }
            },
        }
    }
    acc
}

fn template(dna: &mut DNA, rna_sink: &mut dyn FnMut(DNA)) -> Result<Template, ()> {
    // TODO
    rna_sink("CFPICFPIC".into());
    Err(())
}

fn matchreplace(pattern: Pattern, template: Template) {
    // TODO
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_nat() {
        assert_eq!(nat(&mut "P".into()), Ok(0));
        assert_eq!(nat(&mut "IP".into()), Ok(0));
        assert_eq!(nat(&mut "CP".into()), Ok(1));
        assert_eq!(nat(&mut "ICFCP".into()), Ok(2 | 8));
        assert_eq!(nat(&mut "ICFCIIIIP".into()), Ok(2 | 8));
        assert_eq!(nat(&mut "CIICICP".into()), Ok(1 | 8 | 32));
    }

    fn test_consts() {
        assert_eq!(consts(&mut "".into()), "".into());

        let mut dna = "CFPIC".into();
        assert_eq!(consts(&mut dna), "ICFP".into());
        assert_eq!(dna, "".into());

        // Test replacement of one base
        let mut dna = "CFI".into();
        assert_eq!(consts(&mut dna), "IC".into());
        assert_eq!(dna, "I".into());

        // Test replacement of two bases at EOF
        let mut dna = "CFIF".into();
        assert_eq!(consts(&mut dna), "IC".into());
        assert_eq!(dna, "IF".into());

        // Test replacement of two bases before EOF
        let mut dna = "CFIPC".into();
        assert_eq!(consts(&mut dna), "IC".into());
        assert_eq!(dna, "IFC".into());
    }

    #[test]
    fn test_pattern() {
        fn noop(_: DNA) {}

        // Test 1 from spec
        assert_eq!(
            pattern(&mut "CIIC".into(), &mut noop),
            Ok(vec![PItem::Base(Base::I)])
        );

        // Test 2 from spec
        assert_eq!(
            pattern(&mut "IIP IPICP IIC IC IIF".into(), &mut noop),
            Ok(vec![
                PItem::Open(),
                PItem::Skip(2),
                PItem::Close(),
                PItem::Base(Base::P)
            ])
        );
    }
}
