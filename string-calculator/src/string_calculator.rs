#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum CalculatorError {
    NegativeNumber(i32),
}

pub trait Logger {
    fn log(&self, number: i32);
}

#[allow(dead_code)]
pub struct StringCalculator<'a> {
    logger: &'a dyn Logger,
}

#[allow(dead_code)]
impl<'a> StringCalculator<'a> {
    pub fn new(logger: &'a (dyn Logger + 'a)) -> StringCalculator {
        Self{
            logger
        }
    }
}

#[allow(dead_code)]
fn add(input: &str) -> Result<i32, CalculatorError> {
    let mut sum = 0;
    let mut delim = ',';

    if input.starts_with("//"){
        if let Some(new_delim) = input.chars().nth(2) {
            delim = new_delim;
        }
    }

    for item in input.split(|c: char| c == delim || c == '\n') {
        if let Ok(number) = item.parse::<i32>(){
            if number < 0 {
                return Err(CalculatorError::NegativeNumber(number));
            }
            sum += number;
        }
    }
    Ok(sum)
}

#[cfg(test)]
mod tests {
    use crate::string_calculator::{add};
    use crate::string_calculator::CalculatorError;
    #[test]
    fn test_empty_string_returns_zero() {
        assert_eq!(0, add("").unwrap());
    }
    #[test]
    fn test_one_number_input() {
        assert_eq!(44, add("44").unwrap());
    }
    #[test]
    fn test_two_number_input() {
        assert_eq!(5, add("2,3").unwrap());
    }
    #[test]
    fn test_unknown_number_input() {
        assert_eq!(55, add("1,2,3,4,5,6,7,8,9,10").unwrap());
    }
    #[test]
    fn test_handle_newlines_input() {
        assert_eq!(6, add("1\n2,3").unwrap());
    }
    #[test]
    fn test_various_delimiters_input() {
        assert_eq!(3, add("//;\n1;2").unwrap());
    }
    #[test]
    fn test_negative_number_input() {
        let result = add("1,-2,3");
        assert_eq!(result, Err(CalculatorError::NegativeNumber(-2)),
        "Negative number should result in error");
    }
}