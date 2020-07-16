#![allow(unused)]

mod dna;
mod execute;

use std::io::prelude::*;
use structopt::StructOpt;
use std::path::PathBuf;
use std::fs::File;

use dna::DNA;

use crossbeam_channel::unbounded;

// Struct for command line parsing 
#[derive(StructOpt, Debug)]
#[structopt()]
struct MyOpt {
    #[structopt(name = "DNA", default_value = "numbers.json", parse(from_os_str))]
    dna: PathBuf,
}

fn main() {
    // Parse command line arguments according to the struct
    let opt = MyOpt::from_args();

    // println!("Hello, world!");
    //TODO: Read from the zip-file directly
    let mut file = File::open(opt.dna).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read the file");
    let mut dna: DNA = DNA::from(contents.as_str());

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

    execute::execute(dna, |chunk| s.send(Some(chunk)).unwrap());
    s.send(None).unwrap();
    thr.join();
}
