use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::str::FromStr;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParsePhoneNumberError {
    #[error("phone number must contain 10 digits")]
    InvalidLength,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct PhoneNumber {
    number: String,
}

impl FromStr for PhoneNumber {
    type Err = ParsePhoneNumberError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parsed: String = s.chars().filter(|c| c.is_digit(10)).collect();
        if parsed.chars().count() == 10 {
            return Ok(PhoneNumber { number: parsed });
        } else {
            return Err(ParsePhoneNumberError::InvalidLength);
        }
    }
}

impl Display for PhoneNumber {
    fn fmt(&self, formatter: &mut Formatter) -> FmtResult {
        Display::fmt(&self.number, formatter)
    }
}

impl<'de> Deserialize<'de> for PhoneNumber {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        <String as Deserialize>::deserialize(deserializer)?
            .parse()
            .map_err(serde::de::Error::custom)
    }
}

impl Serialize for PhoneNumber {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_str(self)
    }
}

#[cfg(test)]
mod tests {
    use super::PhoneNumber;

    #[test]
    fn test_valid() {
        fn parse(input: &str, output: &str) {
            assert_eq!(
                input.parse::<PhoneNumber>().unwrap(),
                output.parse::<PhoneNumber>().unwrap()
            )
        }
        parse("012-345-6789", "0123456789");
        parse("222.222.2222", "2222222222");
        parse("(222) 222-2222", "2222222222");
        parse("2222222222", "2222222222");
    }

    #[test]
    fn test_invalid() {
        fn parse(input: &str) {
            assert!(input.parse::<PhoneNumber>().is_err());
        }
        parse("123456789");
    }
}
