use lazy_static::lazy_static;
use regex::Regex;

pub const MIN_LENGTH: usize = 3;
pub const MAX_LENGTH: usize = 16;
pub const PATTERN: &str = "^[A-Za-z0-9]+( ?[A-Za-z0-9])*$";

lazy_static! {
    static ref REGEX: Regex = Regex::new(PATTERN).unwrap();
}

pub enum Error {
    TooShort,
    TooLong,
    WrongFormat,
}

pub fn validate(name: &str) -> Result<(), Error> {
    let len = name.len();

    if len < MIN_LENGTH {
        return Err(Error::TooShort);
    }

    if len > MAX_LENGTH {
        return Err(Error::TooLong);
    }

    if !REGEX.is_match(name) {
        return Err(Error::WrongFormat);
    }

    Ok(())
}
