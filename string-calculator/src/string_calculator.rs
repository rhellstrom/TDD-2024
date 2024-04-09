use std::error::Error;
use std::io;
use mockall::automock;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug, PartialEq)]
pub enum CalculatorError {
    NegativeNumber(i32),
}

#[automock]
pub trait Logger {
    fn log(&self, number: i32);
}

pub struct StringCalculator<L: Logger> {
    logger: L,
}

impl<L: Logger> StringCalculator<L> {
    pub fn new(logger: L) -> Self {
        Self {
            logger
        }
    }

    pub fn run(&self) -> MyResult<()>{
        greeter_text();
        let mut buffer = String::new();
        loop {
            buffer.clear();
            io::stdin().read_line(&mut buffer)?;
            let trimmed_input = buffer.trim();
            
            if trimmed_input.is_empty(){
                return Ok(())
            }
            
            let input = &trimmed_input[6..].trim().trim_matches('\'');
            if let Ok(result) = self.add(input){
                println!("The result is {}", result);
            }
        }
    }
    
    // Filth
    fn add(&self, input: &str) -> Result<i32, CalculatorError> {
        let mut sum = 0;
        let mut delimiters: Vec<String> = vec![",".to_string(), "\n".to_string()];
        let mut input_string = input;
        
        // Check if we have any custom delimiters to take care of
        if input_string.starts_with("//") {
            if let Some(new_delim) = input_string.chars().nth(2) {
                if new_delim == '[' {
                    let re = regex::Regex::new(r"\[(.*?)]").unwrap();
                    for cap in re.captures_iter(input) {
                        if let Some(substring) = cap.get(1) {
                            delimiters.push(substring.as_str().to_string());
                        }
                    }
                } else {
                    delimiters.push(new_delim.to_string())
                }
            }
            let index = input_string.chars().position(|c| c.is_ascii_digit()).unwrap();
            input_string = &input_string[index..];
        }
        
        // Chop the input string to pieces with our delimiters
        let mut split_input = vec![input_string];
        for delimiter in delimiters {
            let mut new_split_input: Vec<&str> = vec![];
            for part in &split_input {
                new_split_input.extend(part.split(&delimiter));
            }
            split_input = new_split_input;
        }
        
        // Parse numbers 
        for item in split_input {
            if let Ok(number) = item.parse::<i32>() {
                if number < 0 {
                    return Err(CalculatorError::NegativeNumber(number));
                }
        
                if number > 1000 {
                    self.logger.log(number);
                }
                sum += number;
            }
        }
        Ok(sum)
    }
}

fn greeter_text() {
    println!("Welcome to string calculator!\n");
    println!("USAGE: scalc '1,2,3' to return the total of the delimiter separated values\n");
    println!("In order to specify the delimiter you start the number input with //[delimiter]");
    println!("USAGE: scalc '//[***][%%%]1***2%%%4' to set your own delimiter.");
}

#[cfg(test)]
mod tests {
    use super::*;
    fn setup_calculator() -> StringCalculator<MockLogger> {
        let mock_logger = MockLogger::new();
        StringCalculator::new(mock_logger)
    }

    #[test]
    fn test_empty_string_returns_zero() {
        let calculator = setup_calculator();
        assert_eq!(Ok(0), calculator.add(""));
    }
    
    #[test]
    fn test_one_number_input() {
        let calculator = setup_calculator();
        assert_eq!(Ok(44), calculator.add("44"));
    }
    
    #[test]
    fn test_two_number_input() {
        let calculator = setup_calculator();
        assert_eq!(Ok(5), calculator.add("2,3"));
    }
    
    #[test]
    fn test_unknown_number_input() {
        let calculator = setup_calculator();
        assert_eq!(Ok(55), calculator.add("1,2,3,4,5,6,7,8,9,10"));
    }
    
    #[test]
    fn test_handle_newlines_input() {
        let calculator = setup_calculator();
        assert_eq!(Ok(6), calculator.add("1\n2,3"));
    }
    
    #[test]
    fn test_various_delimiters_input() {
        let calculator = setup_calculator();
        assert_eq!(Ok(3), calculator.add("//;\n1;2"));
    }
    
    #[test]
    fn test_negative_number_input() {
        let calculator = setup_calculator();
        let result = calculator.add("1,-2,3");
        assert_eq!(result, Err(CalculatorError::NegativeNumber(-2)),
        "Negative number should result in error");
    }
    
    #[test]
    fn test_string_calculator_does_not_log_numbers_below1001() {
        let mut mock_logger = MockLogger::new();
        mock_logger.expect_log()
            .times(0)
            .return_const(());
        let calculator = StringCalculator::new(mock_logger);
        let _result = calculator.add("1000,200,5");
    }
    
    #[test]
    fn test_string_calculator_does_log_number_greater_than1000() {
        let mut mock_logger = MockLogger::new();
        mock_logger.expect_log()
            .times(1)
            .return_const(());
        let calculator = StringCalculator::new(mock_logger);
        let _result = calculator.add("1234,200,5");
    }
    
    #[test]
    fn test_string_calculator_does_log_numbers_greater_than1000() {
        let mut mock_logger = MockLogger::new();
        mock_logger.expect_log()
            .times(2)
            .return_const(());
        let calculator = StringCalculator::new(mock_logger);
        let _result = calculator.add("1234,5678,69");
    }
}