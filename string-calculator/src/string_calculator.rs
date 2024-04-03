pub struct StringCalculator;


#[derive(Debug, PartialEq)]
pub enum CalculatorError {
    NegativeNumber(i32),
}
impl StringCalculator {
    pub fn add(&self, input: &str) -> Result<i32, CalculatorError> {
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
}

#[cfg(test)]
mod tests {
    use crate::string_calculator::StringCalculator;
    use crate::string_calculator::CalculatorError;
    #[test]
    fn test_empty_string_returns_zero() {
        let calculator = StringCalculator;
        assert_eq!(0, calculator.add("").unwrap());
    }
    #[test]
    fn test_one_number_input() {
        let calculator = StringCalculator;
        assert_eq!(44, calculator.add("44").unwrap());
    }
    #[test]
    fn test_two_number_input() {
        let calculator = StringCalculator;
        assert_eq!(5, calculator.add("2,3").unwrap());
    }
    #[test]
    fn test_unknown_number_input() {
        let calculator = StringCalculator;
        assert_eq!(55, calculator.add("1,2,3,4,5,6,7,8,9,10").unwrap());
    }
    #[test]
    fn test_handle_newlines_input() {
        let calculator = StringCalculator;
        assert_eq!(6, calculator.add("1\n2,3").unwrap());
    }
    #[test]
    fn test_various_delimiters_input() {
        let calculator = StringCalculator;
        assert_eq!(3, calculator.add("//;\n1;2").unwrap());
    }
    #[test]
    fn test_negative_number_input() {
        let calculator = StringCalculator;
        let result = calculator.add("1,-2,3");

        assert_eq!(result, Err(CalculatorError::NegativeNumber(-2)),
        "Negative number should result in error");
    }
}