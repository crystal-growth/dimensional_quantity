## [0.1.11] 15.07.2025
- new quantities: AngularWavenumber, GyromagneticRatio, HallCoefficient, LorenzCoefficient, RichardsonConstant, ThermoelectricCoefficient, VolumetricAngularDensityOfStates
- new QuantityGeneric method toml_template(), returning template for toml file defining an arbitrary derived quantity:
     ```rust
     #![feature(generic_const_exprs)]
     use dimensional_quantity::si::extended::f64::quantities::LuminousEfficacy;
     let l2 = LuminousEfficacy::new(1.0).powi::<2>();
     assert_eq!(l2.toml_template(),
     r#"
     name = "Some quantity"
     symbol = "SomeQuantiy"
     snake_case_name = "some_quantity"
     short_dim_formula = "L<sup>-4</sup>⋅M<sup>-2</sup>⋅T<sup>6</sup>⋅J<sup>2</sup>⋅Ω<sup>2</sup>"
     long_dim_formula = "Length<sup>-4</sup>⋅Mass<sup>-2</sup>⋅Time<sup>6</sup>⋅Luminous intensity<sup>2</sup>⋅Solid angle<sup>2</sup>"
     units_formula = "m<sup>-4</sup>⋅kg<sup>-2</sup>⋅s<sup>6</sup>⋅cd<sup>2</sup>⋅sr<sup>2</sup>"
    
     [dimensions]
     length = -4
     mass = -2
     time = 6
     electric_current = 0
     thermodynamic_temperature = 0
     temperature_interval = 0
     amount_of_substance = 0
     luminous_intensity = 2
     angle = 0
     solid_angle = 2
     information = 0
    
     [units.DEFAULT_UNIT]
     multiplier = "1.0"
     symbol = "m⁻⁴⋅kg⁻²⋅s⁶⋅cd²⋅sr²"
     singular = "default_unit"
     plural = "default_units""#
     );
     ```
- new QuantityGeneric methods short_dim_formula(), long_dim_formula(), units_formula(), toml_dim_formula()
## [0.1.10] 22.01.2025
- fixed arithmetic error in to_degree_fahrenheit() method of ThermodynamicTemperature quantity
- Réaumur temperature scale for ThermodynamicTemperature and TemperatureInterval: DEGREE_REAUMUR unit of measure,
  with corresponding methods for ThermodynamicTemperature: to_degree_reaumur() and from_degree_reaumur()
- tests covering conversion between Kelvin, Celsius, Fahrenheit, Rankine, and Réaumur temperature scales
## [0.1.9] 21.01.2025
- new quantity: MagneticReluctance [L⁻²⋅M⁻¹⋅T²⋅I²]
- new QuantityGeneric method: si_uom_string, that returns String with corresponding unit of measure formula:
    ```rust
    #![feature(generic_const_exprs)]
    use dimensional_quantity::si::extended::f64::quantities::LuminousEfficacy;
    let l = LuminousEfficacy::new(1.0);
    assert_eq!(l.si_uom_str(), "m⁻²⋅kg⁻¹⋅s³⋅cd⋅sr");
    assert_eq!(l.powi::<5>().si_uom_str(),"m⁻¹⁰⋅kg⁻⁵⋅s¹⁵⋅cd⁵⋅sr⁵");
    ```
## [0.1.8] 10.01.2025
- new quantities: ThermalResistance and ThermalConductance
## [0.1.7] 21.12.2024
- ```std``` feature enable by default, disable ```default-features``` to build ```no_std``` 
## [0.1.6] 04.10.2024
- new quantities: SpecificEntropy and ThermalInsulance
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
- New QuantityGeneric methods: zero() creates generic quantity with zero value, into_number() converts Ratio into underlying storage
- New MolarFlow quantity

## [0.0.2] 14.04.2023
- Removed std dependencies. Crate is now no_std


## [0.0.1] 03.01.2023
- New SI metric prefixes: QUETTA (10^30), RONNA (10^27), RONTO (19^-27), QUECTO (10^-30)

## [0.0.1-alpha.6] 01.12.2022
- Implemented += and -= operators for SI-extended and SI-isq systems.

## [0.0.1-alpha.5] 01.12.2022
- Square root (sqrt) and cubic root (cbrt) implemented for SI-extended and SI-isq systems.
