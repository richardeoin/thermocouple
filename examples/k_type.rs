#![deny(unsafe_code)]

use thermocouple::{prelude::*, KType};

fn main() {
    println!("Type K thermocouple with reference junction at 25ºC");
    let thermocouple = KType::new();

    println!("1.1mV potential");
    let temperature: Celsius =
        thermocouple.sense_temperature(1.1_f64.millivolts());

    println!("{}", temperature);
}
