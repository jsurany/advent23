mod day_one;
pub(crate) mod prelude;

use anyhow::Result;
use std::{fs::File, io::BufReader};

use day_one::part_one::run as day_one_part_one;
use day_one::part_two::run as day_one_part_two;

use clap::{Parser, ValueEnum};

#[derive(Parser)]
struct Args {
    /// Day of advent to run, from 1 to 25
    #[arg(value_parser = clap::value_parser!(u8).range(1..=25))]
    day: u8,
    /// Which part of the day to run
    #[arg(value_enum, short, long, default_value_t = Part::Two)]
    part: Part,
    /// Whether to run the sample data instead of input data
    #[arg(long, default_value_t = false)]
    sample: bool,
}

#[derive(Clone, ValueEnum)]
enum Part {
    /// Run part One of the day
    One,
    /// Run part two of the day
    Two,
}

fn load(path: String, sample: bool) -> Result<BufReader<File>> {
    let fname = if sample { "sample.txt" } else { "input.txt" };
    let fpath = path + fname;
    let file = File::open(fpath)?;
    Ok(BufReader::new(file))
}

fn main() -> Result<()> {
    let args = Args::parse();

    match args.day {
        1 => {
            let path = String::from("./src/day_one/");
            let buf = load(path, args.sample)?;
            match args.part {
                Part::One => day_one_part_one(buf),
                Part::Two => day_one_part_two(buf),
            }
        }
        _ => unimplemented!(),
    }
}
