extern crate num_bigint;
extern crate num_rational;
extern crate num_traits;

use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::{One, Zero};
use std::str::FromStr;

const MAX_INT: u32 = (1 << 31) - 1;

fn best_r(raw_number: &str) -> Result<String, &'static str> {
    let mut number = if raw_number.contains('.') {
        let parts: Vec<&str> = raw_number.split('.').collect();
        let integer_part = parts[0];
        let decimal_part = parts[1];
        let len = decimal_part.len();

        let numerator = BigInt::from_str(&(integer_part.to_string() + decimal_part)).unwrap();
        let denominator = BigInt::from(10).pow((len as u64).try_into().unwrap());

        BigRational::new(numerator, denominator)
    } else {
        BigRational::from_str(raw_number).unwrap()
    };

    let mut fractions = vec![
        (BigInt::zero(), BigInt::one()),
        (BigInt::one(), BigInt::zero()),
    ];

    loop {
        if number.numer().clone() > MAX_INT.into() {
            break;
        }

        let a = number.to_integer();
        let f = &number - &a;
        let h = &a * &fractions[fractions.len() - 1].0 + &fractions[fractions.len() - 2].0;
        let k = &a * &fractions[fractions.len() - 1].1 + &fractions[fractions.len() - 2].1;

        if h > MAX_INT.into() || k > MAX_INT.into() {
            break;
        }

        fractions.push((h, k));

        if f == BigRational::zero() {
            break;
        }

        number = BigRational::one() / f;
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
    use num_traits::ToPrimitive;

    use super::*;

    #[test]
    fn correctly_calculates_best_rational_approximation() {
        let binding = BigRational::new(
            BigInt::from_str("118").unwrap(),
            BigInt::from_str("37").unwrap(),
        )
        .to_string();

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
