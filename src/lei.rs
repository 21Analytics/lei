/// A 20-character Legal Entity Identifier
/// The checksum validation happens according to ISO7064, similarly to
/// IBAN numbers.
#[derive(Debug, PartialEq)]
pub struct LEI {
    lei: String,
}

#[derive(Debug, PartialEq)]
pub struct ParseLEIError;

impl std::fmt::Display for ParseLEIError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Malformed LEI")
    }
}

impl std::fmt::Display for LEI {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.lei)
    }
}

impl std::str::FromStr for LEI {
    type Err = ParseLEIError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() == 20 && &s[4..6] == "00" && validate_checksum(s) {
            Ok(Self { lei: s.into() })
        } else {
            Err(ParseLEIError)
        }
    }
}

impl LEI {
    /// The issuing Local Operating Unit
    #[must_use]
    pub fn lou(&self) -> String {
        self.lei[..4].into()
    }

    #[must_use]
    pub fn entity(&self) -> String {
        self.lei[6..18].into()
    }
}

fn validate_checksum(address: &str) -> bool {
    mod_97(address) == Ok(1)
}

fn mod_97(address: &str) -> Result<u32, ParseLEIError> {
    address.as_bytes().iter().try_fold(0, |acc, c| {
        // Convert '0'-'Z' to 0-35
        if let Some(digit) = (*c as char).to_digit(36) {
            let multiplier = if digit > 9 { 100 } else { 10 };
            Ok((acc * multiplier + digit) % 97)
        } else {
            Err(ParseLEIError)
        }
    })
}

#[cfg(test)]
mod test {
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
        assert_eq!(mod_97("-1"), Err(ParseLEIError));
        assert_eq!(mod_97("123#"), Err(ParseLEIError));
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
    }

    #[test]
    fn test_malformed() {
        assert_eq!(LEI::from_str(""), Err(ParseLEIError));
        // garbage
        assert_eq!(LEI::from_str("#@%$^#%$#%#@%$^#%$#%"), Err(ParseLEIError));
        // too short
        assert_eq!(LEI::from_str("2594007XIACKNUAW223"), Err(ParseLEIError));
        // too long
        assert_eq!(LEI::from_str("2594007XIACKNUAW22334"), Err(ParseLEIError));
        // wrong checksum
        assert_eq!(LEI::from_str("2594007XIACKNMUAW224"), Err(ParseLEIError));
        // non-zero reserved characters
        assert_eq!(LEI::from_str("2594017XIACKNMUAW223"), Err(ParseLEIError));
    }
}
