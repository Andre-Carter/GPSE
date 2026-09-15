mod chemical;
//mod mathematical;
mod physical;

use std::f64::consts::PI;

fn main() {
    let x = 10.0 * PI;
    println!("{x}");
}

use gpse::physical::constants::SPEED_OF_LIGHT_IN_VACUUM;

#[test]
fn speed_of_light_is_deterministic() {
    assert_eq!(SPEED_OF_LIGHT_IN_VACUUM.value, 299_792_458.0);
}
