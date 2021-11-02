// Borrows heavily from https://github.com/portier/portier-broker
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::str::FromStr;
use thiserror::Error;

fn is_invalid_domain_char(c: char) -> bool {
    matches!(
        c,
        '\0' | '\t' | '\n' | '\r' | ' ' | '#' | '%' | '/' | ':' | '?' | '@' | '[' | '\\' | ']'
    )
}

#[derive(Debug, Error)]
pub enum ParseEmailError {
    #[error("missing '@' separator in email address")]
    NoSeparator,
    #[error("local part of an email address cannot be empty")]
    EmptyLocal,
    #[error("invalid international domain name in email address")]
    InvalidIdna(idna::Errors),
    #[error("domain part of email address cannot be empty")]
    EmptyDomain,
    #[error("email address contains invalid characters in the domain part")]
    InvalidDomainChars,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct EmailAddress {
    serialization: String,
}

impl FromStr for EmailAddress {
    type Err = ParseEmailError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let local_end = s.rfind('@').ok_or(ParseEmailError::NoSeparator)?;
        let local_part = s[..local_end].to_lowercase();
        if local_part.is_empty() {
            return Err(ParseEmailError::EmptyLocal);
        }

        let domain =
            idna::domain_to_ascii(&s[local_end + 1..]).map_err(ParseEmailError::InvalidIdna)?;
        if domain.is_empty() {
            return Err(ParseEmailError::EmptyDomain);
        }
        if domain.find(is_invalid_domain_char).is_some() {
            return Err(ParseEmailError::InvalidDomainChars);
        }

        Ok(EmailAddress::from_parts(&local_part, &domain))
    }
}

impl EmailAddress {
    fn from_parts(local_part: &str, domain: &str) -> EmailAddress {
        EmailAddress {
            serialization: format!("{}@{}", local_part, domain),
        }
    }
}

impl Display for EmailAddress {
    fn fmt(&self, formatter: &mut Formatter) -> FmtResult {
        Display::fmt(&self.serialization, formatter)
    }
}

impl<'de> Deserialize<'de> for EmailAddress {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        <String as Deserialize>::deserialize(deserializer)?
            .parse()
            .map_err(serde::de::Error::custom)
    }
}

impl Serialize for EmailAddress {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_str(self)
    }
}

#[cfg(test)]
mod tests {
    use super::EmailAddress;

    #[test]
    fn test_valid() {
        fn parse(input: &str, output: &str) {
            assert_eq!(
                input.parse::<EmailAddress>().unwrap(),
                output.parse::<EmailAddress>().unwrap()
            )
        }
        parse("EXAMPLE@EXAMPLE.COM", "example@example.com");
        parse("\"ex@mple\"@example.com", "\"ex@mple\"@example.com");
    }

    #[test]
    fn test_invalid() {
        fn parse(input: &str) {
            assert!(input.parse::<EmailAddress>().is_err());
        }
        parse("missing_domain@");
        parse("@missing.local_part");
        parse("missing_separator");
    }
}
