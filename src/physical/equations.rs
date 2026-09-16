use crate::physical::constants::NEWTONIAN_CONSTANT_OF_GRAVITATION;
use crate::entities::celestial::EARTH;
use crate::entities::celestial::MOON;

pub fn earth_moon_force() {
    let _g = NEWTONIAN_CONSTANT_OF_GRAVITATION.value;
    const EARTH_MOON_DISTANCE: f64 = 384_400_000.0;
    let m1m2 = EARTH.mass_kg * MOON.mass_kg;
    let _f = _g * m1m2 / EARTH_MOON_DISTANCE.powi(2);

    println!("{_f}");
}

pub fn gravitational_force(mass_1: f64, mass_2: f64, distance: f64) -> f64 {
    let _g = NEWTONIAN_CONSTANT_OF_GRAVITATION.value;
    let g_force = _g * mass_1 * mass_2 / distance.powi(2);

    return g_force;
}