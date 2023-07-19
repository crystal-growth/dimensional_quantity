//! Definition of Quantity type with f64 as underlying storage
use crate::si::extended::operations_generic::QuantityGeneric;
/// SI [extended system of quantities] dimensional quantity type with f64 as underlying storage
pub type Quantity<
    const L: i64,
    const M: i64,
    const T: i64,
    const I: i64,
    const TH: i64,
    const N: i64,
    const LUM: i64,
    const D_TH: i64,
    const A: i64,
    const SA: i64,
    const INFO: i64,
> = QuantityGeneric<L, M, T, I, TH, N, LUM, D_TH, A, SA, INFO, f64>;
