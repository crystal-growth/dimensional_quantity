# dimensional_quantity

Dimensional quantity: checking dimensions of physical quantities in compile time using generic const expressions
* [129 predefined dimensional quantities for International System of Units (SI)](si::extended::f64).
* [Dimensional units of measure](si::extended::f64::units_of_measure).
* [Dimensional physical constants (CODATA 2022 values)](si::extended::f64::constants).
* [Non-dimensional SI prefixes: metric (kilo, micro, giga, etc.) and binary (Kibi, Mebi, ...)](prefix).

## Usage

* This crate is **experimental** and uses **unstable** [`generic_const_exprs`](https://github.com/rust-lang/rust/issues/76560)
  feature and can only be compiled with **nightly** toolchain.
* If you need a **stable** solution, please check excellent [dimensioned](https://github.com/paholg/dimensioned) and [uom](https://github.com/iliekturtles/uom) crates.
* The only implemented underlying storage types are f64 and f32.

To use this crate, first add this to your `Cargo.toml`:
```toml
[dependencies]
dimensional_quantity = "0.1"
```
 then enable *`generic_const_exprs`* feature in your crate:
```rust
 #![feature(generic_const_exprs)]
```

and build it with nightly toolchain using ```cargo +nightly```:

```bash
cargo +nightly build
```

or add `rust-toolchain.toml` file with the following content to your project
```toml
[toolchain]
channel = "nightly"
```
## Features
### Serialization/deserialization with Serde
```toml
[dependencies]
dimensional_quantity = {version = "0.1", features = ["use_serde"]}
```

### Decimal storage type
```toml
[dependencies]
dimensional_quantity = {version = "0.1", features = ["decimal"]}
```



## Examples

### Creating dimensional quantities from f64

```rust

#![feature(generic_const_exprs)]

use dimensional_quantity::si::extended::f64::quantities::{Velocity};
use dimensional_quantity::si::extended::f64::units_of_measure::velocity::{MILLIMETER_PER_SECOND};

// This will create velocity of 10 m/s, default units are SI units
let v1: Velocity = Velocity::new(10.0);
// This method is constant and works also at compile time:
const SPEED_OF_SOUND: Velocity = Velocity::new(343.0);

// Various units of length are available at
// dimensional_quantity::si::extended::f64::units_of_measure::*
// Units are just constant dimensional quantities
// For example, MILLIMETER unit is defined as
// pub const MILLIMETER: Length = Length::new(1.0E-3);
// One way is using new_with_unit method:
let v2: Velocity = Velocity::new_with_unit(10_000.0, MILLIMETER_PER_SECOND);
// Another is just multiplying f64 with unit:
let v3: Velocity = 10_000.0 * MILLIMETER_PER_SECOND;

// Any of the above methods lead to the same result:
assert_eq!(v1, v2);
assert_eq!(v1, v3);
```

### Converting dimensional quantities back to f64

```rust
use dimensional_quantity::si::extended::f64::quantities::{Velocity};
use dimensional_quantity::si::extended::f64::units_of_measure::velocity::{MILLIMETER_PER_SECOND};

// Getting f64 value of dimensional quantity in SI units:
// Velocity of 10 m/s
let v1: Velocity = Velocity::new(10.0);
// Velocity of 343 m/s
const SPEED_OF_SOUND: Velocity = Velocity::new(343.0);

let v1_value_is_si_units = v1.get_with_si_unit();
// also possible at compile time:
const SPEED_OF_SOUND_IN_SI_UNITS:f64 = SPEED_OF_SOUND.get_with_si_unit();

assert_eq!(SPEED_OF_SOUND_IN_SI_UNITS, 343.0);
assert_eq!(v1_value_is_si_units, 10.0);

// Getting f64 value of dimensional quantity in arbitrary units is possible:
let v1_value_is_mm_per_second = v1.get_with_unit(MILLIMETER_PER_SECOND);
assert_eq!(v1_value_is_mm_per_second, 10_000.0);
```

### Mathematical operations with dimensional quantities

```rust

#![feature(generic_const_exprs)]

use dimensional_quantity::si::extended::f64::quantities::{Area, Energy, Length, Mass, ReciprocalLength, Velocity};
use dimensional_quantity::si::extended::f64::units_of_measure::length::{METER, MICROMETER};
use dimensional_quantity::si::extended::f64::units_of_measure::reciprocal_length::{RECIPROCAL_CENTIMETER};
use core::f64::consts::PI;
let width: Length = 5.0 * METER;
let height: Length = 8.0 * METER;
// Quantities can be multiplied or divided by f64 floating numbers,
// resulting in quantities of same dimension
let double_height: Length = height * 2.0;
let half_width = width / 2.0;

assert_eq!(double_height, Length::new(16.0));
assert_eq!(half_width, Length::new(2.5));

// Dividing f64 by dimensional quantity returns reciprocal quantity:
let red_light_wavelength: Length = 0.65 * MICROMETER;
let red_light_k: ReciprocalLength = 2.0 * PI / red_light_wavelength;
let red_light_k_in_reciprocal_cm = red_light_k.get_with_unit(RECIPROCAL_CENTIMETER);

assert_eq!(red_light_k_in_reciprocal_cm, 966_64.389_341_224_39);
// Most quantities of same dimension (except ones containing ThermodynamicTemperatures,
// that will be discussed below) can be added or subtracted:

let perimeter: Length = 2.0 * (width + height);
// AddAssign and SubAssign operators are supported
let mut width_minus_height = Length::new(0.0);
width_minus_height += width;
width_minus_height -= height;

assert_eq!(perimeter, Length::new(26.0));

// Dimensional quantities can be multiplied and divided:
let area_1: Area = width * height;
let area_2: Area = half_width * double_height;
assert_eq!(area_1, area_2);
let height_1 :Length = area_1 / width;
assert_eq!(height_1, height);

// Dimensional quantities can also be raised to an integer power during compile time
let v: Velocity = Velocity::new(10.0);
let m: Mass = Mass::new(5.0); // 5 kg
let e: Energy = m * v.powi::<2>() / 2.0;
assert_eq!(e, Energy::new(250.0));
```

Attempting to add, subtract, or assign quantities with mismatching dimensions will results in compile-time error:

```compile_fail
#![feature(generic_const_exprs)]
use dimensional_quantity::si::extended::{Area, Length};

let length: Length = Length::new(10.0);
let area: Area = length.powi<2>();
// Type mismatch: can not add Length and Area:
let fail_1 = length + area;
// Type mismatch: can not subtract Area from Length:
let fail_2 = length - area;
// Type mismatch: can not assign Area to Length:
let fail_3: Length = area;
```

### Creating new quantities and tests

If some quantity or unit are not implemented in [predefined dimensional quantities](si::extended::f64) and [predefined units of measure](si::extended::f64::units_of_measure),
then a new quantity and corresponding units of measure can be defined as follows:

```rust
#![feature(generic_const_exprs)]
use dimensional_quantity::si::extended::f64::quantities::{Information, Volume};
use dimensional_quantity::si::extended::f64::Quantity;
// New quantity: amount of information per unit of volume, standard unit of measure: Bit per cubic meter, B ⋅ m<sup>-3</sup>:
pub type VolumetricInformationDensity = Quantity<
                                                -3, // Length
                                                 0, // Mass
                                                 0, // Time
                                                 0, // ElectricCurrent
                                                 0, // ThermodynamicTemperature
                                                 0, // AmountOfSubstance
                                                 0, // LuminousIntensity
                                                 0, // TemperatureInterval
                                                 0, // Angle
                                                 0, // SolidAngle
                                                 1, // Information
                                                  >;

pub const GIGABIT_PER_CUBIC_METER: VolumetricInformationDensity = VolumetricInformationDensity::new(1.0_E9);
let information_density_1: VolumetricInformationDensity  = 5.0 * GIGABIT_PER_CUBIC_METER;
let information_density_2: VolumetricInformationDensity = Information::new(5.0_E9) / Volume::new(1.0);
assert_eq!(information_density_1, information_density_2);
```


## Definition of quantities and units of measure

Quantities and corresponding units are defined at `src/si/quantities_definition/*toml` files.

For example, definition of Area quantity is:
```toml
name = "Area"
symbol = "Area"
snake_case_name = "area"
short_dim_formula = "L<sup>2</sup>"
long_dim_formula = "Length<sup>2</sup>"
units_formula = "m<sup>2</sup>"
[dimensions]
length = 2
mass = 0
time = 0
electric_current = 0
thermodynamic_temperature = 0
temperature_interval = 0
amount_of_substance = 0
luminous_intensity = 0
angle = 0
solid_angle = 0
information = 0

[units.SQUARE_METER]
multiplier = "1.0E0"
symbol = "m²"
singular = "square meter"
plural = "square meters"

[units.SQUARE_CENTIMETER]
multiplier = "1.0_E-4"
symbol = "cm²"
singular = "square centimeter"
plural = "square centimeters"
```


## Licence
Licensed under either of Apache License, Version 2.0 or MIT license (LICENSE-MIT or <https://opensource.org/licenses/MIT>) at your option.

License: MIT OR Apache-2.0
