
pub struct Soroban;

// Define a trait for Soroban behavior
pub trait SorobanBehavior {
    fn format_token_amount(amount: &str, decimals: usize) -> String;
    fn parse_token_amount(value: &str, decimals: usize) -> String;
}

impl SorobanBehavior for Soroban {
   
    fn format_token_amount(amount: &str, decimals: usize) -> String {
        let mut formatted = amount.to_string();

        if amount.contains('.') {
            panic!("No decimal is allowed");
        }

        if decimals > 0 {
            if decimals > formatted.len() {
                formatted = format!("0.{}", formatted.chars().collect::<String>().chars().rev().collect::<String>().chars().take(decimals).collect::<String>().chars().rev().collect::<String>());
            } else {
                formatted = format!(
                    "{}.{}",
                    &formatted[..(formatted.len() - decimals)],
                    &formatted[(formatted.len() - decimals)..]
                );
            }
        }

        // Remove trailing zeroes, if any
        formatted.trim_end_matches('0').to_string()
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
            fraction.chars().chain(std::iter::repeat('0')).take(decimals).collect::<String>()
        );

        shifted
    }
}
