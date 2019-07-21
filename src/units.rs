//! Units for thermocouple operations.

use crate::FP;
use core::fmt;
use core::ops::{Add, Sub};

macro_rules! unit {
    ($($TYPE:ident, $type:ident => $format:expr, $doc:expr;)*) => {
        $(
            #[doc=$doc]
            #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
            #[derive(PartialEq, PartialOrd, Clone, Copy, Debug)]
            pub struct $TYPE(pub FP);

            impl Add for $TYPE {
                type Output = $TYPE;

                fn add(self, rhs: $TYPE) -> $TYPE {
                    $TYPE(self.0 + rhs.0)
                }
            }
            impl Sub for $TYPE {
                type Output = $TYPE;

                fn sub(self, rhs: $TYPE) -> $TYPE {
                    $TYPE(self.0 - rhs.0)
                }
            }


            impl fmt::Display for $TYPE {
                fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                    write!(f, $format, self.0)
                }
            }
        )*

        /// Extension trait that adds convenience methods to the `FP` type
        pub trait FPExt {
            $(
                #[doc=$doc]
                fn $type(self) -> $TYPE;
            )*
        }

        impl FPExt for FP {
            $(
                fn $type(self) -> $TYPE {
                    $TYPE(self)
                }
            )*
        }
    }
}

unit! {
    Millivolts, millivolts =>
        "{:.3}mV", "Unit of electric potential, 1/1000 of the SI
 Base Unit Volt";
    Kelvin, kelvin =>
        "{:.2}K", "Unit of thermodynamic temperature, defined as
 the fraction of 1/273.16 of the thermodynamic temperature of the
 triple point of water";
    Celsius, celsius =>
        "{:.1}ºC", "Unit of thermodynamic temperature";
    Fahrenheit, fahrenheit =>
        "{:.1}ºF", "Unit of thermodynamic temperature";
    Rankine, rankine =>
        "{:.1}ºRa", "Unit of thermodynamic temperature";
    Reaumur, reaumur =>
        "{:.1}ºRé", "Unit of thermodynamic temperature";
}

// Unit conversions
impl From<Kelvin> for Celsius {
    fn from(t: Kelvin) -> Celsius {
        Celsius(t.0 - 273.15)
    }
}
impl From<Fahrenheit> for Celsius {
    fn from(t: Fahrenheit) -> Celsius {
        Celsius((t.0 - 32.0) / 1.8)
    }
}
impl From<Rankine> for Celsius {
    fn from(t: Rankine) -> Celsius {
        Celsius((t.0 - 491.67) / 1.8)
    }
}
impl From<Reaumur> for Celsius {
    fn from(t: Reaumur) -> Celsius {
        Celsius(t.0 * 1.25)
    }
}
impl From<Celsius> for Kelvin {
    fn from(t: Celsius) -> Kelvin {
        Kelvin(t.0 + 273.15)
    }
}
impl From<Celsius> for Fahrenheit {
    fn from(t: Celsius) -> Fahrenheit {
        Fahrenheit(t.0 * 1.8 + 32.0)
    }
}
impl From<Celsius> for Rankine {
    fn from(t: Celsius) -> Rankine {
        Rankine(t.0 * 1.8 + 491.67)
    }
}
impl From<Celsius> for Reaumur {
    fn from(t: Celsius) -> Reaumur {
        Reaumur(t.0 * 0.8)
    }
}
