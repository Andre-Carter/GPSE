pub struct CelestialBody {
    pub name: &'static str,
    pub mass_kg: f64,
    pub radius_m: f64,
}

pub static SUN: CelestialBody = CelestialBody {
    name: "Sun",
    mass_kg: 1.989e30, //kg
    radius_m: 696_340.0, //km
};

pub static EARTH: CelestialBody = CelestialBody {
    name: "Earth",
    mass_kg: 5.9722e24,
    radius_m: 6_371.0,
};

pub static MOON: CelestialBody = CelestialBody {
    name: "Moon",
    mass_kg: 7.34e22,
    radius_m: 1_737.0,
};