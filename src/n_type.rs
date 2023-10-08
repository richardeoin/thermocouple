//! N-Type thermocouple data
use crate::polyval::polyval;
use crate::{Celsius, Millivolts, FP};
const N_TYPE_E_BELOW_0: [FP; 9] = [
    0.000000000000E+00,
    0.261591059620E-01,
    0.109574842280E-04,
    -0.938411115540E-07,
    -0.464120397590E-10,
    -0.263033577160E-11,
    -0.226534380030E-13,
    -0.760893007910E-16,
    -0.934196678350E-19,
];
const N_TYPE_E_ABOVE_0: [FP; 11] = [
    0.000000000000E+00,
    0.259293946010E-01,
    0.157101418800E-04,
    0.438256272370E-07,
    -0.252611697940E-09,
    0.643118193390E-12,
    -0.100634715190E-14,
    0.997453389920E-18,
    -0.608632456070E-21,
    0.208492293390E-24,
    -0.306821961510E-28,
];

const N_TYPE_T0: [FP; 10] = [
    0.0000000E+00,
    3.8436847E+01,
    1.1010485E+00,
    5.2229312E+00,
    7.2060525E+00,
    5.8488586E+00,
    2.7754916E+00,
    7.7075166E-01,
    1.1582665E-01,
    7.3138868E-03,
];
const N_TYPE_T1: [FP; 10] = [
    0.00000E+00,
    3.86896E+01,
    -1.08267E+00,
    4.70205E-02,
    -2.12169E-06,
    -1.17272E-04,
    5.39280E-06,
    -7.98156E-08,
    0.00000E+00,
    0.00000E+00,
];
const N_TYPE_T2: [FP; 10] = [
    1.972485E+01,
    3.300943E+01,
    -3.915159E-01,
    9.855391E-03,
    -1.274371E-04,
    7.767022E-07,
    0.000000E+00,
    0.000000E+00,
    0.000000E+00,
    0.000000E+00,
];

/// Evaluate E(T) for a N-Type thermocouple in the range -270ºC to
/// 1300ºC, where T is in Celsius and E(T) is in millivolts.
pub fn e(t: Celsius) -> Millivolts {
    let t = t.0;
    #[cfg(not(any(feature = "extrapolate")))]
    assert!(t >= -270.0);
    #[cfg(not(any(feature = "extrapolate")))]
    assert!(t <= 1300.0);

    let e = match t > 0.0 {
        false => {
            // -270ºC -> 0ºC

            polyval(N_TYPE_E_BELOW_0, t)
        }
        _ => {
            // 0ºC -> 1300ºC
            polyval(N_TYPE_E_ABOVE_0, t)
        }
    };

    Millivolts(e)
}

/// Evaluate T for a N-Type thermocouple given E(T) in the range
/// -3.990mV to 47.513mV, where T is in Celsius and E(T) is in millivolts.
pub fn t(e: Millivolts) -> Celsius {
    let e = e.0;
    #[cfg(all(feature = "f32", not(feature = "extrapolate")))]
    const TOL: FP = 0.005; // Tolerance for E(T) range
    #[cfg(all(feature = "f64", not(feature = "extrapolate")))]
    const TOL: FP = 0.0005; // Tolerance for E(T) range

    #[cfg(not(any(feature = "extrapolate")))]
    assert!(e >= -3.990 - TOL);
    #[cfg(not(any(feature = "extrapolate")))]
    assert!(e <= 47.513 + TOL);

    let c = match (e < 0.0, e < 20.613) {
        (true, _) => N_TYPE_T0,
        (false, true) => N_TYPE_T1,
        (false, false) => N_TYPE_T2,
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
        "../nist/type_n.tab.rs", -270, 1300, |t| t >= -200 && t <= 1300
    }
}
