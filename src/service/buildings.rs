use crate::db::body;
use crate::model::{meets_requirements, Body, BuildingKind, Resources};
use crate::AppData;
use actix_web::web;

pub struct Building {
    pub kind: BuildingKind,
    pub current_level: i32,
    pub future_level: i32,
    pub cost: Resources<i64>,
    pub required_energy: i64,
}

pub struct BuildingsAndQueuePair {
    pub buildings: Vec<Building>,
    pub queue: (), // TODO
}

pub fn get_buildings_and_queue(body: &Body) -> BuildingsAndQueuePair {
    // FIXME: Check planet's fields and queue size.

    let buildings = body
        .buildings
        .iter()
        .filter(|&(kind, &level)| level > 0 || meets_requirements(&body, kind))
        .fold(Vec::new(), |mut acc, (kind, &level)| {
            acc.push(Building {
                kind,
                current_level: level,
                future_level: level,
                cost: Resources::default(),
                required_energy: 0i64,
            });
            acc
        });

    BuildingsAndQueuePair {
        buildings,
        queue: (),
    }
}

pub fn build(app_data: web::Data<AppData>, mut body: Body, kind: BuildingKind) {
    let conn = &app_data.db_pool.get().unwrap();
    body.buildings[kind] += 1;
    body::update_buildings_by_id(conn, &body.id, &body.buildings);
}
