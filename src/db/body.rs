use crate::model;
use crate::schema::bodies;
use chrono::NaiveDateTime;
use diesel::dsl::{exists, select};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::Error;
use enum_map::{Enum, EnumMap};
use num_traits::FromPrimitive;
use uuid::Uuid;

#[derive(Insertable)]
#[table_name = "bodies"]
pub struct NewBody<'a> {
    pub id: &'a Uuid,
    pub user_id: &'a Uuid,
    pub name: &'a str,
    pub galaxy: i32,
    pub system: i32,
    pub position: i32,
    pub kind: i32,
    pub metal: f64,
    pub crystal: f64,
    pub deuterium: f64,
    pub resources_updated_at: &'a NaiveDateTime,
    pub created_at: &'a NaiveDateTime,
    pub diameter: i32,
    pub temperature: i32,
    pub type_: i32,
    pub image: i32,
}

pub fn create<'a>(conn: &PgConnection, new_body: &'a NewBody) -> Result<(), Error> {
    let rows_inserted = diesel::insert_into(bodies::table)
        .values(new_body)
        .execute(conn)?;
    debug_assert_eq!(rows_inserted, 1);
    Ok(())
}

pub fn exists_by_user_id(conn: &PgConnection, user_id: &Uuid) -> bool {
    select(exists(bodies::table.filter(bodies::user_id.eq(user_id))))
        .get_result(conn)
        .unwrap()
}

#[derive(Queryable)]
pub struct GalaxyBody {
    pub id: Uuid,
    pub user_id: Option<Uuid>,
    pub name: String,
    pub position: i32,
    pub type_: i32,
    pub image: i32,
}

pub fn find_all_by_coordinates_for_galaxy(
    conn: &PgConnection,
    galaxy: i32,
    system: i32,
) -> Option<Vec<GalaxyBody>> {
    bodies::table
        .filter(bodies::galaxy.eq(galaxy).and(bodies::system.eq(system)))
        .select((
            bodies::id,
            bodies::user_id,
            bodies::name,
            bodies::position,
            bodies::type_,
            bodies::image,
        ))
        .load(conn)
        .ok()
}

#[derive(Queryable)]
struct Body {
    id: Uuid,
    user_id: Option<Uuid>,
    name: String,
    galaxy: i32,
    system: i32,
    position: i32,
    kind: i32,
    metal: f64,
    crystal: f64,
    deuterium: f64,
    resources_updated_at: NaiveDateTime,
    created_at: NaiveDateTime,
    diameter: i32,
    temperature: i32,
    type_: i32,
    image: i32,
    buildings: Vec<i32>,
    units: Vec<i32>,
    building_queue: Vec<i32>,
    shipyard_queue: Vec<i32>,
}

impl From<Body> for model::Body {
    fn from(body: Body) -> model::Body {
        let kind = FromPrimitive::from_i32(body.kind).unwrap();
        let coordinates = model::Coordinates {
            galaxy: body.galaxy,
            system: body.system,
            position: body.position,
            kind,
        };

        let type_ = FromPrimitive::from_i32(body.type_).unwrap();

        debug_assert!((kind == model::CoordinatesKind::Planet) == (type_ != model::BodyType::Moon));

        let buildings =
            body.buildings
                .into_iter()
                .enumerate()
                .fold(EnumMap::new(), |mut acc, (i, level)| {
                    acc[Enum::<model::BuildingKind>::from_usize(i)] = level;
                    acc
                });

        model::Body {
            id: body.id,
            user_id: body.user_id,
            name: body.name,
            coordinates,
            metal: body.metal,
            crystal: body.crystal,
            deuterium: body.deuterium,
            diameter: body.diameter,
            temperature: body.temperature,
            type_,
            image: body.image,
            buildings,
        }
    }
}

pub fn find_all_by_user_id(conn: &PgConnection, user_id: &Uuid) -> Vec<model::Body> {
    bodies::table
        .filter(bodies::user_id.eq(user_id))
        .load::<Body>(conn)
        .unwrap()
        .into_iter()
        .map(model::Body::from)
        .collect()
}

pub fn find_homeworld_id_by_user_id(conn: &PgConnection, user_id: &Uuid) -> Option<Uuid> {
    bodies::table
        .filter(bodies::user_id.eq(user_id))
        .select(bodies::id)
        .order(bodies::created_at.asc())
        .first(conn)
        .ok()
}

pub fn update_buildings_by_id(
    conn: &PgConnection,
    id: &Uuid,
    buildings: &EnumMap<model::BuildingKind, i32>,
) {
    buildings.as_slice();
    let res = diesel::update(bodies::table.filter(bodies::id.eq(id)))
        .set(bodies::buildings.eq(buildings.as_slice()))
        .returning(bodies::id)
        .get_result::<Uuid>(conn)
        .unwrap();
    assert_eq!(res, *id);
}
