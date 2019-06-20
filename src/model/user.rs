use crate::schema::users;
use diesel::dsl::{exists, select};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::{sql_function, sql_types};

sql_function!(fn lower(x: sql_types::Text) -> sql_types::Text);

/*
#[derive(Queryable)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub email: String,
    pub password: String,
}
*/

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub name: &'a str,
    pub email: &'a str,
    pub password: &'a str,
}

pub fn create<'a>(conn: &PgConnection, new_user: &'a NewUser) {
    let rows_inserted = diesel::insert_into(users::table)
        .values(new_user)
        .execute(conn)
        .unwrap();
    assert_eq!(rows_inserted, 1);
}

pub fn exists_by_email_ignore_case(conn: &PgConnection, email: &str) -> bool {
    select(exists(
        users::table.filter(lower(users::email).eq(lower(email))),
    ))
    .get_result(conn)
    .unwrap()
}

pub fn exists_by_name_ignore_case(conn: &PgConnection, name: &str) -> bool {
    select(exists(
        users::table.filter(lower(users::name).eq(lower(name))),
    ))
    .get_result(conn)
    .unwrap()
}

pub fn find_password_by_email_ignore_case(conn: &PgConnection, email: &str) -> Option<String> {
    users::table
        .filter(lower(users::email).eq(lower(email)))
        .select(users::password)
        .first(conn)
        .ok()
}
