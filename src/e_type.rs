//! E-Type thermocouple data
use crate::polyval::polyval;
use crate::{Celsius, Millivolts, FP};
const E_TYPE_E_BELOW_0: [FP; 14] = [
    0.000000000000E+00,
    0.586655087080E-01,
    0.454109771240E-04,
    -0.779980486860E-06,
    -0.258001608430E-07,
    -0.594525830570E-09,
    -0.932140586670E-11,
    -0.102876055340E-12,
    -0.803701236210E-15,
    -0.439794973910E-17,
    -0.164147763550E-19,
    -0.396736195160E-22,
    -0.558273287210E-25,
    -0.346578420130E-28,
];
const E_TYPE_E_ABOVE_0: [FP; 11] = [
    0.000000000000E+00,
    0.586655087100E-01,
    0.450322755820E-04,
    0.289084072120E-07,
    -0.330568966520E-09,
    0.650244032700E-12,
    -0.191974955040E-15,
    -0.125366004970E-17,
    0.214892175690E-20,
    -0.143880417820E-23,
    0.359608994810E-27,
];

const E_TYPE_T0: [FP; 10] = [
    0.0000000E+00,
    1.6977288E+01,
    -4.3514970E-01,
    -1.5859697E-01,
    -9.2502871E-02,
    -2.6084314E-02,
    -4.1360199E-03,
    -3.4034030E-04,
    -1.1564890E-05,
    0.0000000E+00,
];
const E_TYPE_T1: [FP; 10] = [
    0.0000000E+00,
    1.7057035E+01,
    -2.3301759E-01,
    6.5435585E-03,
    -7.3562749E-05,
    -1.7896001E-06,
    8.4036165E-08,
    -1.3735879E-09,
    1.0629823E-11,
    -3.2447087E-14,
];

/// Evaluate E(T) for a E-Type thermocouple in the range 0ºC to
/// 1820ºC, where T is in Celsius and E(T) is in millivolts.
pub fn e(t: Celsius) -> Millivolts {
    let t = t.0;
    #[cfg(not(any(feature = "extrapolate")))]
    assert!(t >= -270.0);
    #[cfg(not(any(feature = "extrapolate")))]
    assert!(t <= 1000.0);

    let e = match t > 0.0 {
        false => {
            // -270ºC -> 0ºC
            polyval(E_TYPE_E_BELOW_0, t)
        }
        _ => {
            // 0ºC -> 1000ºC
            polyval(E_TYPE_E_ABOVE_0, t)
        }
    };

    Millivolts(e)
}

/// Evaluate T for a E-Type thermocouple given E(T) in the range
/// -8.825mV to 76.373mV, where T is in Celsius and E(T) is in millivolts.
pub fn t(e: Millivolts) -> Celsius {
    let e = e.0;
    #[cfg(all(feature = "f32", not(feature = "extrapolate")))]
    const TOL: FP = 0.005; // Tolerance for E(T) range
    #[cfg(all(feature = "f64", not(feature = "extrapolate")))]
    const TOL: FP = 0.0005; // Tolerance for E(T) range

    #[cfg(not(any(feature = "extrapolate")))]
    assert!(e >= -8.825 - TOL);
    #[cfg(not(any(feature = "extrapolate")))]
    assert!(e <= 76.373 + TOL);

    let ps = match e < 0.0 {
        true => polyval(E_TYPE_T0, e),
        false => polyval(E_TYPE_T1, e),
    };
    Celsius(ps)
}

#[cfg(test)]
mod tests {
    use crate::tests::compare;
    use crate::{Celsius, FP};

    nist_its_90! {
        // NIST inverse function is only defined over a smaller range
        "../nist/type_e.tab.rs", -270, 1000, |t| t >= -200 && t <= 1000
    }
}
