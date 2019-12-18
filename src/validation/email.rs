use lazy_static::lazy_static;
use regex::Regex;

pub const MAX_LENGTH: usize = 128;
// Based on:
// https://html.spec.whatwg.org/multipage/input.html#valid-e-mail-address
pub const PATTERN: &str = r"^[a-zA-Z0-9.!#$%&'*+/=?^_`{|}~-]+@[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(?:\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*$";

lazy_static! {
    static ref REGEX: Regex = Regex::new(PATTERN).unwrap();
}

#[derive(Debug, PartialEq)]
pub enum Error {
    TooLong,
    WrongFormat,
}

pub fn validate(email: &str) -> Result<(), Error> {
    if email.len() > MAX_LENGTH {
        return Err(Error::TooLong);
    }

    if !REGEX.is_match(email) {
        return Err(Error::WrongFormat);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_length() {
        assert!(validate(&format!("{}@test", "a".repeat(MAX_LENGTH - 5))).is_ok());
        assert_eq!(
            validate(&format!("{}@test", "a".repeat(MAX_LENGTH - 4))),
            Err(Error::TooLong)
        );
    }

    // Stolen from:
    // https://github.com/django/django/blob/2.2.2/tests/validators/tests.py#L43
    #[test]
    fn test_format() {
        // Django tests that work.
        assert!(validate("email@here.com").is_ok());
        assert!(validate("weirder-email@here.and.there.com").is_ok());
        assert!(validate("example@valid-----hyphens.com").is_ok());
        assert!(validate("example@valid-with-hyphens.com").is_ok());
        assert!(validate("email@localhost").is_ok());
        assert!(validate("email@localdomain").is_ok());
        assert!(validate(&format!("example@atm.{}", "a".repeat(63))).is_ok());
        assert!(validate(&format!("example@{}.atm", "a".repeat(63))).is_ok());
        assert!(validate(&format!(
            "example@{}.{}.atm",
            "a".repeat(63),
            "b".repeat(10)
        ))
        .is_ok());
        assert!(validate(&format!("example@atm.{}", "a".repeat(64))).is_err());
        assert!(validate(&format!(
            "example@{}.atm.{}",
            "b".repeat(64),
            "a".repeat(63)
        ))
        .is_err());
        assert!(validate("").is_err());
        assert!(validate("abc").is_err());
        assert!(validate("abc@").is_err());
        assert!(validate("a @x.cz").is_err());
        assert!(validate("abc@.com").is_err());
        assert!(validate("something@@somewhere.com").is_err());
        assert!(validate("email@[127.0.0.256]").is_err());
        assert!(validate("email@[2001:db8::12345]").is_err());
        assert!(validate("email@[2001:db8:0:0:0:0:1]").is_err());
        assert!(validate("email@[::ffff:127.0.0.256]").is_err());
        assert!(validate("example@invalid-.com").is_err());
        assert!(validate("example@-invalid.com").is_err());
        assert!(validate("example@invalid.com-").is_err());
        assert!(validate("example@inv-.alid-.com").is_err());
        assert!(validate("example@inv-.-alid.com").is_err());
        assert!(validate("test@example.com\n\n<script src=\"x.js\">").is_err());
        assert!(validate("\"\\\012\"@here.com").is_err());
        assert!(validate("trailingdot@shouldfail.com.").is_err());
        assert!(validate(&format!("a@{}.us", "a".repeat(63))).is_ok());
        assert!(validate(&format!("a@{}.us", "a".repeat(64))).is_err());
        assert!(validate("a@b.com\n").is_err());
        assert!(validate("a\n@b.com").is_err());
        assert!(validate("\"test@test\"\n@example.com").is_err());
        assert!(validate("a@[127.0.0.1]\n").is_err());

        // Django tests that don't work. This is fine, as HTML5 email is a willful violation of RFC
        // 5322.
        // FIXME: Should HTML5 email be used at all?
        assert!(validate("email@[127.0.0.1]").is_err());
        assert!(validate("email@[2001:dB8::1]").is_err());
        assert!(validate("email@[2001:dB8:0:0:0:0:0:1]").is_err());
        assert!(validate("email@[::fffF:127.0.0.1]").is_err());
        assert!(validate("test@domain.with.idn.tld.उदाहरण.परीक्षा").is_err());
        assert!(validate(r#""test@test"@example.com"#).is_err());
        assert!(validate("abc@bar").is_ok());
        assert!(validate("email@127.0.0.1").is_ok());
        assert!(validate("\"\\\011\"@here.com").is_err());
    }
}
