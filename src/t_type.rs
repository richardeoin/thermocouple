//! T-Type thermocouple data
use crate::{polyval::polyval, Celsius, Millivolts, FP};

const T_TYPE_E_BELOW_0: [FP; 15] = [
    0.000000000000E+00,
    0.387481063640E-01,
    0.441944343470E-04,
    0.118443231050E-06,
    0.200329735540E-07,
    0.901380195590E-09,
    0.226511565930E-10,
    0.360711542050E-12,
    0.384939398830E-14,
    0.282135219250E-16,
    0.142515947790E-18,
    0.487686622860E-21,
    0.107955392700E-23,
    0.139450270620E-26,
    0.797951539270E-30,
];
const T_TYPE_E_ABOVE_0: [FP; 9] = [
    0.000000000000E+00,
    0.387481063640E-01,
    0.332922278800E-04,
    0.206182434040E-06,
    -0.218822568460E-08,
    0.109968809280E-10,
    -0.308157587720E-13,
    0.454791352900E-16,
    -0.275129016730E-19,
];

const T_TYPE_T0: [FP; 8] = [
    0.0000000E+00,
    2.5949192E+01,
    -2.1316967E-01,
    7.9018692E-01,
    4.2527777E-01,
    1.3304473E-01,
    2.0241446E-02,
    1.2668171E-03,
];
const T_TYPE_T1: [FP; 8] = [
    0.000000E+00,
    2.592800E+01,
    -7.602961E-01,
    4.637791E-02,
    -2.165394E-03,
    6.048144E-05,
    -7.293422E-07,
    0.000000E+00,
];

/// Evaluate E(T) for a T-Type thermocouple in the range -270ºC to
/// 400ºC, where T is in Celsius and E(T) is in millivolts.
pub fn e(t: Celsius) -> Millivolts {
    let t = t.0;
    #[cfg(not(any(feature = "extrapolate")))]
    assert!(t >= -270.0);
    #[cfg(not(any(feature = "extrapolate")))]
    assert!(t <= 400.0);

    let e = match t > 0.0 {
        false => {
            // -270ºC -> 0ºC
            polyval(T_TYPE_E_BELOW_0, t)
        }
        _ => {
            // 0ºC -> 400ºC
            polyval(T_TYPE_E_ABOVE_0, t)
        }
    };

    Millivolts(e)
}

/// Evaluate T for a T-Type thermocouple given E(T) in the range
/// -5.603mV to 20.872mV, where T is in Celsius and E(T) is in millivolts.
pub fn t(e: Millivolts) -> Celsius {
    let e = e.0;
    #[cfg(all(feature = "f32", not(feature = "extrapolate")))]
    const TOL: FP = 0.005; // Tolerance for E(T) range
    #[cfg(all(feature = "f64", not(feature = "extrapolate")))]
    const TOL: FP = 0.0005; // Tolerance for E(T) range

    #[cfg(not(any(feature = "extrapolate")))]
    assert!(e >= -5.603 - TOL);
    #[cfg(not(any(feature = "extrapolate")))]
    assert!(e <= 20.872 + TOL);

    let c = match e < 0.0 {
        true => T_TYPE_T0,
        false => T_TYPE_T1,
    };

    // Power Series
    let ps = polyval(c, e);

    Celsius(ps)
}

#[cfg(test)]
mod tests {
    use crate::tests::compare;
    use crate::{Celsius, FP};

    nist_its_90! {
        // NIST inverse function is only defined over a smaller range
        "../nist/type_t.tab.rs", -270, 400, |t| t >= -200
    }
}
