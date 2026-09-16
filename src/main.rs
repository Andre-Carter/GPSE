use gpse::physical::equations::earth_moon_force;
use gpse::entities::celestial::{EARTH, MOON};
use gpse::physical::equations::gravitational_force;
//units
//invalid inputs
//changing state/time
//how equations should compose
//how the eventual simulation layer consumes them
fn main() {
    earth_moon_force();

    let g_force = gravitational_force(
    EARTH.mass_kg,
    MOON.mass_kg,
    384_400_000.0,
    );

    println!("{g_force}");

    let test_1 = gravitational_force(1.0, 1.0, 1.0);
    println!("{test_1}");
    let test_2 = gravitational_force(2.0, 1.0, 1.0);
    println!("{test_2}");
}





