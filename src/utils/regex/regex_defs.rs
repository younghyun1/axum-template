use anyhow::Result;
use thiserror::Error;

pub struct RegexTestPair {
    pub name: &'static str,
    pub regex: &'static str,
    pub test: &'static str,
}

#[derive(Error, Debug)]
pub enum RegexTestError {
    #[error("Invalid regex in {name}: {source}")]
    InvalidRegex { source: regex::Error, name: String },

    #[error("Valid regex but test string does not match: {0}")]
    TestFailed(String),
}

impl RegexTestPair {
    pub fn is_valid(&self) -> Result<()> {
        let regex = match regex::Regex::new(self.regex) {
            Ok(exp) => exp,
            Err(e) => return Err(e.into()),
        };
        if !regex.is_match(self.test) {
            return Err(RegexTestError::TestFailed(self.test.to_string()).into());
        }
        Ok(())
    }
}

pub const EMAIL_VALIDATE_REGEX: &'static str = r"[a-z0-9!#$%&'*+/=?^_`{|}~-]+(\.[a-z0-9!#$%&'*+/=?^_`{|}~-]+)*@[a-z0-9]+([a-z0-9-]*[a-z0-9])?(\.[a-z0-9]+([a-z0-9-]*[a-z0-9])?)+";
pub const USERNAME_VALIDATE_REGEX: &'static str = r"^[A-Za-z\d@$!%*?&-_]{8,}$";

pub const REGEX_LIST: [RegexTestPair; 2] = [
    RegexTestPair {
        name: "EMAIL_VALIDATE_REGEX",
        regex: EMAIL_VALIDATE_REGEX,
        test: "complicated.email+alias_123@example-domain.com",
    },
    RegexTestPair {
        name: "USERNAME_VALIDATE_REGEX",
        regex: USERNAME_VALIDATE_REGEX,
        test: "complex-User_Name.123",
    },
];
