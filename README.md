[![docs](https://docs.rs/thermocouple/badge.svg)](https://docs.rs/thermocouple)

# [Documentation](https://docs.rs/thermocouple)

# thermocouple

Thermocouple routines.

Provides a `sense_temperature` method to convert thermoelectric
potential to temperature. The following thermocouple types are supported:

* Nickel-alloy thermocouples: Type E, J, K, N, T
* Platinum/rhodium-alloy thermocouples: Type B, R, S

This library includes _newtype_ types, or wrapper types, to help with
using the proper units when doing calculations. I investigated
using some a dimensional analysis crate such as uom or dimensioned
instead of making my own newtype types. However after
experimentation, I found this to be difficult to use and not at
all lightweight.

The underlying storage type is either `f64` (default) or
`f32`. The error tolerance is higher for `f32`, see the tests for
details.

### Usage
Add this to your `Cargo.toml`:

```toml
[dependencies]
thermocouple = "0.1.0"
```

and this to your crate root:

```rust
extern crate thermocouple;
```

The temperature of a thermocouple can be calcuated from a voltage reading.

