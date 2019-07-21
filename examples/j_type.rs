#![deny(unsafe_code)]

use thermocouple::{prelude::*, JType};

fn main() {
    println!("Type J thermocouple with reference junction at 0ºC");
    let thermocouple =
        JType::new().with_reference_temperature(0_f64.celsius());

    println!("1.1mV potential");
    let temperature: Celsius =
        thermocouple.sense_temperature(1.1_f64.millivolts());

    println!("{}", temperature);
}
