use num_derive::{FromPrimitive, ToPrimitive};
use std::fmt;

#[derive(Clone, Copy, PartialEq, FromPrimitive, ToPrimitive)]
pub enum CoordinatesKind {
    Planet,
    Moon,
    DebrisField,
}

impl fmt::Display for CoordinatesKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Planet => "P",
            Self::Moon => "M",
            Self::DebrisField => "DF",
        };
        f.write_str(s)
    }
}

pub struct Coordinates {
    pub galaxy: i32,
    pub system: i32,
    pub position: i32,
    pub kind: CoordinatesKind,
}

impl fmt::Display for Coordinates {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}-{}-{}-{}",
            self.galaxy, self.system, self.position, self.kind
        )
    }
}
