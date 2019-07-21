//! S-Type thermocouple data
use crate::{Celsius, Millivolts, FP};

const S_TYPE_E_BELOW_1064_18: [FP; 9] = [
    0.000000000000E+00,
    0.540313308631E-02,
    0.125934289740E-04,
    -0.232477968689E-07,
    0.322028823036E-10,
    -0.331465196389E-13,
    0.255744251786E-16,
    -0.125068871393E-19,
    0.271443176145E-23,
];
const S_TYPE_E_ABOVE_1064_18_BELOW_1664_5: [FP; 5] = [
    0.132900444085E+01,
    0.334509311344E-02,
    0.654805192818E-05,
    -0.164856259209E-08,
    0.129989605174E-13,
];
const S_TYPE_E_ABOVE_1664_5: [FP; 5] = [
    0.146628232636E+03,
    -0.258430516752E+00,
    0.163693574641E-03,
    -0.330439046987E-07,
    -0.943223690612E-14,
];

const S_TYPE_T0: [FP; 10] = [
    0.00000000E+00,
    1.84949460E+02,
    -8.00504062E+01,
    1.02237430E+02,
    -1.52248592E+02,
    1.88821343E+02,
    -1.59085941E+02,
    8.23027880E+01,
    -2.34181944E+01,
    2.79786260E+00,
];
const S_TYPE_T1: [FP; 10] = [
    1.291507177E+01,
    1.466298863E+02,
    -1.534713402E+01,
    3.145945973E+00,
    -4.163257839E-01,
    3.187963771E-02,
    -1.291637500E-03,
    2.183475087E-05,
    -1.447379511E-07,
    8.211272125E-09,
];
const S_TYPE_T2: [FP; 10] = [
    -8.087801117E+01,
    1.621573104E+02,
    -8.536869453E+00,
    4.719686976E-01,
    -1.441693666E-02,
    2.081618890E-04,
    0.000000000E+00,
    0.000000000E+00,
    0.000000000E+00,
    0.000000000E+00,
];
const S_TYPE_T3: [FP; 10] = [
    5.333875126E+04,
    -1.235892298E+04,
    1.092657613E+03,
    -4.265693686E+01,
    6.247205420E-01,
    0.000000000E+00,
    0.000000000E+00,
    0.000000000E+00,
    0.000000000E+00,
    0.000000000E+00,
];

/// Evaluate E(T) for a S-Type thermocouple in the range -50ºC to
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
            const C: [FP; 9] = S_TYPE_E_BELOW_1064_18;

            // Power Series
            C[0] + C[1] * t
                + C[2] * t * t
                + C[3] * t * t * t
                + C[4] * t * t * t * t
                + C[5] * t * t * t * t * t
                + C[6] * t * t * t * t * t * t
                + C[7] * t * t * t * t * t * t * t
                + C[8] * t * t * t * t * t * t * t * t
        }
        (true, false) => {
            // 1064.18ºC -> 1664.5ºC
            const C: [FP; 5] = S_TYPE_E_ABOVE_1064_18_BELOW_1664_5;

            // Power Series
            C[0] + C[1] * t
                + C[2] * t * t
                + C[3] * t * t * t
                + C[4] * t * t * t * t
        }
        (true, true) => {
            // 1664.5ºC -> 1768.1ºC
            const C: [FP; 5] = S_TYPE_E_ABOVE_1664_5;

            // Power Series
            C[0] + C[1] * t
                + C[2] * t * t
                + C[3] * t * t * t
                + C[4] * t * t * t * t
        }
    };

    Millivolts(e)
}

/// Evaluate T for a S-Type thermocouple given E(T) in the range
/// -0.235mV to 18.693mV, where T is in Celsius and E(T) is in millivolts.
pub fn t(e: Millivolts) -> Celsius {
    let e = e.0;
    #[cfg(all(feature = "f32", not(feature = "extrapolate")))]
    const TOL: FP = 0.005; // Tolerance for E(T) range
    #[cfg(all(feature = "f64", not(feature = "extrapolate")))]
    const TOL: FP = 0.00056; // Tolerance for E(T) range

    #[cfg(not(any(feature = "extrapolate")))]
    assert!(e >= -0.235 - TOL);
    #[cfg(not(any(feature = "extrapolate")))]
    assert!(e <= 18.693 + TOL);

    let c = match (e < 1.874, e < 11.950, e < 17.536) {
        (true, _, _) => S_TYPE_T0,
        (false, true, _) => S_TYPE_T1,
        (false, false, true) => S_TYPE_T2,
        (false, false, false) => S_TYPE_T3,
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
        // NIST inverse function is defined over full range
        "../nist/type_s.tab.rs", -50, 1768, |_| true
    }
}
