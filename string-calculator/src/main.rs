use std::io;

mod string_calculator;

fn main() -> io::Result<()>{
    greeter_text();
    Ok(())
}

fn greeter_text() {
    println!("Welcome to string calculator!\n");
    println!("USAGE: scalc '1,2,3' to return the total of the delimiter separated values\n");
    println!("In order to specify the delimiter you start the number input with //[delimiter]");
    println!("USAGE: scalc '//[***][%%%]1***2%%%4' to set your own delimiter.");
}