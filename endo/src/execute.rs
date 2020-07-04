use crate::dna::DNA;

pub fn execute(dna: &mut DNA, mut rna_sink: impl FnMut(DNA)) {
    while step(dna, &mut rna_sink).is_ok() {
        // do nothing
    }
}

fn step(dna: &mut DNA, rna_sink: &mut impl FnMut(DNA)) -> Result<(), ()> {
    let p = pattern(dna, rna_sink)?;
    let t = template(dna, rna_sink)?;
    // TODO: matchreplace
    Ok(())
}

fn pattern(dna: &mut DNA, rna_sink: &mut impl FnMut(DNA)) -> Result<DNA, ()> {
    rna_sink("ICFPICFPI".into());
    Ok(DNA::default())
}

fn template(dna: &mut DNA, rna_sink: &mut impl FnMut(DNA)) -> Result<DNA, ()> {
    rna_sink("CFPICFPIC".into());
    Err(())
}
