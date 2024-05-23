//! Dimensional physical constants with f64 precision, CODATA 2022 values.
#![allow(clippy::excessive_precision)]
#![allow(clippy::inconsistent_digit_grouping)]

use crate::si::extended::f32::quantities::*;
include!(concat!(env!("OUT_DIR"), "/codata2022_f32.rs"));
