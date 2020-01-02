use crate::model::{BuildingKind, Coordinates, CoordinatesKind, Resources};
use enum_map::EnumMap;
use num_derive::{FromPrimitive, ToPrimitive};
use rand::{random, thread_rng};
use rand_distr::{Distribution, Normal};
use std::fmt;
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

#[derive(Clone, Copy, Debug, PartialEq, FromPrimitive, ToPrimitive)]
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

impl fmt::Display for BodyType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = format!("{:?}", self);
        f.write_str(&s.to_ascii_uppercase())
    }
}

#[derive(Clone)]
pub struct Body {
    pub id: Uuid,
    pub user_id: Option<Uuid>,
    pub name: String,
    pub coordinates: Coordinates,
    pub metal: f64,
    pub crystal: f64,
    pub deuterium: f64,
    pub diameter: i32,
    pub temperature: i32,
    pub type_: BodyType,
    pub image: i32,
    pub buildings: EnumMap<BuildingKind, i32>,
}

impl Body {
    fn planet_max_fields(&self) -> i32 {
        debug_assert!(self.diameter > 0);
        let x = self.diameter as f64 / 1000.0;
        let terraformer_level = self.buildings[BuildingKind::Terraformer];
        (x * x) as i32 + terraformer_level * 6
    }

    fn moon_max_fields(&self) -> i32 {
        let lunar_base_level = self.buildings[BuildingKind::LunarBase];
        1 + lunar_base_level * 3
    }

    pub fn max_fields(&self) -> i32 {
        match self.coordinates.kind {
            CoordinatesKind::Planet => self.planet_max_fields(),
            CoordinatesKind::Moon => self.moon_max_fields(),
            CoordinatesKind::DebrisField => unreachable!(),
        }
    }

    pub fn used_fields(&self) -> i32 {
        self.buildings.values().sum()
    }
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

// Mines and synthesizer.
pub struct ResourceSourceProduction {
    pub prod: i64,
    pub cur_usage: i64, // Current energy usage.
    pub max_usage: i64, // Max energy usage.
}

pub struct SolarPlantProduction {
    pub prod: i64, // Energy production.
}

pub struct FusionReactorProduction {
    pub prod: i64,  // Energy production.
    pub usage: i64, // Deuterium usage.
}

pub struct SolarSatellitesProduction {
    pub prod: i64, // Energy production.
}

pub struct EnergyBalance {
    pub used: i64,
    pub available: i64,
}

impl EnergyBalance {
    pub fn total(&self) -> i64 {
        self.used + self.available
    }
}

pub struct Production {
    pub base_prod: Resources<i64>,
    pub metal_mine: ResourceSourceProduction,
    pub crystal_mine: ResourceSourceProduction,
    pub deuterium_synthesizer: ResourceSourceProduction,
    pub solar_plant: SolarPlantProduction,
    pub fusion_reactor: FusionReactorProduction,
    pub solar_satellites: SolarSatellitesProduction,
    pub energy_balance: EnergyBalance,
    pub efficiency: f64,
    pub total_prod: Resources<i64>,
}

// TODO: Move to config.

const PRODUCTION_SPEED: i64 = 1;

const METAL_BASE_PRODUCTION: i64 = 20;
const CRYSTAL_BASE_PRODUCTION: i64 = 10;
const DEUTERIUM_BASE_PRODUCTION: i64 = 0;

const METAL_MINE_BASE_PRODUCTION: i64 = 30;
const CRYSTAL_MINE_BASE_PRODUCTION: i64 = 20;
const DEUTERIUM_SYNTHESIZER_BASE_PRODUCTION: i64 = 10;

const METAL_MINE_BASE_ENERGY_USAGE: i64 = 10;
const CRYSTAL_MINE_BASE_ENERGY_USAGE: i64 = 10;
const DEUTERIUM_SYNTHESIZER_BASE_ENERGY_USAGE: i64 = 20;

const SOLAR_PLANT_BASE_ENERGY_PRODUCTION: i64 = 20;
const FUSION_REACTOR_BASE_ENERGY_PRODUCTION: i64 = 30;
const FUSION_REACTOR_BASE_DEUTERIUM_CONSUMPTION: i64 = 10;

impl Body {
    pub fn get_production(&self) -> Production {
        // TODO: Move this to body table.
        // 1 -> 10%, ..., 10 -> 100%
        let metal_mine_f = 10;
        let crystal_mine_f = 10;
        let deuterium_synthesizer_f = 10;
        let solar_plant_f = 10;
        let fusion_reactor_f = 10;
        let solar_satellites_f = 10;

        let buildings = &self.buildings;

        // Resources sources.

        let base_prod = Resources {
            metal: METAL_BASE_PRODUCTION * PRODUCTION_SPEED,
            crystal: CRYSTAL_BASE_PRODUCTION * PRODUCTION_SPEED,
            deuterium: DEUTERIUM_BASE_PRODUCTION * PRODUCTION_SPEED,
        };

        let calc_max_prod_and_usage = |factor, kind, base_prod, base_usage, temp_factor| {
            let f = 0.1 * factor as f64;
            let level = buildings[kind];
            let p = 1.1_f64.powi(level);
            let prod =
                ((base_prod * level as i64) as f64 * p * temp_factor * f) as i64 * PRODUCTION_SPEED;
            let max_usage = ((base_usage * level as i64) as f64 * p * f).ceil() as i64;
            (prod, max_usage)
        };

        let (metal_mine_max_prod, metal_mine_max_usage) = calc_max_prod_and_usage(
            metal_mine_f,
            BuildingKind::MetalMine,
            METAL_MINE_BASE_PRODUCTION,
            METAL_MINE_BASE_ENERGY_USAGE,
            1.0,
        );

        let (crystal_mine_max_prod, crystal_mine_max_usage) = calc_max_prod_and_usage(
            crystal_mine_f,
            BuildingKind::CrystalMine,
            CRYSTAL_MINE_BASE_PRODUCTION,
            CRYSTAL_MINE_BASE_ENERGY_USAGE,
            1.0,
        );

        let temp_factor = 1.28 - 0.002 * self.temperature as f64;
        let (deuterium_synthesizer_max_prod, deuterium_synthesizer_max_usage) =
            calc_max_prod_and_usage(
                deuterium_synthesizer_f,
                BuildingKind::DeuteriumSynthesizer,
                DEUTERIUM_SYNTHESIZER_BASE_PRODUCTION,
                DEUTERIUM_SYNTHESIZER_BASE_ENERGY_USAGE,
                temp_factor,
            );

        // Energy sources.

        let solar_plant_prod = {
            let f = 0.1 * solar_plant_f as f64;
            let level = buildings[BuildingKind::SolarPlant];
            let p = 1.1_f64.powi(level);
            ((SOLAR_PLANT_BASE_ENERGY_PRODUCTION * level as i64) as f64 * p * f) as i64
        };

        let (fusion_reactor_prod, fusion_reactor_usage) = {
            let f = 0.1 * fusion_reactor_f as f64;
            let level = buildings[BuildingKind::FusionReactor];
            let energy_tech_level = 0; // TODO: Get this from user's techs.
            let energy_f = (1.05 + 0.01 * energy_tech_level as f64).powi(level);
            let prod = (((FUSION_REACTOR_BASE_ENERGY_PRODUCTION * level as i64) as f64 * energy_f)
                .floor()
                * f)
                .round() as i64;
            let p = 1.1_f64.powi(level);
            let usage = ((FUSION_REACTOR_BASE_DEUTERIUM_CONSUMPTION * level as i64) as f64 * p)
                .ceil() as i64
                * PRODUCTION_SPEED;
            (prod, usage)
        };

        let solar_satellites_prod = {
            let n = 0; // TODO: Get number of sats from body.
            let f = 0.1 * solar_satellites_f as f64;
            let one_energy = ((self.temperature as f64 / 4.0 + 20.0) as i64).clamp(5, 50);
            ((n * one_energy) as f64 * f).round() as i64
        };

        // Energy balance & efficiency.

        let total_energy = solar_plant_prod + fusion_reactor_prod + solar_satellites_prod;
        let used_energy =
            metal_mine_max_usage + crystal_mine_max_usage + deuterium_synthesizer_max_usage;
        let available_energy = total_energy - used_energy;

        let efficiency = if used_energy == 0 {
            0.0
        } else {
            (total_energy as f64 / used_energy as f64).min(1.0)
        };

        // Current energy usage.

        let calc_cur_usage = |max_usage| (max_usage as f64 * efficiency) as i64;

        let metal_mine_cur_usage = calc_cur_usage(metal_mine_max_usage);
        let crystal_mine_cur_usage = calc_cur_usage(crystal_mine_max_usage);
        let deuterium_synthesizer_cur_usage = calc_cur_usage(deuterium_synthesizer_max_usage);

        // Current production given the efficiency.

        let calc_real_prod = |max_prod| (max_prod as f64 * efficiency) as i64;

        let metal_mine_prod = calc_real_prod(metal_mine_max_prod);
        let crystal_mine_prod = calc_real_prod(crystal_mine_max_prod);
        let deuterium_synthesizer_prod = calc_real_prod(deuterium_synthesizer_max_prod);

        // Total production.

        let total_prod = Resources {
            metal: base_prod.metal + metal_mine_prod,
            crystal: base_prod.crystal + crystal_mine_prod,
            deuterium: base_prod.deuterium + deuterium_synthesizer_prod - fusion_reactor_usage,
        };

        Production {
            base_prod,
            metal_mine: ResourceSourceProduction {
                prod: metal_mine_prod,
                cur_usage: metal_mine_cur_usage,
                max_usage: metal_mine_max_usage,
            },
            crystal_mine: ResourceSourceProduction {
                prod: crystal_mine_prod,
                cur_usage: crystal_mine_cur_usage,
                max_usage: crystal_mine_max_usage,
            },
            deuterium_synthesizer: ResourceSourceProduction {
                prod: deuterium_synthesizer_prod,
                cur_usage: deuterium_synthesizer_cur_usage,
                max_usage: deuterium_synthesizer_max_usage,
            },
            solar_plant: SolarPlantProduction {
                prod: solar_plant_prod,
            },
            fusion_reactor: FusionReactorProduction {
                prod: fusion_reactor_prod,
                usage: fusion_reactor_usage,
            },
            solar_satellites: SolarSatellitesProduction {
                prod: solar_satellites_prod,
            },
            energy_balance: EnergyBalance {
                used: used_energy,
                available: available_energy,
            },
            efficiency,
            total_prod,
        }
    }
}
