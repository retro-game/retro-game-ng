use rand::{random, thread_rng};
use rand_distr::{Distribution, Normal};
use uuid::Uuid;

pub const HOMEWORLD_METAL: f64 = 1000.0;
pub const HOMEWORLD_CRYSTAL: f64 = 500.0;
pub const HOMEWORLD_DEUTERIUM: f64 = 0.0;

#[allow(dead_code)]
pub const COLONY_METAL: f64 = 1000.0;
#[allow(dead_code)]
pub const COLONY_CRYSTAL: f64 = 500.0;
#[allow(dead_code)]
pub const COLONY_DEUTERIUM: f64 = 0.0;

pub enum BodyType {
    Moon,
    Dry,
    Desert,
    Jungle,
    Normal,
    Water,
    Ice,
    Gas,
}

pub struct Body {
    pub id: Uuid,
    pub user_id: Option<Uuid>,
    pub name: String,
    pub galaxy: i32,
    pub system: i32,
    pub position: i32,
    pub kind: i32,
    pub metal: f64,
    pub crystal: f64,
    pub deuterium: f64,
}

#[allow(dead_code)]
pub fn generate_planet_diameter(position: i32) -> i32 {
    debug_assert!(position >= 1 && position <= 15);
    let x = (8 - position).abs() as f64;
    let mean = 200.0 - 10.0 * x;
    let std_dev = 60.0 - 5.0 * x;
    let normal = Normal::new(mean, std_dev).unwrap();
    let sample = normal.sample(&mut thread_rng());
    let num_fields = sample.max(42.0);
    (num_fields.sqrt() * 100.0) as i32 * 10
}

#[allow(dead_code)]
pub fn generate_moon_diameter(chance: f64) -> i32 {
    debug_assert!(chance >= 0.01 && chance <= 0.2);
    let r = random::<u32>() % (20 - 10 + 1) + 10;
    let num_fields = r + 3 * (100.0 * chance) as u32;
    ((num_fields as f64).sqrt() * 1000.0) as i32
}

pub fn generate_body_temperature(position: i32) -> i32 {
    debug_assert!(position >= 1 && position <= 15);
    let x = (8 - position).abs() as f64;
    let mean = 30.0 + 1.75 * x.signum() * x * x;
    let std_dev = 10.0;
    let normal = Normal::new(mean, std_dev).unwrap();
    let sample = normal.sample(&mut thread_rng()) as i32;
    sample.max(-60).min(120)
}

pub fn generate_planet_type(position: i32) -> BodyType {
    debug_assert!(position >= 1 && position <= 15);
    match position {
        1..=3 => {
            if random() {
                BodyType::Dry
            } else {
                BodyType::Desert
            }
        }
        4..=6 => BodyType::Jungle,
        7..=8 => BodyType::Normal,
        9 => {
            if random() {
                BodyType::Normal
            } else {
                BodyType::Water
            }
        }
        10..=12 => BodyType::Water,
        13 => BodyType::Ice,
        14..=15 => {
            if random() {
                BodyType::Ice
            } else {
                BodyType::Gas
            }
        }
        _ => unreachable!(),
    }
}
