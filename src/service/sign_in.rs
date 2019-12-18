use crate::db::{body, user};
use crate::util::password::verify;
use crate::AppData;
use actix_web::web;
use uuid::Uuid;

pub struct SignInData {
    pub user_id: Uuid,
    pub homeworld_id: Option<Uuid>,
}

pub fn sign_in(app_data: web::Data<AppData>, email: &str, password: &str) -> Option<SignInData> {
    let conn = &app_data.db_pool.get().unwrap();

    let (id, hash) = match user::find_id_and_password_by_email_ignore_case(conn, email) {
        Some(pair) => pair,
        None => return None,
    };

    if !verify(password, &hash) {
        return None;
    }

    let homeworld_id = body::find_homeworld_id_by_user_id(conn, &id);
    Some(SignInData {
        user_id: id,
        homeworld_id,
    })
}
