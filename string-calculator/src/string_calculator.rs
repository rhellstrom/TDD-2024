pub struct StringCalculator;
impl StringCalculator {
    pub fn add(&self, input: &str) -> i32 {
        let mut sum = 0;
        for item in input.split(|c: char| !c.is_numeric()) {
            if let Ok(number) = item.parse::<i32>(){
                sum += number;
            }
        }
        sum
    }
}

#[cfg(test)]
mod tests {
    use crate::string_calculator::StringCalculator;
    #[test]
    fn test_empty_string_returns_zero() {
        let calculator = StringCalculator;
        assert_eq!(0, calculator.add(""));
    }
    #[test]
    fn test_one_number_input() {
        let calculator = StringCalculator;
        assert_eq!(44, calculator.add("44"));
    }
    #[test]
    fn test_two_number_input() {
        let calculator = StringCalculator;
        assert_eq!(5, calculator.add("2, 3"));
    }
}