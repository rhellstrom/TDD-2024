mod string_calculator;

use std::io;
use std::io::Write;
use crate::string_calculator::StringCalculator;

fn main() -> io::Result<()>{
    let mut buffer = String::new();
    let calculator = StringCalculator;

    //Loop until the user inputs "exit"
    loop {
        print!("? ");
        io::stdout().flush()?;

        buffer.clear();
        io::stdin().read_line(&mut buffer)?;
        if buffer.trim().eq_ignore_ascii_case("exit"){
            break;
        }

        let result = calculator.add(&buffer);
        println!("{}", result);
    }
    println!("Exiting");
    Ok(())
}