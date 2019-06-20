pub const MIN_LENGTH: usize = 8;
pub const MAX_LENGTH: usize = 128;

#[derive(Debug, PartialEq)]
pub enum Error {
    TooShort,
    TooLong,
}

pub fn validate(password: &str) -> Result<(), Error> {
    let len = password.len();

    if len < MIN_LENGTH {
        return Err(Error::TooShort);
    }

    if len > MAX_LENGTH {
        return Err(Error::TooLong);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_length() {
        assert_eq!(validate(&"a".repeat(MIN_LENGTH - 1)), Err(Error::TooShort));
        assert!(validate(&"a".repeat(MIN_LENGTH)).is_ok());
    }

    #[test]
    fn test_max_length() {
        assert!(validate(&"a".repeat(MAX_LENGTH)).is_ok());
        assert_eq!(validate(&"a".repeat(MAX_LENGTH + 1)), Err(Error::TooLong));
    }
}
