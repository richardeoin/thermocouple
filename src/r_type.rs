//! R-Type thermocouple data
use crate::{polyval::polyval, Celsius, Millivolts, FP};

const R_TYPE_E_BELOW_1064_18: [FP; 10] = [
    0.000000000000E+00,
    0.528961729765E-02,
    0.139166589782E-04,
    -0.238855693017E-07,
    0.356916001063E-10,
    -0.462347666298E-13,
    0.500777441034E-16,
    -0.373105886191E-19,
    0.157716482367E-22,
    -0.281038625251E-26,
];
const R_TYPE_E_ABOVE_1064_18_BELOW_1664_5: [FP; 6] = [
    0.295157925316E+01,
    -0.252061251332E-02,
    0.159564501865E-04,
    -0.764085947576E-08,
    0.205305291024E-11,
    -0.293359668173E-15,
];
const R_TYPE_E_ABOVE_1664_5: [FP; 5] = [
    0.152232118209E+03,
    -0.268819888545E+00,
    0.171280280471E-03,
    -0.345895706453E-07,
    -0.934633971046E-14,
];

const R_TYPE_T0: [FP; 11] = [
    0.0000000E+00,
    1.8891380E+02,
    -9.3835290E+01,
    1.3068619E+02,
    -2.2703580E+02,
    3.5145659E+02,
    -3.8953900E+02,
    2.8239471E+02,
    -1.2607281E+02,
    3.1353611E+01,
    -3.3187769E+00,
];
const R_TYPE_T1: [FP; 11] = [
    1.334584505E+01,
    1.472644573E+02,
    -1.844024844E+01,
    4.031129726E+00,
    -6.249428360E-01,
    6.468412046E-02,
    -4.458750426E-03,
    1.994710149E-04,
    -5.313401790E-06,
    6.481976217E-08,
    0.000000000E+00,
];
const R_TYPE_T2: [FP; 11] = [
    -8.199599416E+01,
    1.553962042E+02,
    -8.342197663E+00,
    4.279433549E-01,
    -1.191577910E-02,
    1.492290091E-04,
    0.000000000E+00,
    0.000000000E+00,
    0.000000000E+00,
    0.000000000E+00,
    0.000000000E+00,
];
const R_TYPE_T3: [FP; 11] = [
    3.406177836E+04,
    -7.023729171E+03,
    5.582903813E+02,
    -1.952394635E+01,
    2.560740231E-01,
    0.000000000E+00,
    0.000000000E+00,
    0.000000000E+00,
    0.000000000E+00,
    0.000000000E+00,
    0.000000000E+00,
];

/// Evaluate E(T) for a R-Type thermocouple in the range -50ºC to
/// 1768.1ºC, where T is in Celsius and E(T) is in millivolts.
pub fn e(t: Celsius) -> Millivolts {
    let t = t.0;
    #[cfg(not(any(feature = "extrapolate")))]
    assert!(t >= -50.0);
    #[cfg(not(any(feature = "extrapolate")))]
    assert!(t <= 1768.1);

    let e = match (t > 1064.18, t > 1664.5) {
        (false, _) => {
            // -50ºC -> 1064.18ºC
            polyval(R_TYPE_E_BELOW_1064_18, t)
        }
        (true, false) => {
            // 1064.18ºC -> 1664.5ºC
            polyval(R_TYPE_E_ABOVE_1064_18_BELOW_1664_5, t)
        }
        (true, true) => {
            // 1664.5ºC -> 1768.1ºC
            polyval(R_TYPE_E_ABOVE_1664_5, t)
        }
    };

    Millivolts(e)
}

/// Evaluate T for a R-Type thermocouple given E(T) in the range
/// -0.226mV to 21.103mV, where T is in Celsius and E(T) is in millivolts.
pub fn t(e: Millivolts) -> Celsius {
    let e = e.0;
    #[cfg(all(feature = "f32", not(feature = "extrapolate")))]
    const TOL: FP = 0.005; // Tolerance for E(T) range
    #[cfg(all(feature = "f64", not(feature = "extrapolate")))]
    const TOL: FP = 0.0005; // Tolerance for E(T) range

    #[cfg(not(any(feature = "extrapolate")))]
    assert!(e >= -0.226 - TOL);
    #[cfg(not(any(feature = "extrapolate")))]
    assert!(e <= 21.103 + TOL);

    let c = match (e < 1.923, e < 13.228, e < 19.739) {
        (true, _, _) => R_TYPE_T0,
        (false, true, _) => R_TYPE_T1,
        (false, false, true) => R_TYPE_T2,
        (false, false, false) => R_TYPE_T3,
    };

    let ps = polyval(c, e);

    Celsius(ps)
}

#[cfg(test)]
mod tests {
    use crate::tests::compare;
    use crate::{Celsius, FP};

    nist_its_90! {
        // NIST inverse function is defined over full range
        "../nist/type_r.tab.rs", -50, 1768, |_| true
    }
}
