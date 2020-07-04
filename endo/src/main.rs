#![allow(unused)]

mod dna;
mod execute;

use dna::DNA;

fn main() {
    println!("Hello, world!");
    let mut dna: DNA = "ICFP".into();
    execute::execute(&mut dna, |dna| println!("Got some RNA: {:?}", dna))
}
