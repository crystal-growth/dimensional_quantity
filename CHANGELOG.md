## [0.1.5] 01.10.2024
- rust_decimal dependency version upgrade to 1.36
- new quantities: Compressibility and ReciprocalTemperature 
## [0.1.3] 23.05.2024
- Updated CODATA 2022 Physical Constants
## [0.1.2] 16.05.2024
- Fixed broken ```use_serde``` feature
- Initial support for Decimal storage type 
## [0.1.1] 13.05.2024
- QuantityGeneric implements DivAssign and MulAssign traits for Storage:
```rust
use dimensional_quantity::si::isq::f64::quantities::{Volume};
let mut v1 = Volume::new(1.0);
v1 *= 10.0;
assert_eq!(v1, Volume::new(10.0));
```
and Ratio:

```rust
use dimensional_quantity::si::isq::f64::quantities::{Volume,Ratio};
let mut v1 = Volume::new(1.0);
let r = Ratio::new(10.0);
v1 *= r;
assert_eq!(v1, Volume::new(10.0));
```
## [0.1.0] 12.05.2024
- Div<Quantity> trait implemented for f32 and f64
- From<Num> trait implemented for Ratio (dimensionless quantity)
- From<Ratio> trait implemented for f32 and f64
- PartialOrd and Ord traits derived for Quantity

## [0.0.5] 05.05.2024
- New ThermalPressureCoefficient quantity
- Update `toml` dependency
## [0.0.4] 21.07.2023
- Breaking change in ISQ-extended system: SolidAngle is now a basic unit of measure
- num_traits::Zero trait implemented for QuantityGeneric

## [0.0.3] 14.07.2023
- Serde support with "use_serde" feature
- New QuantityGeneric functions: zero() creates generic quantity with zero value, into_number() converts Ratio into underlying storage
- New MolarFlow quantity

## [0.0.2] 14.04.2023
- Removed std dependencies. Crate is now no_std


## [0.0.1] 03.01.2023
- New SI metric prefixes: QUETTA (10^30), RONNA (10^27), RONTO (19^-27), QUECTO (10^-30)

## [0.0.1-alpha.6] 01.12.2022
- Implemented += and -= operators for SI-extended and SI-isq systems.

## [0.0.1-alpha.5] 01.12.2022
- Square root (sqrt) and cubic root (cbrt) implemented for SI-extended and SI-isq systems.
