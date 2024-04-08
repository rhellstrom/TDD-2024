use mockall::automock;

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

#[allow(dead_code)]
impl<L: Logger> StringCalculator<L> {
    pub fn new(logger: L) -> Self {
        Self {
            logger
        }
    }

    fn add(&self, input: &str) -> Result<i32, CalculatorError> {
        let mut sum = 0;
        let mut delim = ',';

        if input.starts_with("//") {
            if let Some(new_delim) = input.chars().nth(2) {
                delim = new_delim;
            }
        }

        for item in input.split(|c: char| c == delim || c == '\n') {
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