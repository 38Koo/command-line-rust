use clap::Parser;
use std::{error::Error, fs::File, io::{self, BufRead, BufReader}};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Parser,Debug)]
#[command(author, version, about)]
pub struct  Args {
  #[arg(default_value = "-", value_name = "FILE")]
  files: Vec<String>,

  #[arg(
    short('n'),
    long,
    default_value = "10",
    value_name = "LINES",
    value_parser = clap::value_parser!(u64).range(1..)
  )]
  lines: u64,

  #[arg(
    short('c'),
    long,
    value_name = "BYTES",
    conflicts_with("lines"),
    value_parser = clap::value_parser!(u64).range(1..)
  )]
  bytes: Option<u64>
}

pub fn run(args: Args) -> MyResult<()> {
  for (_, filename) in args.files.iter().enumerate() {
    match  open(filename) {
        Err(err) => eprintln!("{filename}: {err}"),
        Ok(_) => {
          println!("Opened {}", filename)
        }
    }
  }
  Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>>  {
  match filename {
    "-" => Ok(Box::new(BufReader::new(io::stdin()))),
    _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
  }
}
