use std::error::Error;

use clap::{App, Arg};

#[derive(Debug)]
pub struct Config {
  files: Vec<String>,
  number_lines: bool,
  number_nonblank_lines: bool,
}

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn run(config: Config) -> MyResult<()> {
  for filename in config.files {
    println!("{}", filename);
  }
  Ok(())
}


pub fn get_args() -> MyResult<Config> {
  let matches = App::new("catr")
    .version("0.1.0")
    .about("Rust cat")
    .arg(
      Arg::with_name("files")
        .value_name("FILE")
        .help("Input file(s)")
        .multiple(true)
        .default_value("-"),
    )
    .arg(
      Arg::with_name("number") 
        .short("n")
        .long("number")
        .help("Number lines")
        .takes_value(false)
        .conflicts_with("number-nonblank"),
    )
    .arg(
      Arg::with_name("number-nonblank")
        .short("b")    
        .long("number-nonblank")
        .help("Number nonblank lines")
        .takes_value(false),
    )
    .get_matches();

  Ok(Config {
    files: matches.values_of_lossy("files").unwrap(),
    number_lines: matches.is_present("number"),
    number_nonblank_lines: matches.is_present("number-nonblank"),
  })
}
