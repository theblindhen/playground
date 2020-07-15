// Deserializing
mod ints;
    
use serde::Deserialize;

use structopt::StructOpt;
use std::path::PathBuf;
use std::io::BufReader;
use std::fs::File;
use std::str::FromStr;

use std::time::Instant;

use num_bigint::BigInt;
use num_traits::Zero;

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

#[derive(Deserialize, Debug)]
struct JsonGcd {
    a: String,
    b: String,
}

struct Gcd {
    a: BigInt,
    b: BigInt
}

// TODO: Should be TryForm since it may fail
impl From<JsonGcd> for Gcd {
    fn from(g: JsonGcd) -> Self {
        Gcd { a: BigInt::from_str(&g.a).unwrap(),
              b: BigInt::from_str(&g.b).unwrap() }
    }
}

// Parse a json file on disk
fn file_json(path: PathBuf) -> Result<Vec<Gcd>, Box<dyn std::error::Error>> {

    let file = File::open(path)?;
    let reader = BufReader::new(file);
    // Read the JSON contents of the file as an instance of `User`.
    let json = serde_json::from_reader(reader)?;
    let jsonGcds : Vec<JsonGcd> = json;
    let gcds : Vec<Gcd> = jsonGcds.into_iter()
                                  .map(|g| { Gcd::from(g) })
                                  .collect();
    Ok(gcds)
}

fn main() {
    // Parse command line arguments according to the struct
    let opt = MyOpt::from_args();

    match file_json(opt.file) {
        Err(e) => println!("\nThere was an error with reading the JSON file:\n{:#?}", e),
        Ok(gcds) => {
            println!("First pair:\na = {},\nb = {}", gcds[0].a, gcds[0].b);
            println!("Their gcd: {}", ints::big_ea_simple(&gcds[0].a, &gcds[0].b)); // warm-up

            let before = Instant::now();
            let mut sum : BigInt = Zero::zero(); // Make sure compiler doesn't remove gcd calls
            for i in 1..10 {
                for g in &gcds {
                    let gcd = ints::big_ea_simple(&g.a, &g.b);
                    sum += gcd;
                }
            }
            println!("Simple elapsed: {:.2?}. Gcd sum {}", before.elapsed(), sum);

            ints::big_ea_fast(&gcds[0].a, &gcds[0].b); // warmup
            let before = Instant::now();
            let mut sum : BigInt = Zero::zero(); // Make sure compiler doesn't remove gcd calls
            for i in 1..10 {
                for g in &gcds {
                    let gcd = ints::big_ea_simple(&g.a, &g.b);
                    sum += gcd;
                }
            }
            println!("Simple elapsed: {:.2?}. Gcd sum {}", before.elapsed(), sum);

            // println!("gcd = {}", &gcd);
            ()
        }
    }
}
