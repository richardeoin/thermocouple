//! Thermocouple routines.
//!
//! Provides a `sense_temperature` method to convert thermoelectric
//! potential to temperature. The following thermocouple types are supported:
//!
//! * Nickel-alloy thermocouples: Type E, J, K, N, T
//! * Platinum/rhodium-alloy thermocouples: Type B, R, S
//!
//! This library includes _newtype_ types, or wrapper types, to help with using
//! the proper units when doing calculations. I investigated using a dimensional
//! analysis crate such as uom or dimensioned instead of making my own newtype
//! types. However after experimentation, I found this to be difficult to use
//! and not at all lightweight.
//!
//! The underlying storage type is either `f64` (default) or `f32`. For the
//! `f64` storage type, the results from this crate match the [NIST ITS-90
//! Thermocouple Database][ITS-90] exactly.
//!
//! ## Usage
//! Add this to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! thermocouple = "0.1.3"
//! ```
//!
//! and this to your crate root:
//!
//! ```rust
//! extern crate thermocouple;
//! ```
//!
//! The temperature of a thermocouple can be calcuated from a voltage reading.
//!
#![cfg_attr(any(feature = "k-type"), doc = " ```rust")]
#![cfg_attr(not(any(feature = "k-type")), doc = " ```rust,ignore")]
//! use thermocouple::{prelude::*, KType};
//!
//! // Assuming reference junction at 25ºC
//! let temperature: Celsius = KType::new().sense_temperature(Millivolts(1.1));
//! ```
//!
//! The reference junction temperature can be set explicitly.
//!
#![cfg_attr(any(feature = "k-type"), doc = " ```rust")]
#![cfg_attr(not(any(feature = "k-type")), doc = " ```rust,ignore")]
//! use thermocouple::{prelude::*, KType};
//!
//! // Reference junction at 0ºC
//! let thermocouple =
//!     KType::new().with_reference_temperature(Celsius(0.0));
//!
//! let temperature: Celsius =
//!     thermocouple.sense_temperature(Millivolts(2.0));
//! ```
//!
//! ## Tests
//!
//! The tests check against every value provided in the [NIST ITS-90
//! Thermocouple Database][ITS-90] (DOI:
//! [http://dx.doi.org/10.18434/T4S888](http://dx.doi.org/10.18434/T4S888))
//!
//! For a `f64` base storage type, they should match the table
//! exactly. Re-formatted versions of the tables for the automatic tests can be
//! found in the `nist` directory.
//!
//! [ITS-90]: https://srdata.nist.gov/its90/main/its90_main_page.html
//!

#![no_std]
// rustc lints.
#![forbid(unsafe_code)]
#![warn(
    bare_trait_objects,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    trivial_casts,
    trivial_numeric_casts,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

#[cfg(test)]
#[macro_use]
extern crate std;

#[cfg(any(feature = "f32"))]
#[doc = "Underlying storage type: `f32`"]
pub type FP = f32;
#[cfg(any(feature = "f64"))]
#[doc = "Underlying storage type: `f64`"]
pub type FP = f64;
#[cfg(not(any(feature = "f32", feature = "f64")))]
compile_error!(
    "A least one underlying storage type must be enabled.
Use a feature gate to enable."
);
#[cfg(all(feature = "f32", feature = "f64"))]
compile_error!(
    "Cannot enable multiple storage types. Try setting
`default-features = false`."
);

#[macro_use]
mod test_utils;
mod polyval;
mod units;
pub use units::{
    Celsius, FPExt, Fahrenheit, Kelvin, Millivolts, Rankine, Reaumur,
};

/// Trait for thermocouple functionality
pub trait ThermocoupleCore<W> {
    /// Returns the thermocouple temperature for a given
    /// thermoelectric potential.
    fn sense_temperature(&self, voltage: Millivolts) -> W;
    /// Return the thermoelectric potential for a given thermocouple
    /// temperature.
    fn sense_voltage(&self, temperature: W) -> Millivolts;
}

macro_rules! thermocouple {
    ($($Type:ident, $mod:ident: $doc:expr => $($unit:ty),+;)*) => {
        $(
            mod $mod;

            #[doc=$doc]
            #[derive(Clone, Copy, Debug)]
            pub struct $Type {
                /// E(T) at the reference junction
                reference_potential: Millivolts,
            }
            impl $Type {
                /// New thermocouple instance. The reference junction is
                /// assumed to be at 25ºC / 298.15K.
                pub fn new() -> $Type {
                    $Type {
                        reference_potential: $mod::e((25.0).celsius()),
                    }
                }
                /// Sets the reference junction temperature used.
                pub fn with_reference_temperature<T>(
                    self,
                    reference_temperature: T,
                ) -> Self where
                    T: Into<Celsius>,
                {
                    $Type {
                        reference_potential: $mod::e(reference_temperature.into()),
                    }
                }
            }
            impl Default for $Type {
                fn default() -> Self {
                    $Type::new()
                }
            }

            $(
                impl ThermocoupleCore<$unit> for $Type {
                    /// Return the thermocouple temperature for a
                    /// given thermoelectric potential.
                    fn sense_temperature(
                        &self,
                        voltage: Millivolts,
                    ) -> $unit {
                        $mod::t(voltage + self.reference_potential).into()
                    }
                    /// Return the thermoelectric potential for a
                    /// given thermocouple temperature.
                    fn sense_voltage(&self, temperature: $unit) -> Millivolts {
                        $mod::e(temperature.into()) - self.reference_potential
                    }
                }
            )+
        )*
    };
}

#[cfg(any(feature = "k-type"))]
thermocouple! {
    KType, k_type: "Type K thermocouple (chromel-alumel)" =>
        Celsius, Kelvin, Fahrenheit, Rankine, Reaumur;
}

thermocouple! {
    BType, b_type: "Type B thermocouple (platinum/rhodium alloy)" =>
        Celsius, Kelvin, Fahrenheit, Rankine, Reaumur;

    EType, e_type: "Type E thermocouple (chromel-constantan)" =>
        Celsius, Kelvin, Fahrenheit, Rankine, Reaumur;

    JType, j_type: "Type J thermocouple (iron-constantan)" =>
        Celsius, Kelvin, Fahrenheit, Rankine, Reaumur;

    NType, n_type: "Type N thermocouple (nicrosil-nisil)" =>
        Celsius, Kelvin, Fahrenheit, Rankine, Reaumur;

    RType, r_type: "Type R thermocouple (platinum/rhodium alloy)" =>
        Celsius, Kelvin, Fahrenheit, Rankine, Reaumur;

    SType, s_type: "Type S thermocouple (platinum/rhodium alloy)" =>
        Celsius, Kelvin, Fahrenheit, Rankine, Reaumur;

    TType, t_type: "Type T thermocouple (copper-constantan)" =>
        Celsius, Kelvin, Fahrenheit, Rankine, Reaumur;
}

/// A convenience wrapper to allow the user to import all the traits
/// and structures required.
pub mod prelude {
    pub use crate::units::FPExt as _thermocouple_FPExt;
    pub use crate::ThermocoupleCore;
    pub use crate::{
        Celsius, Fahrenheit, Kelvin, Millivolts, Rankine, Reaumur,
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Utility function to compare floating point results
    pub fn compare(a: FP, b: FP, tol: FP) {
        assert!((a - b).abs() < tol);
    }

    macro_rules! test_temperature_default {
        ($($Type:ident, $testname:ident, $result:expr;)*) => {
            $(
                #[test]
                /// Test the correct temperature is calcuated at a
                /// 1.1mV thermoelectric voltage, for a reference
                /// junction at 25ºC
                fn $testname() {
                    let temperature: Celsius =
                        $Type::new().sense_temperature(Millivolts(1.1));
                    println!("{}", temperature.0);
                    compare(temperature.0, $result, 0.05); // ±0.05ºC tolerance
                }
            )*
        }
    }

    #[cfg(any(feature = "k-type"))]
    test_temperature_default! {
        KType, k_type_1_1m_v, 51.870;
    }
    test_temperature_default! {
        BType, b_type_1_1m_v, 470.511;
        EType, e_type_1_1m_v, 42.808;
        JType, j_type_1_1m_v, 46.058;
        NType, n_type_1_1m_v, 64.953;
        RType, r_type_1_1m_v, 173.779;
        SType, s_type_1_1m_v, 176.278;
        TType, t_type_1_1m_v, 51.312;
    }
}
