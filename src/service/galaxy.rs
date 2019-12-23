use crate::db::body;
use crate::model::BodyType;
use crate::AppData;
use actix_web::web;
use num_traits::FromPrimitive;

#[derive(Clone)]
pub struct Slot {
    pub name: String,
    pub type_: BodyType,
    pub image: i32,
}

pub fn get_system(app_data: web::Data<AppData>, galaxy: i32, system: i32) -> Vec<Option<Slot>> {
    let conn = &app_data.db_pool.get().unwrap();

    let mut slots = Vec::new();
    slots.resize(15, None);

    let bodies = body::find_all_by_coordinates_for_galaxy(conn, galaxy, system).unwrap();
    for body in bodies.into_iter() {
        let pos = body.position;
        debug_assert!(pos >= 1 && pos <= 15);
        let index = (pos - 1) as usize;

        let type_ = FromPrimitive::from_i32(body.type_).unwrap();

        debug_assert!(slots[index].is_none());
        slots[index] = Some(Slot {
            name: body.name,
            type_,
            image: body.image,
        });
    }

    slots
}
