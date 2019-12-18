// FIXME: Move these to config.
pub const NUM_GALAXIES: i32 = 9;
pub const NUM_SYSTEMS: i32 = 500;
pub const NUM_POSITIONS: i32 = 15;

pub fn validate_galaxy(galaxy: i32) -> bool {
    galaxy >= 1 && galaxy <= NUM_GALAXIES
}

pub fn validate_system(system: i32) -> bool {
    system >= 1 && system <= NUM_SYSTEMS
}

pub fn validate_position(position: i32) -> bool {
    position >= 1 && position <= NUM_POSITIONS
}

pub fn validate(galaxy: i32, system: i32, position: i32) -> bool {
    validate_galaxy(galaxy) && validate_system(system) && validate_position(position)
}
