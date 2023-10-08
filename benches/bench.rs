#![feature(test)]

extern crate test;
extern crate thermocouple;

use test::Bencher;
use thermocouple::prelude::*;

///
/// Type K
///
#[bench]
fn bench_type_k_celcius(b: &mut Bencher) {
    let thermocouple = thermocouple::KType::new();

    b.iter(|| {
        for i in 0..1000000 {
            let _: Celsius =
                thermocouple.sense_temperature(Millivolts(2.0));
        }
    });
}
///
/// Type J
///
#[bench]
fn bench_type_j_celcius(b: &mut Bencher) {
    let thermocouple = thermocouple::JType::new();

    b.iter(|| {
        let _: Celsius =
            thermocouple.sense_temperature(Millivolts(2.0));
    });
}
