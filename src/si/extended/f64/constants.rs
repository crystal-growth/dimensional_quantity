//! Dimensional physical constants with f64 precision, CODATA 2018 values.
use crate::si::extended::f64::quantities::*;
include!(concat!(env!("OUT_DIR"), "/codata2018_f64.rs"));
