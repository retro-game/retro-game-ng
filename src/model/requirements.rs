use crate::model::{Body, BuildingKind, TechnologyKind};

pub struct Requirements {
    pub buildings: &'static [(BuildingKind, i32)],
    pub technologies: &'static [(TechnologyKind, i32)],
}

pub static BUILDING_REQUIREMENTS: &[Requirements] = &[
    /* MetalMine */ Requirements {
        buildings: &[],
        technologies: &[],
    },
    /* CrystalMine */ Requirements {
        buildings: &[],
        technologies: &[],
    },
    /* DeuteriumSynthesizer */
    Requirements {
        buildings: &[],
        technologies: &[],
    },
    /* SolarPlant */ Requirements {
        buildings: &[],
        technologies: &[],
    },
    /* FusionReactor */
    Requirements {
        buildings: &[(BuildingKind::DeuteriumSynthesizer, 5)],
        technologies: &[(TechnologyKind::EnergyTechnology, 3)],
    },
    /* RoboticsFactory */
    Requirements {
        buildings: &[],
        technologies: &[],
    },
    /* NaniteFactory */
    Requirements {
        buildings: &[(BuildingKind::RoboticsFactory, 10)],
        technologies: &[(TechnologyKind::ComputerTechnology, 10)],
    },
    /* Shipyard */
    Requirements {
        buildings: &[(BuildingKind::RoboticsFactory, 2)],
        technologies: &[],
    },
    /* MetalStorage */ Requirements {
        buildings: &[],
        technologies: &[],
    },
    /* CrystalStorage */
    Requirements {
        buildings: &[],
        technologies: &[],
    },
    /* DeuteriumTank */ Requirements {
        buildings: &[],
        technologies: &[],
    },
    /* ResearchLab */ Requirements {
        buildings: &[],
        technologies: &[],
    },
    /* Terraformer */
    Requirements {
        buildings: &[(BuildingKind::NaniteFactory, 1)],
        technologies: &[(TechnologyKind::EnergyTechnology, 12)],
    },
    /* AllianceDepot */ Requirements {
        buildings: &[],
        technologies: &[],
    },
    /* LunarBase */ Requirements {
        buildings: &[],
        technologies: &[],
    },
    /* SensorPhalanx */
    Requirements {
        buildings: &[(BuildingKind::LunarBase, 1)],
        technologies: &[],
    },
    /* JumpGate */
    Requirements {
        buildings: &[(BuildingKind::LunarBase, 1)],
        technologies: &[(TechnologyKind::HyperspaceTechnology, 7)],
    },
    /* MissileSilo */
    Requirements {
        buildings: &[(BuildingKind::Shipyard, 1)],
        technologies: &[],
    },
];

fn get_requirements(kind: BuildingKind) -> &'static Requirements {
    let index = kind as usize;
    debug_assert!(index < BUILDING_REQUIREMENTS.len());
    &BUILDING_REQUIREMENTS[index]
}

pub fn meets_requirements(body: &Body, kind: BuildingKind) -> bool {
    let requirements = get_requirements(kind);
    let buildings_ok = !requirements
        .buildings
        .iter()
        .any(|&(kind, level)| body.buildings[kind] < level);
    let technologies_ok = true; // TODO
    buildings_ok && technologies_ok
}
