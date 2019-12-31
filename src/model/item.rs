use enum_map::Enum;
use std::fmt;

#[derive(Clone, Copy, Debug, Enum)]
pub enum BuildingKind {
    MetalMine,
    CrystalMine,
    DeuteriumSynthesizer,
    SolarPlant,
    FusionReactor,
    RoboticsFactory,
    NaniteFactory,
    Shipyard,
    MetalStorage,
    CrystalStorage,
    DeuteriumTank,
    ResearchLab,
    Terraformer,
    AllianceDepot,
    LunarBase,
    SensorPhalanx,
    JumpGate,
    MissileSilo,
}

impl fmt::Display for BuildingKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO: This is only temporary solution. We should replace the files in skins, e.g.
        // METAL_MINE.gif -> MetalMine.gif.
        let variant_str = format!("{:?}", self);
        let mut result = String::with_capacity(variant_str.len() + 1);
        let mut first = true;
        for c in variant_str.chars() {
            let uppercase = c.to_ascii_uppercase();
            if uppercase == c && !first {
                result.push('_');
            }
            result.push(uppercase);
            first = false;
        }
        f.write_str(&result)
    }
}

#[derive(Enum)]
pub enum TechnologyKind {
    EspionageTechnology,
    ComputerTechnology,
    WeaponsTechnology,
    ShieldingTechnology,
    ArmorTechnology,
    EnergyTechnology,
    HyperspaceTechnology,
    CombustionDrive,
    ImpulseDrive,
    HyperspaceDrive,
    LaserTechnology,
    IonTechnology,
    PlasmaTechnology,
    IntergalacticResearchNetwork,
    Astrophysics,
    GravitonTechnology,
}
