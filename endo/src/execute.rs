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
    Ref { n: usize, l: usize },
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
    let mut p = vec![]; // TODO: avoid allocation?
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

/// May leave `dna` inconsistent when EOF reached
fn template(mut dna: &mut DNA, rna_sink: &mut dyn FnMut(DNA)) -> Result<Template, ()> {
    let mut t = vec![]; // TODO: avoid allocation?
    loop {
        match dna.pop() {
            Some(Base::C) => t.push(TItem::Base(Base::I)),
            Some(Base::F) => t.push(TItem::Base(Base::C)),
            Some(Base::P) => t.push(TItem::Base(Base::F)),
            Some(Base::I) => match dna.pop() {
                Some(Base::C) => t.push(TItem::Base(Base::P)),
                Some(Base::F) | Some(Base::P) => {
                    let l = nat(&mut dna)?;
                    let n = nat(&mut dna)?;
                    t.push(TItem::Ref { n, l });
                }
                Some(Base::I) => match dna.pop() {
                    Some(Base::C) | Some(Base::F) => return Ok(t),
                    Some(Base::P) => {
                        let n = nat(&mut dna)?;
                        t.push(TItem::RefLen(n));
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

    #[test]
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
        assert_eq!(dna, "IPC".into());
    }

    fn noop(_: DNA) {}

    #[test]
    fn test_pattern() {
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

        // The IF pattern item is always followed by a base that's ignored. Then
        // comes a sequence of escaped bases followed by I[IFP] as a terminator,
        // which doubles as the start of the next pattern item!
        assert_eq!(
            pattern(&mut "IFC() IP(P) IIF".into(), &mut noop),
            Ok(vec![PItem::Search("".into()), PItem::Skip(0)])
        );

        // Same example as above but with non-trivial literals.
        assert_eq!(
            pattern(&mut "IFI(C F P IC) IP(CP) IIF".into(), &mut noop),
            Ok(vec![PItem::Search("ICFP".into()), PItem::Skip(1)])
        );

        let mut rna = vec![];
        let t = pattern(&mut "P III(ICFPICF) IC IIC".into(), &mut |x| rna.push(x));
        assert_eq!(t, Ok(vec![PItem::Base(Base::F), PItem::Base(Base::P)]));
        assert_eq!(rna, vec!["ICFPICF".into()]);
    }

    #[test]
    fn test_template() {
        assert_eq!(template(&mut "".into(), &mut noop), Err(()));

        assert_eq!(
            template(&mut "IF(P,CP) IIP(ICP) IIF".into(), &mut noop),
            Ok(vec![TItem::Ref { l: 0, n: 1 }, TItem::RefLen(2)])
        );

        let mut rna = vec![];
        let t = template(&mut "C III(ICFPICF) F IIC".into(), &mut |x| rna.push(x));
        assert_eq!(t, Ok(vec![TItem::Base(Base::I), TItem::Base(Base::C)]));
        assert_eq!(rna, vec!["ICFPICF".into()]);
    }
}
