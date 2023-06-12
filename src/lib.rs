//! # Rust LEI Library
//!
//! This crate provides functionality to work with Legal Entity
//! Identifiers (LEIs):
//!
//! ```
//! use leim as lei;
//! assert!(lei::LEI::try_from("2594007XIACKNMUAW223").is_ok());
//! assert_eq!(
//!     lei::LEI::try_from("2594007XIACKNMUAW222"),
//!     Err(lei::Error::InvalidChecksum)
//! );
//! ```

#![cfg_attr(docsrs, feature(doc_cfg))]

/// Functionality related to registration authorities.
pub mod registration_authority;

use rand::Rng;

/// The errors emitted when parsing a LEI.
#[derive(Debug, PartialEq, thiserror::Error)]
pub enum Error {
    /// The LEI had an invalid length.
    #[error("invalid length: {0}, expected 20")]
    InvalidLength(usize),
    /// The LEI had an invalid checksum.
    #[error("invalid checksum")]
    InvalidChecksum,
    /// The LEI contained an invalid character.
    #[error("invalid character at position {pos}: {char}")]
    InvalidChar { pos: usize, char: char },
}

type Result<T> = std::result::Result<T, Error>;

/// A 20-character Legal Entity Identifier. The checksum validation
/// happens according to ISO7064, similarly to  IBAN numbers.
/// <https://www.gleif.org/en/about-lei/iso-17442-the-lei-code-structure>
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize)]
#[cfg_attr(
    feature = "diesel",
    derive(diesel::deserialize::FromSqlRow, diesel::expression::AsExpression)
)]
#[cfg_attr(feature = "diesel", diesel(sql_type = diesel::sql_types::Text))]
#[serde(transparent)]
pub struct LEI {
    lei: String,
}

impl<'de> serde::Deserialize<'de> for LEI {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> std::result::Result<Self, D::Error> {
        let string: String = serde::Deserialize::deserialize(d)?;
        string.as_str().try_into().map_err(serde::de::Error::custom)
    }
}

#[cfg(feature = "async-graphql")]
#[cfg_attr(docsrs, doc(cfg(feature = "async-graphql")))]
async_graphql::scalar!(LEI);

impl std::fmt::Display for LEI {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.lei.fmt(f)
    }
}

impl TryFrom<&str> for LEI {
    type Error = Error;
    fn try_from(from: &str) -> Result<Self> {
        if from.len() != 20 {
            return Err(Error::InvalidLength(from.len()));
        }
        if !validate_checksum(from) {
            return Err(Error::InvalidChecksum);
        }
        Ok(Self { lei: from.into() })
    }
}

#[cfg(feature = "diesel")]
#[cfg_attr(docsrs, doc(cfg(feature = "diesel")))]
impl<DB> diesel::deserialize::FromSql<diesel::sql_types::Text, DB> for LEI
where
    DB: diesel::backend::Backend,
    String: diesel::deserialize::FromSql<diesel::sql_types::Text, DB>,
{
    fn from_sql(bytes: diesel::backend::RawValue<DB>) -> diesel::deserialize::Result<Self> {
        Ok(std::convert::TryFrom::try_from(
            String::from_sql(bytes)?.as_str(),
        )?)
    }
}

#[cfg(feature = "diesel")]
#[cfg_attr(docsrs, doc(cfg(feature = "diesel")))]
impl<DB> diesel::serialize::ToSql<diesel::sql_types::Text, DB> for LEI
where
    DB: diesel::backend::Backend,
    str: diesel::serialize::ToSql<diesel::sql_types::Text, DB>,
{
    fn to_sql<'b>(
        &'b self,
        out: &mut diesel::serialize::Output<'b, '_, DB>,
    ) -> diesel::serialize::Result {
        self.lei.as_str().to_sql(out)
    }
}

impl LEI {
    /// Constructs a random LEI with a valid checksum (only for
    /// testing purposes).
    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        let prefix: String = (0..4)
            .map(|_| rng.sample(rand::distributions::Alphanumeric))
            .map(char::from)
            .collect::<String>()
            .to_uppercase();
        let infix: String = (0..12)
            .map(|_| rng.sample(rand::distributions::Alphanumeric))
            .map(char::from)
            .collect::<String>()
            .to_uppercase();
        // Use placeholder 0s to compute needed checksum
        let checksum = 98 - mod_97(&format!("{prefix}00{infix}00")).unwrap();
        Self::try_from(format!("{prefix}00{infix}{checksum:02}").as_str()).unwrap()
    }
}

fn validate_checksum(lei: &str) -> bool {
    mod_97(lei).map_or_else(|_| false, |m| m == 1)
}

fn mod_97(lei: &str) -> Result<u32> {
    lei.as_bytes()
        .iter()
        .enumerate()
        .try_fold(0, |acc, (i, c)| {
            // Convert '0'-'Z' to 0-35
            let digit = (*c as char).to_digit(36).ok_or(Error::InvalidChar {
                pos: i,
                char: *c as char,
            })?;
            let multiplier = if digit > 9 { 100 } else { 10 };
            Ok((acc * multiplier + digit) % 97)
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mod_97() {
        assert_eq!(mod_97("").unwrap(), 0);
        assert_eq!(mod_97("1").unwrap(), 1);
        assert_eq!(mod_97("02").unwrap(), 2);
        assert_eq!(mod_97("96").unwrap(), 96);
        assert_eq!(mod_97("97").unwrap(), 0);
        assert_eq!(mod_97("98").unwrap(), 1);
        assert_eq!(mod_97("9799").unwrap(), 2);
        assert_eq!(
            mod_97("-1").unwrap_err(),
            Error::InvalidChar { pos: 0, char: '-' }
        );
        assert_eq!(
            mod_97("123#").unwrap_err(),
            Error::InvalidChar { pos: 3, char: '#' }
        );
    }

    #[test]
    fn test_happy_parse() {
        // from https://lei.info/portal/resources/lei-code/
        LEI::try_from("2594007XIACKNMUAW223").unwrap();
        // from https://en.wikipedia.org/wiki/Legal_Entity_Identifier
        LEI::try_from("54930084UKLVMY22DS16").unwrap();
        LEI::try_from("213800WSGIIZCXF1P572").unwrap();
        LEI::try_from("5493000IBP32UQZ0KL24").unwrap();
        // Standard Chartered Bank
        LEI::try_from("RILFO74KP1CM8P6PCT96").unwrap();
    }

    #[test]
    fn test_malformed() {
        for lei in ["", "2594007XIACKNUAW223", "2594007XIACKNUAW22334"] {
            assert_eq!(
                LEI::try_from(lei).unwrap_err(),
                Error::InvalidLength(lei.len())
            );
        }
        assert_eq!(
            LEI::try_from("2594007XIACKNMUAW224").unwrap_err(),
            Error::InvalidChecksum
        );
    }

    #[test]
    fn test_random() {
        LEI::random();
    }
}
