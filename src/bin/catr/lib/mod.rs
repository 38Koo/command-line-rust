use std::error::Error;

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn run() -> MyResult<()> {
    println!("Hello, world!");
    Ok(())
}

pub mod catr; // catr.rs を読み込み