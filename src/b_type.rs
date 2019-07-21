//! B-Type thermocouple data
use crate::{Celsius, Millivolts, FP};

const B_TYPE_E_BELOW_630_615: [FP; 7] = [
    0.000000000000E+00,
    -0.246508183460E-03,
    0.590404211710E-05,
    -0.132579316360E-08,
    0.156682919010E-11,
    -0.169445292400E-14,
    0.629903470940E-18,
];
const B_TYPE_E_ABOVE_630_615: [FP; 9] = [
    -0.389381686210E+01,
    0.285717474700E-01,
    -0.848851047850E-04,
    0.157852801640E-06,
    -0.168353448640E-09,
    0.111097940130E-12,
    -0.445154310330E-16,
    0.989756408210E-20,
    -0.937913302890E-24,
];

const B_TYPE_T0: [FP; 9] = [
    9.8423321E+01,
    6.9971500E+02,
    -8.4765304E+02,
    1.0052644E+03,
    -8.3345952E+02,
    4.5508542E+02,
    -1.5523037E+02,
    2.9886750E+01,
    -2.4742860E+00,
];
const B_TYPE_T1: [FP; 9] = [
    2.1315071E+02,
    2.8510504E+02,
    -5.2742887E+01,
    9.9160804E+00,
    -1.2965303E+00,
    1.1195870E-01,
    -6.0625199E-03,
    1.8661696E-04,
    -2.4878585E-06,
];

/// Evaluate E(T) for a B-Type thermocouple in the range 0ºC to
/// 1820ºC, where T is in Celsius and E(T) is in millivolts.
pub fn e(t: Celsius) -> Millivolts {
    let t = t.0;
    #[cfg(not(any(feature = "extrapolate")))]
    assert!(t >= 0.0);
    #[cfg(not(any(feature = "extrapolate")))]
    assert!(t <= 1820.0);

    let e = match t > 630.615 {
        false => {
            // 0ºC -> 630.615ºC
            const C: [FP; 7] = B_TYPE_E_BELOW_630_615;

            // Power Series
            C[0] + C[1] * t
                + C[2] * t * t
                + C[3] * t * t * t
                + C[4] * t * t * t * t
                + C[5] * t * t * t * t * t
                + C[6] * t * t * t * t * t * t
        }
        _ => {
            // 630.615ºC -> 1820ºC
            const C: [FP; 9] = B_TYPE_E_ABOVE_630_615;

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
    };

    Millivolts(e)
}

/// Evaluate T for a B-Type thermocouple given E(T) in the range
/// 0.291mV to 13.280mV, where T is in Celsius and E(T) is in millivolts.
pub fn t(e: Millivolts) -> Celsius {
    let e = e.0;
    #[cfg(all(feature = "f32", not(feature = "extrapolate")))]
    const TOL: FP = 0.005; // Tolerance for E(T) range
    #[cfg(all(feature = "f64", not(feature = "extrapolate")))]
    const TOL: FP = 0.0005; // Tolerance for E(T) range

    #[cfg(not(any(feature = "extrapolate")))]
    assert!(e >= 0.291 - TOL);
    #[cfg(not(any(feature = "extrapolate")))]
    assert!(e <= 13.82 + TOL);

    let c = match e < 2.431 {
        true => B_TYPE_T0,
        false => B_TYPE_T1,
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
        + c[8] * e * e * e * e * e * e * e * e;

    Celsius(ps)
}

#[cfg(test)]
mod tests {
    use crate::tests::compare;
    use crate::{Celsius, FP};

    nist_its_90! {
        // NIST inverse function is only defined over a smaller range
        "../nist/type_b.tab.rs", 0, 1820, |t| t >= 250 && t < 1820
    }
}
