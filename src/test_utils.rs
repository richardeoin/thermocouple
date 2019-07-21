//! Utilities for running unit tests.

#[allow(unused_macros)]

/// Run a unit test against the NIST ITS-90 Thermocouple Database
macro_rules! nist_its_90 {
    ($tab_filename:expr, $low:expr, $high:expr, $t_defined:expr) => {
        /// Tolerances:
        #[cfg(any(feature = "f32"))]
        const E_ERROR_MAX: FP = 0.1; // ±0.1mV maximum error
        #[cfg(any(feature = "f32"))]
        const T_ERROR_MAX: FP = 0.25; // ±0.25ºC maximum error

        #[cfg(any(feature = "f64"))]
        /// The maximum error should be ±0.5µV, so that the results
        /// matches the ITS-90 tables exactly
        const E_ERROR_MAX: FP = 0.0005; // ±0.5µV maximum error
        #[cfg(any(feature = "f64"))]
        const T_ERROR_MAX: FP = 0.05; // ±0.05ºC maximum error

        #[test]
        fn test_e() {
            let nist_tab_e = include!($tab_filename);

            for (t, e) in ($low..).zip(nist_tab_e.iter()) {
                let t_tab = Celsius(t as FP);
                let e_calc = super::e(t_tab);

                println!("{:?}:", t_tab);
                println!("{:?}, {:?}", e, e_calc);
                println!("");

                compare(*e, e_calc.0, E_ERROR_MAX);
            }
        }

        #[test]
        fn test_t() {
            let nist_tab_e = include!($tab_filename);

            for (t, _) in ($low..).zip(nist_tab_e.iter()) {
                // NIST inverse function may only be defined over a
                // smaller range
                let is_t_defined = ($t_defined)(t);

                if is_t_defined {
                    let t_tab = Celsius(t as FP);
                    let e_calc = super::e(t_tab);
                    println!("{:?}:", e_calc);

                    let t_calc = super::t(e_calc);
                    println!("{:?}, {:?}", t, t_calc);
                    println!("");

                    compare(t as FP, t_calc.0, T_ERROR_MAX);
                }
            }
        }

        #[test]
        #[should_panic]
        #[cfg(not(any(feature = "extrapolate")))]
        fn test_c_too_low() {
            let _ = super::e(Celsius(($low as FP) - 1.0));
        }

        #[test]
        #[should_panic]
        #[cfg(not(any(feature = "extrapolate")))]
        fn test_c_too_high() {
            let _ = super::e(Celsius(($high as FP) + 1.0));
        }
    };
}
