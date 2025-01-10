#![feature(generic_const_exprs)]
#![allow(incomplete_features)]
#![deny(missing_docs)]
#![allow(clippy::inconsistent_digit_grouping)]
#![deny(unsafe_code)]
#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::excessive_precision)]


#![doc = include_str!("../README.md")]

pub mod prefix;
pub mod si;
mod test;
