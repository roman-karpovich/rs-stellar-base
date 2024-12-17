pub struct Soroban;

// Define a trait for Soroban behavior
pub trait SorobanBehavior {
    fn format_token_amount(amount: &str, decimals: usize) -> String;
    fn parse_token_amount(value: &str, decimals: usize) -> String;
}

impl SorobanBehavior for Soroban {
    fn format_token_amount(amount: &str, decimals: usize) -> String {
        // Check if input contains a decimal point
        if amount.contains('.') {
            panic!("No decimals are allowed");
        }

        // If no decimals, return the original amount
        if decimals == 0 {
            return amount.to_string();
        }

        // Pad with zeros to ensure correct decimal representation
        let padded = format!("{:0>10}", amount);

        // If decimals are more than padded length, return zero-padded decimal
        if decimals > padded.len() {
            return format!(
                "0.{}",
                padded
                    .chars()
                    .rev()
                    .take(decimals)
                    .collect::<String>()
                    .chars()
                    .rev()
                    .collect::<String>()
            );
        }

        // Split the amount into whole and fractional parts
        let (whole, fraction) = padded.split_at(padded.len() - decimals);

        // Format the amount with a leading zero before the decimal point if necessary
        let formatted = format!(
            "{}.{}",
            whole.trim_start_matches('0'),
            fraction
        );

        // Ensure the result includes a leading zero if the fractional part exists
        let mut result = if formatted.starts_with('.') {
            format!("0{}", formatted)
        } else {
            formatted
        };

        // Remove trailing zeroes
        result = result.trim_end_matches('0').to_string();

        // If the result has only the decimal point left, remove it
        if result.ends_with('.') {
            result.pop();
        }

        result
    }

    fn parse_token_amount(value: &str, decimals: usize) -> String {
        let parts: Vec<&str> = value.split('.').collect();

        if parts.len() > 2 {
            panic!("Invalid decimal value: {}", value);
        }

        let whole = parts[0];
        let fraction = parts.get(1).unwrap_or(&"");

        let shifted = format!(
            "{}{}",
            whole,
            fraction
                .chars()
                .chain(std::iter::repeat('0'))
                .take(decimals)
                .collect::<String>()
        );

        shifted
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::panic;
    
    #[test]
    fn test_format_token_amount_success_cases() {
        let test_cases = [
            ("1000000001", 7, "100.0000001"),
            ("10000000010", 5, "100000.0001"),
            ("10000000010", 0, "10000000010"),
            ("10000", 10, "0.000001"),
            ("1567890", 10, "0.000156789"),
            ("1230", 0, "1230"),
        ];

        for (amount, decimals, expected) in test_cases.iter() {
            assert_eq!(
                Soroban::format_token_amount(amount, *decimals), 
                *expected,
                "Failed for amount: {}, decimals: {}",
                amount,
                decimals
            );
        }
    }

    #[test]
    fn test_format_token_amount_failure_cases() {
        let test_cases = [
            ("1000000001.1", 7),
            ("10000.00001.1", 4),
        ];

        for (amount, decimals) in test_cases.iter() {
            let result = panic::catch_unwind(|| {
                Soroban::format_token_amount(amount, *decimals)
            });

            assert!(
                result.is_err(), 
                "Expected panic for amount: {}, decimals: {}",
                amount,
                decimals
            );
        }
    }

    // Test cases for the `parse_token_amount` function
    #[test]
    fn test_parse_token_amount_success_cases() {
        let test_cases = [
            // Test the input with a whole number and expected decimal shifts
            ("100", 2, "10000"),
            ("123.4560", 5, "12345600"),
            ("100", 5, "10000000"),
        ];

        for (amount, decimals, expected) in test_cases.iter() {
            assert_eq!(
                Soroban::parse_token_amount(amount, *decimals),
                *expected,
                "Failed for amount: {}, decimals: {}",
                amount,
                decimals
            );
        }
    }

    #[test]
    fn test_parse_token_amount_failure_cases() {
        let test_cases = [
            // Invalid case with multiple decimal points
            ("1000000.001.1", 7, "Invalid decimal value")
        ];

        for (amount, decimals, expected) in test_cases.iter() {
            let result = panic::catch_unwind(|| {
                Soroban::parse_token_amount(amount, *decimals);
            });

            assert!(
                result.is_err(),
                "Expected panic for amount: {}, decimals: {}",
                amount,
                decimals
            );

            if let Err(err) = result {
                let err_msg = err.downcast_ref::<String>().unwrap();
                assert!(err_msg.contains(expected), "Error message does not match: {}", err_msg);
            }
        }
    }
}