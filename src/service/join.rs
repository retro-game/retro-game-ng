use crate::model::user;
use crate::util::password::hash;
use crate::validation::{email, password, user_name};
use crate::AppData;
use actix_web::web;
use bitflags::bitflags;
use diesel::pg::PgConnection;
use std::convert::From;

bitflags! {
    pub struct ErrorFlags : u32 {
        const EMAIL_TOO_LONG = 1 << 0;
        const EMAIL_WRONG_FORMAT = 1 << 1;
        const EMAIL_EXISTS = 1 << 2;

        const NAME_TOO_SHORT = 1 << 3;
        const NAME_TOO_LONG = 1 << 4;
        const NAME_WRONG_FORMAT = 1 << 5;
        const NAME_EXISTS = 1 << 6;

        const PASSWORD_TOO_SHORT = 1 << 7;
        const PASSWORD_TOO_LONG = 1 << 8;

        const PASSWORDS_DO_NOT_MATCH = 1 << 9;
    }
}

impl From<email::Error> for ErrorFlags {
    fn from(err: email::Error) -> Self {
        use email::Error;
        match err {
            Error::TooLong => Self::EMAIL_TOO_LONG,
            Error::WrongFormat => Self::EMAIL_WRONG_FORMAT,
        }
    }
}

impl From<user_name::Error> for ErrorFlags {
    fn from(err: user_name::Error) -> Self {
        use user_name::Error;
        match err {
            Error::TooShort => Self::NAME_TOO_SHORT,
            Error::TooLong => Self::NAME_TOO_LONG,
            Error::WrongFormat => Self::NAME_WRONG_FORMAT,
        }
    }
}

impl From<password::Error> for ErrorFlags {
    fn from(err: password::Error) -> Self {
        use password::Error;
        match err {
            Error::TooShort => Self::PASSWORD_TOO_SHORT,
            Error::TooLong => Self::PASSWORD_TOO_LONG,
        }
    }
}

fn validate(
    conn: &PgConnection,
    email: &str,
    name: &str,
    password: &str,
    password_confirm: &str,
) -> ErrorFlags {
    let mut flags = ErrorFlags::empty();

    if let Err(err) = email::validate(email) {
        flags |= ErrorFlags::from(err);
    } else if user::exists_by_email_ignore_case(conn, email) {
        flags |= ErrorFlags::EMAIL_EXISTS;
    }

    if let Err(err) = user_name::validate(name) {
        flags |= ErrorFlags::from(err);
    } else if user::exists_by_name_ignore_case(conn, name) {
        flags |= ErrorFlags::NAME_EXISTS;
    }

    if let Err(err) = password::validate(password) {
        flags |= ErrorFlags::from(err);
    }

    if password != password_confirm {
        flags |= ErrorFlags::PASSWORDS_DO_NOT_MATCH;
    }

    flags
}

pub fn join(
    app_data: web::Data<AppData>,
    email: &str,
    name: &str,
    password: &str,
    password_confirm: &str,
) -> ErrorFlags {
    let conn = &app_data.db_pool.get().unwrap();

    let flags = validate(conn, email, name, password, password_confirm);
    if !flags.is_empty() {
        return flags;
    }

    let new_user = user::NewUser {
        name,
        email,
        password: &hash(password),
    };
    user::create(conn, &new_user);

    ErrorFlags::empty()
}
