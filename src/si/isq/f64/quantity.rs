//! Definition of Quantity type with f64 as underlying storage
use crate::si::isq::operations_generic::QuantityGeneric;

/// SI ISQ dimensional quantity type with f64 as underlying storage
pub type Quantity<
    const L: i64,
    const M: i64,
    const T: i64,
    const I: i64,
    const TH: i64,
    const N: i64,
    const LUM: i64,
> = QuantityGeneric<L, M, T, I, TH, N, LUM, f64>;
