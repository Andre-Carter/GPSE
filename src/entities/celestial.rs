pub struct CelestialBody {
    pub name: &'static str,
    pub mass_kg: f64,
    pub radius_m: f64,
}

pub static SUN: CelestialBody = CelestialBody {
    name: "Sun",
    mass: 1.989 * 10^30, //kg
    radius_m: 696,340, //km
};

pub static EARTH: CelestialBody = CelestialBody {
    name: "Earth",
    mass: 5.9722 * 10^24,
    radius_m: 6371.0,
};

pub static MOON: CelestialBody = CelestialBody {
    name: "Moon",
    mass: 7.34 * 10^22,
    radius_m: 1737,
};