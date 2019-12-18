use crate::db::body;
use crate::model;
use crate::util::date_time;
use crate::validation::coordinates;
use crate::AppData;
use actix_web::web;
use diesel::result::{DatabaseErrorKind, Error};
use diesel::{result, Connection};
use log::{error, warn};
use uuid::Uuid;

pub enum CreateHomeworldError {
    Internal,
    HomeworldExists,
    PositionTaken,
}

pub fn create_homeworld(
    app_data: web::Data<AppData>,
    user_id: Uuid,
    galaxy: i32,
    system: i32,
    position: i32,
) -> Result<Uuid, CreateHomeworldError> {
    // This should have been already validated in the controller.
    debug_assert!(coordinates::validate(galaxy, system, position));
    debug_assert!(position >= 4 && position <= 12);

    let body_id = Uuid::new_v4();
    let now = date_time::now();
    let temperature = model::generate_body_temperature(position);
    let type_ = model::generate_planet_type(position) as i32;
    let image = (rand::random::<u32>() % 11) as i32;
    let new_body = body::NewBody {
        id: &body_id,
        user_id: &user_id,
        name: "Homeworld",
        galaxy,
        system,
        position,
        kind: 0,
        metal: model::HOMEWORLD_METAL,
        crystal: model::HOMEWORLD_CRYSTAL,
        deuterium: model::HOMEWORLD_DEUTERIUM,
        resources_updated_at: &now,
        created_at: &now,
        diameter: 12800,
        temperature,
        type_,
        image,
    };

    let conn = &app_data.db_pool.get().unwrap();
    let result = conn.transaction::<(), _, _>(|| {
        if body::exists_by_user_id(conn, &user_id) {
            return Err(result::Error::RollbackTransaction);
        }

        body::create(conn, &new_body)
    });

    match result {
        Ok(()) => Ok(body_id),
        Err(err) => match err {
            Error::RollbackTransaction => {
                warn!(
                    "Creating a homeworld failed, user already has a homeworld: user_id={}",
                    user_id,
                );
                Err(CreateHomeworldError::HomeworldExists)
            }
            Error::DatabaseError(db_err_kind, detail) => {
                if let DatabaseErrorKind::UniqueViolation = db_err_kind {
                    warn!(
                        "Creating a homeworld failed, a position is taken: user_id={} coordinates={}:{}:{}",
                        user_id, galaxy, system, position,
                    );
                    Err(CreateHomeworldError::PositionTaken)
                } else {
                    warn!(
                        "Creating a homeworld failed, internal db error: {}",
                        detail.message(),
                    );
                    Err(CreateHomeworldError::Internal)
                }
            }
            _ => {
                error!("Creating a homeworld failed, internal error: {:?}", err,);
                Err(CreateHomeworldError::Internal)
            }
        },
    }
}
