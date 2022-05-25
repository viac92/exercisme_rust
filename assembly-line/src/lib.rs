// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub fn production_rate_per_hour(speed: u8) -> f64 {
    let mut speed = speed as f64;
    let mut returns: f64; 

    if speed < 5.0 {
        returns = speed * 221.0;

    } else if speed > 8.0 { 
        if speed > 10.0 {
            speed = 10.0;
        }
        returns = speed * 221.0 * 0.77;

    } else {
        returns = speed * 221.0 * 0.90;
    }
    returns
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    production_rate_per_hour(speed) as u32 / 60
}


