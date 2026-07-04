const INVALID_CNPJ: [&str; 10] = [
    "00000000000000",
    "11111111111111",
    "22222222222222",
    "33333333333333",
    "44444444444444",
    "55555555555555",
    "66666666666666",
    "77777777777777",
    "88888888888888",
    "99999999999999",
];

fn sum_digits(value: &str, initial_factor: usize, initial_position: usize) -> u32 {
    let mut sum = 0u32;
    for factor in (2..=initial_factor).rev() {
        let index = initial_position + (initial_factor - factor);
        let digit = value.as_bytes()[index].wrapping_sub(b'0') as u32;
        sum += digit * factor as u32;
    }
    sum
}

fn check_digit(sum: u32) -> char {
    let remainder = sum % 11;
    let digit = 11 - remainder;
    let value = if digit > 9 { 0 } else { digit };
    char::from(b'0' + value as u8)
}

fn is_valid_cnpj_digits(digits: &str) -> bool {
    if digits.len() != 14 || INVALID_CNPJ.contains(&digits) {
        return false;
    }
    let base = &digits[..12];
    let provided = &digits[12..];
    let sum1 = sum_digits(base, 5, 0) + sum_digits(base, 9, 4);
    let digit1 = check_digit(sum1);
    let with_first = format!("{base}{digit1}");
    let sum2 = sum_digits(&with_first, 6, 0) + sum_digits(&with_first, 9, 5);
    let digit2 = check_digit(sum2);
    provided == format!("{digit1}{digit2}")
}

use std::fmt;

use crate::error::CommerceError;

/// Brazilian company tax identifier with check-digit validation (BR-CO-001).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Cnpj(String);

impl Cnpj {
    pub fn parse(raw: &str) -> Result<Self, CommerceError> {
        let digits: String = raw.chars().filter(|c| c.is_ascii_digit()).collect();
        if !is_valid_cnpj_digits(&digits) {
            return Err(CommerceError::InvalidCnpj);
        }
        Ok(Self(digits))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Cnpj {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn br_co_001_given_invalid_check_digits_when_parse_then_invalid_cnpj() {
        assert_eq!(
            Cnpj::parse("00000000000000"),
            Err(CommerceError::InvalidCnpj)
        );
    }

    #[test]
    fn given_valid_cnpj_when_parse_then_ok() {
        let cnpj = Cnpj::parse("11222333000181").expect("valid cnpj");
        assert_eq!(cnpj.as_str(), "11222333000181");
    }
}
