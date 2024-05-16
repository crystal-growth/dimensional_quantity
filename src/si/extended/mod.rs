//! International System of Units (SI) with extended International System of Quantities (ISQ) implementation.
//!
//! Standard quantities from ISQ:
//!   -   Length (base unit meter, m)
//!   -   Mass (base unit kilogram, kg)
//!   -   Time (base unit second, s)
//!   -   Electric current (base unit ampere, A)
//!   -   Thermodynamic temperature (base unit kelvin, K)
//!   -   Amount of substance (base unit mole, mol)
//!   -   Luminous intensity (base unit candela, cd)
//!
//! Additional quantities:
//!   -   Angle (base unit radian, rad)
//!   -   SolidAngle (base unit steradian, sr)
//!   -   Temperature interval (base unit Δkelvin, ΔK )
//!   -   Information (base unit byte, B)

pub mod f32;
pub mod f64;
#[cfg(feature = "decimal")]
pub mod decimal;
pub mod operations_generic;
