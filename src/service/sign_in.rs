use crate::model::user;
use crate::util::password::verify;
use crate::AppData;
use actix_web::web;

pub fn sign_in(app_data: web::Data<AppData>, email: &str, password: &str) -> bool {
    let conn = &app_data.db_pool.get().unwrap();
    match user::find_password_by_email_ignore_case(conn, email) {
        Some(hash) => verify(password, &hash),
        None => false,
    }
}
