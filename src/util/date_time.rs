use chrono::{NaiveDateTime, Utc};

pub fn now() -> NaiveDateTime {
    // FIXME: Remove ms part.
    Utc::now().naive_utc()
}
