//! J-Type thermocouple data
use crate::polyval::polyval;
use crate::{Celsius, Millivolts, FP};
const J_TYPE_E_BELOW_760: [FP; 9] = [
    0.000000000000E+00,
    0.503811878150E-01,
    0.304758369300E-04,
    -0.856810657200E-07,
    0.132281952950E-09,
    -0.170529583370E-12,
    0.209480906970E-15,
    -0.125383953360E-18,
    0.156317256970E-22,
];
const J_TYPE_E_ABOVE_760: [FP; 6] = [
    0.296456256810E+03,
    -0.149761277860E+01,
    0.317871039240E-02,
    -0.318476867010E-05,
    0.157208190040E-08,
    -0.306913690560E-12,
];

const J_TYPE_T0: [FP; 9] = [
    0.0000000E+00,
    1.9528268E+01,
    -1.2286185E+00,
    -1.0752178E+00,
    -5.9086933E-01,
    -1.7256713E-01,
    -2.8131513E-02,
    -2.3963370E-03,
    -8.3823321E-05,
];
const J_TYPE_T1: [FP; 9] = [
    0.000000E+00,
    1.978425E+01,
    -2.001204E-01,
    1.036969E-02,
    -2.549687E-04,
    3.585153E-06,
    -5.344285E-08,
    5.099890E-10,
    0.000000E+00,
];
const J_TYPE_T2: [FP; 9] = [
    -3.11358187E+03,
    3.00543684E+02,
    -9.94773230E+00,
    1.70276630E-01,
    -1.43033468E-03,
    4.73886084E-06,
    0.00000000E+00,
    0.00000000E+00,
    0.00000000E+00,
];

/// Evaluate E(T) for a J-Type thermocouple in the range -210ºC to
/// 1200ºC, where T is in Celsius and E(T) is in millivolts.
pub fn e(t: Celsius) -> Millivolts {
    let t = t.0;
    #[cfg(not(any(feature = "extrapolate")))]
    assert!(t >= -210.0);
    #[cfg(not(any(feature = "extrapolate")))]
    assert!(t <= 1200.0);

    let e = match t > 760.0 {
        false => {
            // -210ºC -> 760ºC
            polyval(J_TYPE_E_BELOW_760, t)
        }
        _ => {
            // 760ºC -> 1200ºC
            polyval(J_TYPE_E_ABOVE_760, t)
        }
    };

    Millivolts(e)
}

/// Evaluate T for a J-Type thermocouple given E(T) in the range
/// -8.095mV to 69.553mV, where T is in Celsius and E(T) is in millivolts.
pub fn t(e: Millivolts) -> Celsius {
    let e = e.0;
    #[cfg(all(feature = "f32", not(feature = "extrapolate")))]
    const TOL: FP = 0.005; // Tolerance for E(T) range
    #[cfg(all(feature = "f64", not(feature = "extrapolate")))]
    const TOL: FP = 0.0005; // Tolerance for E(T) range

    #[cfg(not(any(feature = "extrapolate")))]
    assert!(e >= -8.095 - TOL);
    #[cfg(not(any(feature = "extrapolate")))]
    assert!(e <= 69.553 + TOL);

    let c = match (e < 0.0, e < 42.919) {
        (true, _) => J_TYPE_T0,
        (false, true) => J_TYPE_T1,
        (false, false) => J_TYPE_T2,
    };

    let ps = polyval(c, e);

    Celsius(ps)
}

#[cfg(test)]
mod tests {
    use crate::tests::compare;
    use crate::{Celsius, FP};

    nist_its_90! {
        // NIST inverse function is only defined over a smaller range
        "../nist/type_j.tab.rs", -210, 1200, |t| t <= 1200
    }
}
