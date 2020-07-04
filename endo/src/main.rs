#![allow(unused)]

mod dna;
mod execute;

use dna::DNA;

use crossbeam_channel::unbounded;

fn main() {
    println!("Hello, world!");
    let mut dna: DNA = "ICFP".into();

    // Create a channel of unbounded capacity.
    let (s, r) = unbounded();

    let thr = std::thread::spawn(move || {
        loop {
            let orna = r.recv().unwrap();
            match orna {
                None => return,
                Some(dna) => println!("Got some RNA: {:?}", dna),
            }
        }
    });

    execute::execute(&mut dna, |dna| s.send(Some(dna)).unwrap());
    s.send(None).unwrap();
    thr.join();
}
