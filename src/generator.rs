const DIGITS: u32 = 9;

pub fn generate_nth_number(n: u32) -> Result<u32, String> {
    let max_n = factorial(DIGITS as usize);

    if n == 0 {
        return Err("sequence item number must be greater than zero".to_string());
    } else if n > max_n {
        return Err(format!("sequence item number must be less than or equal to {}", max_n).to_string());
    }

    let mut available_digits = (1..DIGITS+1).collect::<Vec<_>>();
    let mut digits_selected: Vec<u32> = Vec::new();

    while !available_digits.is_empty() {
        let selected_index = select_digit_index(n, available_digits.len());

        digits_selected.push(available_digits.remove(selected_index))
    }

    Ok(from_digits(digits_selected))
}

fn select_digit_index(n: u32, number_of_available_digits: usize) -> usize {
    let choices_within_branch = factorial(number_of_available_digits);
    let index_within_branch = (n-1) % choices_within_branch;
    let sub_branch_size = factorial(number_of_available_digits - 1);

    (index_within_branch / sub_branch_size) as usize
}

fn factorial(n: usize) -> u32 {
    (1..n+1).fold(1, |acc, i| acc*i as u32)
}

fn from_digits(digits: Vec<u32>) -> u32 {
    let digit_count = digits.len();

    digits.iter().enumerate()
          .map(|(index, digit)| digit * 10u32.pow((digit_count - index - 1) as u32))
          .fold(0, |acc, value| acc + value)
}

#[cfg(test)]
mod test {
    use super::{generate_nth_number, factorial};

    #[test]
    fn generates_first_number() {
        assert_eq!(generate_nth_number(1), Ok(123456789))
    }

    #[test]
    fn generates_second_number() {
        assert_eq!(generate_nth_number(2), Ok(123456798))
    }

    #[test]
    fn generates_third_number() {
        assert_eq!(generate_nth_number(3), Ok(123456879))
    }

    #[test]
    fn generates_boundary_number() {
        assert_eq!(generate_nth_number(factorial(8) + 1), Ok(213456789))
    }

    #[test]
    fn generates_boundary_number_plus_one() {
        assert_eq!(generate_nth_number(factorial(8) + 2), Ok(213456798))
    }

    #[test]
    fn generates_last_number() {
        assert_eq!(generate_nth_number(factorial(9)), Ok(987654321))
    }

    #[test]
    fn returns_error_for_zero() {
        assert_eq!(generate_nth_number(0), Err("sequence item number must be greater than zero".to_string()))
    }

    #[test]
    fn returns_error_for_greater_than_last_item_in_sequence() {
        assert_eq!(generate_nth_number(factorial(9) + 1), Err("sequence item number must be less than or equal to 362880".to_string()))
    }
}
