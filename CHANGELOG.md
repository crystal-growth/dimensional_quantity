## [0.0.5] 05.05.2024
New ThermalPressureCoefficient quantity
Update `toml` dependency
## [0.0.4] 21.07.2023
Breaking change in ISQ-extended system: SolidAngle is now a basic unit of measure
num_traits::Zero trait implemented for QuantityGeneric

## [0.0.3] 14.07.2023
Serde support with "use_serde" feature
New QuantityGeneric functions: zero() creates generic quantity with zero value, into_number() converts Ratio into underlying storage
New MolarFlow quantity 

## [0.0.2] 14.04.2023
Removed std dependencies. Crate is now no_std


## [0.0.1] 03.01.2023
New SI metric prefixes: QUETTA (10^30), RONNA (10^27), RONTO (19^-27), QUECTO (10^-30)

## [0.0.1-alpha.6] 01.12.2022
Implemented += and -= operators for SI-extended and SI-isq systems.

## [0.0.1-alpha.5] 01.12.2022
Square root (sqrt) and cubic root (cbrt) implemented for SI-extended and SI-isq systems.