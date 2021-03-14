[![docs](https://docs.rs/thermocouple/badge.svg)](https://docs.rs/thermocouple)

# [Documentation](https://docs.rs/thermocouple)

# thermocouple

Thermocouple routines.

Provides a `sense_temperature` method to convert thermoelectric
potential to temperature. The following thermocouple types are supported:

* Nickel-alloy thermocouples: Type E, J, K, N, T
* Platinum/rhodium-alloy thermocouples: Type B, R, S

This library includes _newtype_ types, or wrapper types, to help with using
the proper units when doing calculations. I investigated using a dimensional
analysis crate such as uom or dimensioned instead of making my own newtype
types. However after experimentation, I found this to be difficult to use
and not at all lightweight.

The underlying storage type is either `f64` (default) or `f32`. For the
`f64` storage type, the results from this crate match the [NIST ITS-90
Thermocouple Database][ITS-90] exactly.

### Usage
Add this to your `Cargo.toml`:

```toml
[dependencies]
thermocouple = "0.1.3"
```

and this to your crate root:

```rust
extern crate thermocouple;
```

[ITS-90]: https://srdata.nist.gov/its90/main/its90_main_page.html
