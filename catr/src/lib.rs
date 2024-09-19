use anyhow::Result;
use clap::Parser;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Parser, Debug)]
pub struct Args {
  #[arg(value_name = "FILE", default_value = "-")]
  files: Vec<String>,

  #[arg(short('n'), long("number"), conflicts_with("number_nonblank_lines"))]
  number_lines: bool,

  #[arg(short('b'), long("number-nonblank"))]
  number_nonblank_lines: bool
}

pub fn run(args: Args) -> MyResult<()> {
  for filename in args.files {
    match open(&filename) {
      Err(err) => eprintln!("{filename}: {err}"),
      Ok(file) => {
        let mut prev_num = 0;
        for (line_num, line_result) in file.lines().enumerate() {
          let line = line_result?;
          if args.number_lines {
            println!("{:6}\t{line}", line_num + 1);
          } else if args.number_nonblank_lines {
            if line.is_empty() {
              println!();
            } else {
              prev_num += 1;
              println!("{prev_num:6}\t{line}")
            }
          } else {
            println!("{line}")
          }
        }
      }
    }
  }
  Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
  match filename {
    "-" => Ok(Box::new(BufReader::new(io::stdin()))),
    _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
  }
}
