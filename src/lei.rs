use rand::Rng;
use std::str::FromStr;

/// A 20-character Legal Entity Identifier
/// The checksum validation happens according to ISO7064, similarly to
/// IBAN numbers.
/// <https://www.gleif.org/en/about-lei/iso-17442-the-lei-code-structure>
#[allow(clippy::clippy::upper_case_acronyms)]
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    diesel::FromSqlRow,
    diesel::AsExpression,
)]
#[sql_type = "diesel::sql_types::Text"]
#[serde(transparent)]
pub struct LEI {
    lei: String,
}

#[derive(thiserror::Error, Debug, PartialEq)]
#[error("ParseLeiError: {0}")]
pub struct ParseLEIError(&'static str);

impl std::fmt::Display for LEI {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.lei)
    }
}

impl std::str::FromStr for LEI {
    type Err = ParseLEIError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 20 {
            return Err(ParseLEIError("invalid length"));
        }
        if !validate_checksum(s) {
            return Err(ParseLEIError("invalid checksum"));
        }
        Ok(Self { lei: s.into() })
    }
}

impl std::convert::TryFrom<&str> for LEI {
    type Error = ParseLEIError;
    fn try_from(from: &str) -> Result<Self, Self::Error> {
        Self::from_str(from)
    }
}

util::sql_via_string!(LEI);

#[juniper::graphql_scalar()]
impl<S: ScalarValue> GraphQLScalar for LEI {
    fn resolve(&self) -> juniper::Value {
        juniper::Value::scalar(self.lei.clone())
    }

    fn from_input_value(v: &InputValue) -> Option<Self> {
        v.as_string_value().and_then(|s| LEI::from_str(s).ok())
    }

    fn from_str<'a>(value: ScalarToken<'a>) -> juniper::ParseScalarResult<'a, S> {
        <String as juniper::ParseScalarValue<S>>::from_str(value)
    }
}

impl LEI {
    pub fn random() -> Result<Self, ParseLEIError> {
        let mut rng = rand::thread_rng();
        let prefix: String = (0..4)
            .map(|_| rng.sample(rand::distributions::Alphanumeric))
            .collect::<String>()
            .to_uppercase();
        let infix: String = (0..12)
            .map(|_| rng.sample(rand::distributions::Alphanumeric))
            .collect::<String>()
            .to_uppercase();
        // Use placeholder 0s to compute needed checksum
        let checksum = 98 - mod_97(&format!("{}00{}00", prefix, infix))?;
        Self::from_str(&format!("{}00{}{:02}", prefix, infix, checksum))
    }

    /// The issuing Local Operating Unit
    #[must_use]
    pub fn lou(&self) -> String {
        self.lei.get(..4).unwrap().into()
    }

    #[must_use]
    pub fn entity(&self) -> String {
        self.lei.get(6..18).unwrap().into()
    }
}

fn validate_checksum(address: &str) -> bool {
    mod_97(address) == Ok(1)
}

fn mod_97(address: &str) -> Result<u32, ParseLEIError> {
    address.as_bytes().iter().try_fold(0, |acc, c| {
        // Convert '0'-'Z' to 0-35
        (*c as char)
            .to_digit(36)
            .map_or(Err(ParseLEIError("invalid character")), |digit| {
                let multiplier = if digit > 9 { 100 } else { 10 };
                Ok((acc * multiplier + digit) % 97)
            })
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_mod_97() {
        assert_eq!(mod_97(""), Ok(0));
        assert_eq!(mod_97("1"), Ok(1));
        assert_eq!(mod_97("02"), Ok(2));
        assert_eq!(mod_97("96"), Ok(96));
        assert_eq!(mod_97("97"), Ok(0));
        assert_eq!(mod_97("98"), Ok(1));
        assert_eq!(mod_97("9799"), Ok(2));
        assert_eq!(
            mod_97("-1").unwrap_err().to_string(),
            "ParseLeiError: invalid character"
        );
        assert_eq!(
            mod_97("123#").unwrap_err().to_string(),
            "ParseLeiError: invalid character"
        );
    }

    #[test]
    fn test_happy_parse() {
        // from https://lei.info/portal/resources/lei-code/
        let lei = LEI::from_str(&"2594007XIACKNMUAW223").unwrap();
        assert_eq!(lei.lou(), String::from("2594"));
        assert_eq!(lei.entity(), String::from("7XIACKNMUAW2"));
        // from https://en.wikipedia.org/wiki/Legal_Entity_Identifier
        LEI::from_str(&"54930084UKLVMY22DS16").unwrap();
        LEI::from_str(&"213800WSGIIZCXF1P572").unwrap();
        LEI::from_str(&"5493000IBP32UQZ0KL24").unwrap();
        // Standard Chartered Bank
        LEI::from_str(&"RILFO74KP1CM8P6PCT96").unwrap();
    }

    #[test]
    fn test_malformed() {
        assert_eq!(
            LEI::from_str("").unwrap_err().to_string(),
            "ParseLeiError: invalid length"
        );
        assert_eq!(
            LEI::from_str("2594007XIACKNUAW223")
                .unwrap_err()
                .to_string(),
            "ParseLeiError: invalid length"
        );
        assert_eq!(
            LEI::from_str("2594007XIACKNUAW22334")
                .unwrap_err()
                .to_string(),
            "ParseLeiError: invalid length"
        );
        assert_eq!(
            LEI::from_str("2594007XIACKNMUAW224")
                .unwrap_err()
                .to_string(),
            "ParseLeiError: invalid checksum"
        );
    }

    #[test]
    fn test_random() {
        LEI::random().unwrap();
    }
}
