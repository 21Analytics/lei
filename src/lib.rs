pub mod registration_authority;

use anyhow::{Context, Result};
use rand::Rng;

/// A 20-character Legal Entity Identifier
/// The checksum validation happens according to ISO7064, similarly to
/// IBAN numbers.
/// <https://www.gleif.org/en/about-lei/iso-17442-the-lei-code-structure>
#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    serde::Serialize,
    diesel::deserialize::FromSqlRow,
    diesel::expression::AsExpression,
)]
#[diesel(sql_type = diesel::sql_types::Text)]
#[serde(transparent)]
pub struct LEI {
    lei: String,
}

impl<'de> serde::Deserialize<'de> for LEI {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let string: String = serde::Deserialize::deserialize(d)?;
        string
            .as_str()
            .try_into()
            .context("LEI is not valid")
            .map_err(serde::de::Error::custom)
    }
}

async_graphql::scalar!(LEI);

impl std::fmt::Display for LEI {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.lei.fmt(f)
    }
}

impl TryFrom<&str> for LEI {
    type Error = anyhow::Error;
    fn try_from(from: &str) -> Result<Self, Self::Error> {
        if from.len() != 20 {
            anyhow::bail!("invalid length: {}, must be 20", from.len())
        }
        if !validate_checksum(from) {
            anyhow::bail!("invalid checksum")
        }
        Ok(Self { lei: from.into() })
    }
}

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
    pub fn random() -> Result<Self> {
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
        let checksum = 98 - mod_97(&format!("{prefix}00{infix}00"))?;
        Self::try_from(format!("{prefix}00{infix}{checksum:02}").as_str())
    }
}

fn validate_checksum(address: &str) -> bool {
    mod_97(address).map_or_else(|_| false, |m| m == 1)
}

fn mod_97(address: &str) -> Result<u32> {
    address
        .as_bytes()
        .iter()
        .enumerate()
        .try_fold(0, |acc, (i, c)| {
            // Convert '0'-'Z' to 0-35
            let digit = (*c as char)
                .to_digit(36)
                .context(format!("invalid character at position {i}: {c}"))?;
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
        assert!(mod_97("-1")
            .unwrap_err()
            .to_string()
            .starts_with("invalid character"));
        assert!(mod_97("123#")
            .unwrap_err()
            .to_string()
            .starts_with("invalid character"));
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
            assert!(LEI::try_from(lei)
                .unwrap_err()
                .to_string()
                .starts_with("invalid length"));
        }
        assert_eq!(
            LEI::try_from("2594007XIACKNMUAW224")
                .unwrap_err()
                .to_string(),
            "invalid checksum"
        );
    }

    #[test]
    fn test_random() {
        LEI::random().unwrap();
    }
}
