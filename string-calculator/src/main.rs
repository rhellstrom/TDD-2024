use std::io;
use crate::string_calculator::{Logger, StringCalculator};

mod string_calculator;

pub struct StubLogger;

impl Logger for StubLogger {
    fn log(&self, number: i32){
        println!("Logged number {}", number);
    }
}

fn main() -> io::Result<()>{
    let logger = StubLogger;
    if let Err(e) = StringCalculator::new(logger).run(){
        eprintln!("{}", e);
        std::process::exit(1);
    }
    Ok(())
}