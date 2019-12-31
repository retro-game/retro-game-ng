use crate::model::{meets_requirements, Body, BuildingKind, Resources};

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
