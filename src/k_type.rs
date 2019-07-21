//! K-Type thermocouple data
use crate::{Celsius, Millivolts, FP};

#[cfg(any(feature = "f32"))]
#[allow(unused_imports)]
use libm::F32Ext;

#[cfg(any(feature = "f64"))]
#[allow(unused_imports)]
use libm::F64Ext;

const K_TYPE_E_BELOW_0: [FP; 11] = [
    0.000000000000E+00,
    0.394501280250E-01,
    0.236223735980E-04,
    -0.328589067840E-06,
    -0.499048287770E-08,
    -0.675090591730E-10,
    -0.574103274280E-12,
    -0.310888728940E-14,
    -0.104516093650E-16,
    -0.198892668780E-19,
    -0.163226974860E-22,
];
const K_TYPE_E_ABOVE_0: [FP; 10] = [
    -0.176004136860E-01,
    0.389212049750E-01,
    0.185587700320E-04,
    -0.994575928740E-07,
    0.318409457190E-09,
    -0.560728448890E-12,
    0.560750590590E-15,
    -0.320207200030E-18,
    0.971511471520E-22,
    -0.121047212750E-25,
];

const K_TYPE_T0: [FP; 10] = [
    0.0000000E+00,
    2.5173462E+01,
    -1.1662878E+00,
    -1.0833638E+00,
    -8.9773540E-01,
    -3.7342377E-01,
    -8.6632643E-02,
    -1.0450598E-02,
    -5.1920577E-04,
    0.0000000E+00,
];
const K_TYPE_T1: [FP; 10] = [
    0.000000E+00,
    2.508355E+01,
    7.860106E-02,
    -2.503131E-01,
    8.315270E-02,
    -1.228034E-02,
    9.804036E-04,
    -4.413030E-05,
    1.057734E-06,
    -1.052755E-08,
];
const K_TYPE_T2: [FP; 10] = [
    -1.318058E+02,
    4.830222E+01,
    -1.646031E+00,
    5.464731E-02,
    -9.650715E-04,
    8.802193E-06,
    -3.110810E-08,
    0.000000E+00,
    0.000000E+00,
    0.000000E+00,
];

/// Evaluate E(T) for a K-type thermocouple in the range -270ºC to
/// 1372ºC, where T is in Celsius and E(T) is in millivolts.
pub fn e(t: Celsius) -> Millivolts {
    let t = t.0;
    #[cfg(not(any(feature = "extrapolate")))]
    assert!(t >= -270.0);
    #[cfg(not(any(feature = "extrapolate")))]
    assert!(t <= 1372.0);

    let e = match t > 0.0 {
        false => {
            // -270ºC -> 0ºC
            const C: [FP; 11] = K_TYPE_E_BELOW_0;

            // Power Series
            let ps = C[0]
                + C[1] * t
                + C[2] * t * t
                + C[3] * t * t * t
                + C[4] * t * t * t * t
                + C[5] * t * t * t * t * t
                + C[6] * t * t * t * t * t * t
                + C[7] * t * t * t * t * t * t * t
                + C[8] * t * t * t * t * t * t * t * t
                + C[9] * t * t * t * t * t * t * t * t * t
                + C[10] * t * t * t * t * t * t * t * t * t * t;

            ps
        }
        _ => {
            // 0ºC -> 1372ºC
            const C: [FP; 10] = K_TYPE_E_ABOVE_0;
            let a0 = 0.118597600000E+00;
            let a1 = -0.118343200000E-03;
            let a2 = 0.126968600000E+03;

            // Power Series
            let ps = C[0]
                + C[1] * t
                + C[2] * t * t
                + C[3] * t * t * t
                + C[4] * t * t * t * t
                + C[5] * t * t * t * t * t
                + C[6] * t * t * t * t * t * t
                + C[7] * t * t * t * t * t * t * t
                + C[8] * t * t * t * t * t * t * t * t
                + C[9] * t * t * t * t * t * t * t * t * t;

            // Exponential
            let es = a0 * (a1 * (t - a2) * (t - a2)).exp();

            ps + es
        }
    };

    Millivolts(e)
}

/// Evaluate T for a K-type thermocouple given E(T) in the range
/// -5.891mV to 54.886mV, where T is in Celsius and E(T) is in millivolts.
pub fn t(e: Millivolts) -> Celsius {
    let e = e.0;
    #[cfg(all(feature = "f32", not(feature = "extrapolate")))]
    const TOL: FP = 0.005; // Tolerance for E(T) range
    #[cfg(all(feature = "f64", not(feature = "extrapolate")))]
    const TOL: FP = 0.0005; // Tolerance for E(T) range

    #[cfg(not(any(feature = "extrapolate")))]
    assert!(e >= -5.891 - TOL);
    #[cfg(not(any(feature = "extrapolate")))]
    assert!(e <= 54.886 + TOL);

    let c = match (e < 0.0, e < 20.644) {
        (true, _) => K_TYPE_T0,
        (false, true) => K_TYPE_T1,
        (false, false) => K_TYPE_T2,
    };

    // Power Series
    let ps = c[0]
        + c[1] * e
        + c[2] * e * e
        + c[3] * e * e * e
        + c[4] * e * e * e * e
        + c[5] * e * e * e * e * e
        + c[6] * e * e * e * e * e * e
        + c[7] * e * e * e * e * e * e * e
        + c[8] * e * e * e * e * e * e * e * e
        + c[9] * e * e * e * e * e * e * e * e * e;

    Celsius(ps)
}

#[cfg(test)]
mod tests {
    use crate::tests::compare;
    use crate::{Celsius, FP};

    nist_its_90! {
        // NIST inverse function is only defined over a smaller range
        "../nist/type_k.tab.rs", -270, 1372, |t| t >= -200 && t < 1372
    }
}
