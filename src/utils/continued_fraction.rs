use num_traits::{One, Zero};
use std::str::FromStr;

const MAX_INT: u32 = (1 << 31) - 1;

fn best_r(raw_number: &str) -> Result<String, &'static str> {
    let mut number = raw_number.parse::<f64>().unwrap();

    let mut fractions = vec![(0f64, 1f64), (1f64, 0f64)];

    loop {
        let a = (number as i64) as f64;
        let f = number - a;
        let h = a * fractions[fractions.len() - 1].0 + fractions[fractions.len() - 2].0;
        let k = a * fractions[fractions.len() - 1].1 + fractions[fractions.len() - 2].1;

        if h > MAX_INT.into() || k > MAX_INT.into() {
            break;
        }

        fractions.push((h, k));

        if f == 0f64 {
            break;
        }

        number = 1f64 / f;
    }

    let (n, d) = fractions.last().unwrap();

    if n.is_zero() || d.is_zero() {
        return Err("Couldn't find approximation");
    }

    Ok(format!("{},{}", n, d))
}

fn main() {
    match best_r("3.141592653589793238") {
        Ok(res) => println!("{}", res),
        Err(e) => println!("Error: {}", e),
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn correctly_calculates_best_rational_approximation() {
        let binding = (118f64 / 37f64).to_string();
        dbg!(&binding);

        let tests = vec![
            ("1,10", "0.1"),
            ("1,100", "0.01"),
            ("1,1000", "0.001"),
            ("54301793,100000", "543.017930"),
            ("31969983,100000", "319.69983"),
            ("93,100", "0.93"),
            ("1,2", "0.5"),
            ("173,100", "1.730"),
            ("5333399,6250000", "0.85334384"),
            ("11,2", "5.5"),
            ("272783,100000", "2.72783"),
            ("638082,1", "638082.0"),
            ("36731261,12500000", "2.93850088"),
            ("1451,25", "58.04"),
            ("8253,200", "41.265"),
            ("12869,2500", "5.1476"),
            ("4757,50", "95.14"),
            ("3729,5000", "0.74580"),
            ("4119,1", "4119.0"),
            ("118,37", &binding),
        ];

        for (expected, input) in tests {
            let result = best_r(input).unwrap();
            assert_eq!(result, expected);
        }
    }

    #[test]
    #[should_panic(expected = "Couldn't find approximation")]
    fn throws_error_when_approximation_cannot_be_found() {
        best_r("0.0000000003").unwrap();
        best_r("2147483648").unwrap();
    }
}
