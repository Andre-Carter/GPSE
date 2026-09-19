use gpse::cli::commands::cli_commands;
use gpse::entities::celestial::{EARTH, MOON};
use gpse::mathematical::expressions::expressions_test;
use gpse::physical::equations::gravitational_force;
use gpse::bible::kjv_1611::test_bible;
//units
//invalid inputs
//changing state/time
//how equations should compose
//how the eventual simulation layer consumes them

fn main() {
    test_bible();
    
    let g_force = gravitational_force(EARTH.mass_kg, MOON.mass_kg, 384_400_000.0);

    println!("{g_force}");

    expressions_test(27.0, 15.0);

    cli_commands();
}
