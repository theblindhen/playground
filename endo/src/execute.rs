use crate::dna::{Base, DNA};

/// The `dna` value is consumed since the implementation does not follow the
/// specification about mutation of `dna` in the step where the program ends.
pub fn execute(mut dna: DNA, mut rna_sink: impl FnMut(DNA)) {
    // TODO: these lines are just for testing. Remove soon.
    rna_sink("ICFPICFPI".into());
    rna_sink("CFPICFPIC".into());

    while step(&mut dna, &mut rna_sink).is_ok() {
        // do nothing
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Finish;

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

fn step(dna: &mut DNA, rna_sink: &mut dyn FnMut(DNA)) -> Result<(), Finish> {
    let p = pattern(dna, rna_sink)?;
    let t = template(dna, rna_sink)?;
    matchreplace(dna, p, t);
    Ok(())
}

/// May leave `dna` inconsistent when EOF reached
fn pattern(mut dna: &mut DNA, rna_sink: &mut dyn FnMut(DNA)) -> Result<Pattern, Finish> {
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
    Err(Finish)
}

/// MSB is last
fn nat(dna: &mut DNA) -> Result<usize, Finish> {
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
    Err(Finish)
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
fn template(mut dna: &mut DNA, rna_sink: &mut dyn FnMut(DNA)) -> Result<Template, Finish> {
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
    Err(Finish)
}

fn matchreplace(mut dna: &mut DNA, pattern: Pattern, template: Template) {
    let mut i : usize = 0;
    let mut env : Vec<DNA> = vec![];
    let mut c_rev : Vec<usize> = vec![];
    for p in pattern {
        match p {
            PItem::Base(b) => {
                match dna.at(i) {
                    b => { i += 1 },
                    _ => return
                }
            },
            PItem::Skip(n) => {
                i += n;
                if i > dna.len() {
                    return
                }
            },
            PItem::Search(s) => {
                match dna.find_first(&s, i) {
                    None => return,
                    Some(idx) => i = idx,
                }
            },
            PItem::Open() => {
                c_rev.push(i)
            },
            PItem::Close() => {
                let from =  c_rev.pop().unwrap();
                env.push(dna.subseq(from, i))
            }
        }
    }
    let mut r = replace(template, env);
    let tail = dna.subseq(i, dna.len());
    dna.assign(r);
    dna.concat(tail)
}

fn replace(template: Template, env : Vec<DNA>) -> DNA {
    let mut r = DNA::default();
    for t in template {
        match t {
            TItem::Base(b) => r.append(b),
            TItem::Ref{n, l} => {
                r.concat(protect(l, env[n].clone()))
            },
            TItem::RefLen(n) => {
                r.concat(asnat(env[n].len()))
            }
        }
    }
    r
}

fn protect(l: usize, d: DNA) -> DNA {
    if l == 0 {
        d
    } else {
        protect(l-1, quote(d))
    }
}

fn quote(d: DNA) -> DNA {
    let mut r = DNA::default();
    for b in d {
        match b {
            Base::I => r.append(Base::C),
            Base::C => r.append(Base::F),
            Base::F => r.append(Base::P),
            Base::P => {
                r.append(Base::I);
                r.append(Base::C)
            }
        }
    }
    r
}


fn asnat(mut n: usize) -> DNA {
    let mut r = DNA::default();
    while n > 0 {
        if n % 2 == 0 { // Even
            r.append(Base::I);
        } else {
            r.append(Base::C);
        }
        n /= 2;
    }
    r.append(Base::P);
    r
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
        assert_eq!(template(&mut "".into(), &mut noop), Err(Finish));

        assert_eq!(
            template(&mut "IF(P,CP) IIP(ICP) IIF".into(), &mut noop),
            Ok(vec![TItem::Ref { l: 0, n: 1 }, TItem::RefLen(2)])
        );

        let mut rna = vec![];
        let t = template(&mut "C III(ICFPICF) F IIC".into(), &mut |x| rna.push(x));
        assert_eq!(t, Ok(vec![TItem::Base(Base::I), TItem::Base(Base::C)]));
        assert_eq!(rna, vec!["ICFPICF".into()]);
    }

    #[test]
    fn test_asnat() {
        assert_eq!(nat(&mut asnat(0)), Ok(0));
        assert_eq!(nat(&mut asnat(1)), Ok(1));
        assert_eq!(nat(&mut asnat(2)), Ok(2));
        assert_eq!(nat(&mut asnat(9)), Ok(9));
        assert_eq!(nat(&mut asnat(9384)), Ok(9384));
    }

    #[test]
    fn test_quote() {
        assert_eq!(quote("ICFP".into()), "CFPIC".into());
        assert_eq!(quote("IICCFFPP".into()), "CCFFPPICIC".into());
    }

    #[test]
    fn test_protect() {
        assert_eq!(protect(0, "ICFP".into()), "ICFP".into());
        let l1 = protect(1, "ICFP".into());
        assert_eq!(l1, "CFPIC".into());
        assert_eq!(protect(2, "ICFP".into()), protect(1, l1));
        assert_eq!(protect(2, "ICFP".into()), "FPICCF".into());
    }

    #[test]
    fn test_step() {
        let mut dna : DNA = "IIPIPICPIICICIIFICCIFPPIICCFPC".into();
        let mut rna_sink = |rna| ();
        step(&mut dna, &mut rna_sink);
        assert_eq!(dna, "PICFC".into());
        
        let mut dna : DNA = "IIPIPICPIICICIIFICCIFCCCPPIICCFPC".into();
        let mut rna_sink = |rna| ();
        step(&mut dna, &mut rna_sink);
        assert_eq!(dna, "PIICCFCFFPC".into());
        
        let mut dna : DNA = "IIPIPIICPIICIICCIICFCFC".into();
        let mut rna_sink = |rna| ();
        step(&mut dna, &mut rna_sink);
        assert_eq!(dna, "I".into());
    }
}
