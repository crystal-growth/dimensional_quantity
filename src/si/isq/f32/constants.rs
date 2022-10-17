//! Dimensional physical constants with f32 precision, CODATA 2018 values.
#![allow(clippy::excessive_precision)]
#![allow(clippy::inconsistent_digit_grouping)]

use crate::si::isq::f32::quantities::*;
include!(concat!(env!("OUT_DIR"), "/codata2018_f32.rs"));
