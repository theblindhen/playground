// Deserializing
mod ints;
    
use serde::Deserialize;

use structopt::StructOpt;
use std::path::PathBuf;
use std::io::BufReader;
use std::fs::File;
use std::str::FromStr;

use num_bigint::BigInt;

// Struct for command line parsing 
#[derive(StructOpt, Debug)]
#[structopt()]
struct MyOpt {
    // The number of occurrences of the `v/verbose` flag
    /// Verbose mode (-v, -vv, -vvv, etc.)
    #[structopt(short = "v", long = "verbose", parse(from_occurrences))]
    verbose: u8,

    /// Run in interactive mode
    #[structopt(short, long)]
    interactive: bool,

    #[structopt(default_value = "1000", short, long)]
    timeout: u32,

    /// Input file
    #[structopt(name = "MAP", default_value = "numbers.json", parse(from_os_str))]
    file: PathBuf,
}

// Parse a json file on disk
#[derive(Deserialize, Debug)]
struct Gcd {
    a: String,
    b: String,
}

fn file_json(path: PathBuf) -> Result<Vec<Gcd>, Box<dyn std::error::Error>> {

    let file = File::open(path)?;
    let reader = BufReader::new(file);
    // Read the JSON contents of the file as an instance of `User`.
    let json = serde_json::from_reader(reader)?;
    Ok(json)
}

fn main() {
    // Parse command line arguments according to the struct
    let opt = MyOpt::from_args();

    match file_json(opt.file) {
        Err(e) => println!("\nThere was an error with reading the JSON file:\n{:#?}", e),
        Ok(gcds) => {
            println!("a = {},\nb = {}", gcds[0].a, gcds[0].b);
            let a = BigInt::from_str(&gcds[0].a).unwrap();
            let b = BigInt::from_str(&gcds[0].b).unwrap();
            let gcd = ints::big_ea(&a, &b);
            println!("gcd = {}", &gcd);
            ()
        }
    }
}
