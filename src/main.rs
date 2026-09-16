//use std::f64::consts::PI;

fn main() {
    //speed_of_light_is_deterministic();
    gravity();
}

//use gpse::physical::constants::SPEED_OF_LIGHT_IN_VACUUM;

//#[test]
//fn speed_of_light_is_deterministic() {
//    assert_eq!(SPEED_OF_LIGHT_IN_VACUUM.value, 299_792_458.0);
//}

use gpse::physical::constants::NEWTONIAN_CONSTANT_OF_GRAVITATION;

fn gravity() {
    let gravitational_constant = NEWTONIAN_CONSTANT_OF_GRAVITATION.value;
    println!("{gravitational_constant}"); 
}