#![allow(clippy::suspicious_arithmetic_impl)]

//! Dimensional quantity type with generic underlying storage
//!
use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};
use num_traits::{Num, Zero};
#[cfg(feature = "std")]
pub use num_traits::float::Float;
#[cfg(not(feature = "std"))]
pub use num_traits::float::FloatCore as Float;
#[cfg(feature = "use_serde")]
use serde::{Deserialize, Serialize};

/// Dimensional quantity
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "use_serde", derive(Serialize, Deserialize))]
pub struct QuantityGeneric<
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
    Storage: Num,
>(Storage);

impl<
        const L: i64,
        const M: i64,
        const T: i64,
        const I: i64,
        const TH: i64,
        const N: i64,
        const LUM: i64,
        const A: i64,
        const SA: i64,
        const D_TH: i64,
        const INFO: i64,
        Storage: Num + Zero,
    > Zero for QuantityGeneric<L, M, T, I, TH, N, LUM, D_TH, A, SA, INFO, Storage>
where
    Assert<{ TH != 1 }>: IsTrue,
    QuantityGeneric<L, M, T, I, TH, N, LUM, D_TH, A, SA, INFO, Storage>: Sized,
    Storage: Zero,
{
    fn zero() -> Self {
        QuantityGeneric::<L, M, T, I, TH, N, LUM, D_TH, A, SA, INFO, Storage>(Storage::zero())
    }

    fn is_zero(&self) -> bool {
        self.0.is_zero()
    }

    fn set_zero(&mut self) {
        *self = Zero::zero();
    }
}

impl<
        const L: i64,
        const M: i64,
        const T: i64,
        const I: i64,
        // const TH: i64,
        const N: i64,
        const LUM: i64,
        const A: i64,
        const SA: i64,
        const INFO: i64,
        const D_TH: i64,
        Storage: Num,
    > QuantityGeneric<L, M, T, I, 1, N, LUM, D_TH, A, SA, INFO, Storage>
where
    QuantityGeneric<L, M, T, I, 1, N, LUM, D_TH, A, SA, INFO, Storage>: Sized,
{
    /// Create a new absolute-temperature-like Quantity with zero value.
    pub fn zero() -> QuantityGeneric<L, M, T, I, 1, N, LUM, D_TH, A, SA, INFO, Storage> {
        QuantityGeneric::<L, M, T, I, 1, N, LUM, D_TH, A, SA, INFO, Storage>(Storage::zero())
    }
}

impl<
        const L1: i64,
        const M1: i64,
        const T1: i64,
        const I1: i64,
        const TH1: i64,
        const N1: i64,
        const LUM1: i64,
        const D_TH1: i64,
        const A1: i64,
        const SA1: i64,
        const INFO1: i64,
        const L2: i64,
        const M2: i64,
        const T2: i64,
        const I2: i64,
        const TH2: i64,
        const N2: i64,
        const LUM2: i64,
        const D_TH2: i64,
        const A2: i64,
        const SA2: i64,
        const INFO2: i64,
        Storage: Num,
    > Mul<QuantityGeneric<L2, M2, T2, I2, TH2, N2, LUM2, D_TH2, A2, SA2, INFO2, Storage>>
    for QuantityGeneric<L1, M1, T1, I1, TH1, N1, LUM1, D_TH1, A1, SA1, INFO1, Storage>
where
    QuantityGeneric<
        { L1 + L2 },
        { M1 + M2 },
        { T1 + T2 },
        { I1 + I2 },
        { TH1 + TH2 },
        { N1 + N2 },
        { LUM1 + LUM2 },
        { D_TH1 + D_TH2 },
        { A1 + A2 },
        { SA1 + SA2 },
        { INFO1 + INFO2 },
        Storage,
    >: Sized,
    Storage: Num,
{
    type Output = QuantityGeneric<
        { L1 + L2 },
        { M1 + M2 },
        { T1 + T2 },
        { I1 + I2 },
        { TH1 + TH2 },
        { N1 + N2 },
        { LUM1 + LUM2 },
        { D_TH1 + D_TH2 },
        { A1 + A2 },
        { SA1 + SA2 },
        { INFO1 + INFO2 },
        Storage,
    >;
    /// Multiply two dimensional quantities
    fn mul(
        self,
        rhs: QuantityGeneric<L2, M2, T2, I2, TH2, N2, LUM2, D_TH2, A2, SA2, INFO2, Storage>,
    ) -> QuantityGeneric<
        { L1 + L2 },
        { M1 + M2 },
        { T1 + T2 },
        { I1 + I2 },
        { TH1 + TH2 },
        { N1 + N2 },
        { LUM1 + LUM2 },
        { D_TH1 + D_TH2 },
        { A1 + A2 },
        { SA1 + SA2 },
        { INFO1 + INFO2 },
        Storage,
    > {
        let x = self.0;
        let y = rhs.0;
        QuantityGeneric::<
            { L1 + L2 },
            { M1 + M2 },
            { T1 + T2 },
            { I1 + I2 },
            { TH1 + TH2 },
            { N1 + N2 },
            { LUM1 + LUM2 },
            { D_TH1 + D_TH2 },
            { A1 + A2 },
            { SA1 + SA2 },
            { INFO1 + INFO2 },
            Storage,
        >(x * y)
    }
}

impl<
        const L1: i64,
        const M1: i64,
        const T1: i64,
        const I1: i64,
        const TH1: i64,
        const N1: i64,
        const LUM1: i64,
        const D_TH1: i64,
        const A1: i64,
        const SA1: i64,
        const INFO1: i64,
        const L2: i64,
        const M2: i64,
        const T2: i64,
        const I2: i64,
        const TH2: i64,
        const N2: i64,
        const LUM2: i64,
        const D_TH2: i64,
        const A2: i64,
        const SA2: i64,
        const INFO2: i64,
        Storage: Num,
    > Div<QuantityGeneric<L2, M2, T2, I2, TH2, N2, LUM2, D_TH2, A2, SA2, INFO2, Storage>>
    for QuantityGeneric<L1, M1, T1, I1, TH1, N1, LUM1, D_TH1, A1, SA1, INFO1, Storage>
where
    QuantityGeneric<
        { L1 - L2 },
        { M1 - M2 },
        { T1 - T2 },
        { I1 - I2 },
        { TH1 - TH2 },
        { N1 - N2 },
        { LUM1 - LUM2 },
        { D_TH1 - D_TH2 },
        { A1 - A2 },
        { SA1 - SA2 },
        { INFO1 - INFO2 },
        Storage,
    >: Sized,
{
    type Output = QuantityGeneric<
        { L1 - L2 },
        { M1 - M2 },
        { T1 - T2 },
        { I1 - I2 },
        { TH1 - TH2 },
        { N1 - N2 },
        { LUM1 - LUM2 },
        { D_TH1 - D_TH2 },
        { A1 - A2 },
        { SA1 - SA2 },
        { INFO1 - INFO2 },
        Storage,
    >;

    fn div(
        self,
        rhs: QuantityGeneric<L2, M2, T2, I2, TH2, N2, LUM2, D_TH2, A2, SA2, INFO2, Storage>,
    ) -> QuantityGeneric<
        { L1 - L2 },
        { M1 - M2 },
        { T1 - T2 },
        { I1 - I2 },
        { TH1 - TH2 },
        { N1 - N2 },
        { LUM1 - LUM2 },
        { D_TH1 - D_TH2 },
        { A1 - A2 },
        { SA1 - SA2 },
        { INFO1 - INFO2 },
        Storage,
    > {
        let x = self.0;
        let y = rhs.0;
        QuantityGeneric::<
            { L1 - L2 },
            { M1 - M2 },
            { T1 - T2 },
            { I1 - I2 },
            { TH1 - TH2 },
            { N1 - N2 },
            { LUM1 - LUM2 },
            { D_TH1 - D_TH2 },
            { A1 - A2 },
            { SA1 - SA2 },
            { INFO1 - INFO2 },
            Storage,
        >(x / y)
    }
}

/// Multiply dimensional quantity by dimensionless number
impl<
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
        Storage: Num,
    > Mul<Storage> for QuantityGeneric<L, M, T, I, TH, N, LUM, D_TH, A, SA, INFO, Storage>
{
    type Output = QuantityGeneric<L, M, T, I, TH, N, LUM, D_TH, A, SA, INFO, Storage>;

    fn mul(
        self,
        rhs: Storage,
    ) -> QuantityGeneric<L, M, T, I, TH, N, LUM, D_TH, A, SA, INFO, Storage> {
        let x = self.0;
        QuantityGeneric::<L, M, T, I, TH, N, LUM, D_TH, A, SA, INFO, Storage>(x * rhs)
    }
}

/// Divide dimensional quantity by dimensionless number
impl<
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
        Storage: Num,
    > Div<Storage> for QuantityGeneric<L, M, T, I, TH, N, LUM, D_TH, A, SA, INFO, Storage>
{
    type Output = QuantityGeneric<L, M, T, I, TH, N, LUM, D_TH, A, SA, INFO, Storage>;

    fn div(
        self,
        rhs: Storage,
    ) -> QuantityGeneric<L, M, T, I, TH, N, LUM, D_TH, A, SA, INFO, Storage> {
        let q = self.0;
        QuantityGeneric::<L, M, T, I, TH, N, LUM, D_TH, A, SA, INFO, Storage>(q / rhs)
    }
}

///  Assert for generic const parameters
pub enum Assert<const COND: bool> {}
///  Assert for generic const parameters
pub trait IsTrue {}

impl IsTrue for Assert<true> {}

/// Perform *= operation for a dimensional quantity and a number with Storage type.
/// ```
/// #![feature(generic_const_exprs)]
/// use dimensional_quantity::si::extended::f64::quantities::{Volume};
/// let mut v1 = Volume::new(1.0);
/// v1 *= 10.0;
/// assert_eq!(v1, Volume::new(10.0));
/// ```
impl<
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
        Storage: Num,
    > MulAssign<Storage> for QuantityGeneric<L, M, T, I, TH, N, LUM, D_TH, A, SA, INFO, Storage>
where
    QuantityGeneric<L, M, T, I, TH, N, LUM, D_TH, A, SA, INFO, Storage>: Sized,
    Storage: MulAssign,
{
    fn mul_assign(&mut self, rhs: Storage) {
        self.0 *= rhs;
    }
}

/// Perform *= operation for a dimensional quantity and a Ratio with same Storage type.
/// ```
/// #![feature(generic_const_exprs)]
/// use dimensional_quantity::si::extended::f64::quantities::{Volume,Ratio};
/// let mut v1 = Volume::new(1.0);
/// let r = Ratio::new(10.0);
/// v1 *= r;
/// assert_eq!(v1, Volume::new(10.0));
/// ```
impl<
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
        Storage: Num,
    > MulAssign<QuantityGeneric<0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, Storage>>
    for QuantityGeneric<L, M, T, I, TH, N, LUM, D_TH, A, SA, INFO, Storage>
where
    QuantityGeneric<L, M, T, I, TH, N, LUM, D_TH, A, SA, INFO, Storage>: Sized,
    Storage: MulAssign,
{
    fn mul_assign(&mut self, rhs: QuantityGeneric<0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, Storage>) {
        self.0 *= rhs.0;
    }
}

/// Perform /= operation for a dimensional quantity and a Ratio with same Storage type.
/// ```
/// #![feature(generic_const_exprs)]
/// use dimensional_quantity::si::extended::f64::quantities::{Volume,Ratio};
/// let mut v1 = Volume::new(10.0);
/// let r = Ratio::new(10.0);
/// v1 /= r;
/// assert_eq!(v1, Volume::new(1.0));
/// ```
impl<
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
        Storage: Num,
    > DivAssign<QuantityGeneric<0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, Storage>>
    for QuantityGeneric<L, M, T, I, TH, N, LUM, D_TH, A, SA, INFO, Storage>
where
    QuantityGeneric<L, M, T, I, TH, N, LUM, D_TH, A, SA, INFO, Storage>: Sized,
    Storage: DivAssign,
{
    fn div_assign(&mut self, rhs: QuantityGeneric<0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, Storage>) {
        self.0 /= rhs.0;
    }
}

/// Perform /= operation for a dimensional quantity and a number with Storage type.
/// ```
/// #![feature(generic_const_exprs)]
/// use dimensional_quantity::si::extended::f64::quantities::{Volume};
/// let mut v1 = Volume::new(10.0);
/// v1 /= 10.0;
/// assert_eq!(v1, Volume::new(1.0));
/// ```
impl<
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
        Storage: Num,
    > DivAssign<Storage> for QuantityGeneric<L, M, T, I, TH, N, LUM, D_TH, A, SA, INFO, Storage>
where
    QuantityGeneric<L, M, T, I, TH, N, LUM, D_TH, A, SA, INFO, Storage>: Sized,
    Storage: DivAssign,
{
    fn div_assign(&mut self, rhs: Storage) {
        self.0 /= rhs;
    }
}

/// Addition of dimensional quantities with equal dimension formula, except ones containing absolute temperature
impl<
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
        Storage: Num,
    > Add<QuantityGeneric<L, M, T, I, TH, N, LUM, D_TH, A, SA, INFO, Storage>>
    for QuantityGeneric<L, M, T, I, TH, N, LUM, D_TH, A, SA, INFO, Storage>
where
    QuantityGeneric<L, M, T, I, TH, N, LUM, D_TH, A, SA, INFO, Storage>: Sized,
    Assert<{ TH != 1 }>: IsTrue,
{
    type Output = QuantityGeneric<L, M, T, I, TH, N, LUM, D_TH, A, SA, INFO, Storage>;

    fn add(
        self,
        rhs: QuantityGeneric<L, M, T, I, TH, N, LUM, D_TH, A, SA, INFO, Storage>,
    ) -> QuantityGeneric<L, M, T, I, TH, N, LUM, D_TH, A, SA, INFO, Storage> {
        let x = self.0;
        let y = rhs.0;
        QuantityGeneric::<L, M, T, I, TH, N, LUM, D_TH, A, SA, INFO, Storage>(x + y)
    }
}

/// Perform += operation for dimensional quantities with equal dimension formula, except ones containing absolute temperature
impl<
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
        Storage: Num,
    > AddAssign<QuantityGeneric<L, M, T, I, TH, N, LUM, D_TH, A, SA, INFO, Storage>>
    for QuantityGeneric<L, M, T, I, TH, N, LUM, D_TH, A, SA, INFO, Storage>
where
    QuantityGeneric<L, M, T, I, TH, N, LUM, D_TH, A, SA, INFO, Storage>: Sized,
    Storage: AddAssign,
    Assert<{ TH != 1 }>: IsTrue,
{
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

/// Subtraction of dimensional quantities with equal dimension formula, except ones containing absolute temperature
impl<
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
        Storage: Num,
    > Sub<QuantityGeneric<L, M, T, I, TH, N, LUM, D_TH, A, SA, INFO, Storage>>
    for QuantityGeneric<L, M, T, I, TH, N, LUM, D_TH, A, SA, INFO, Storage>
where
    QuantityGeneric<L, M, T, I, TH, N, LUM, D_TH, A, SA, INFO, Storage>: Sized,
    Assert<{ TH != 1 }>: IsTrue,
{
    type Output = QuantityGeneric<L, M, T, I, TH, N, LUM, D_TH, A, SA, INFO, Storage>;

    fn sub(
        self,
        rhs: QuantityGeneric<L, M, T, I, TH, N, LUM, D_TH, A, SA, INFO, Storage>,
    ) -> QuantityGeneric<L, M, T, I, TH, N, LUM, D_TH, A, SA, INFO, Storage> {
        let x = self.0;
        let y = rhs.0;
        QuantityGeneric::<L, M, T, I, TH, N, LUM, D_TH, A, SA, INFO, Storage>(x - y)
    }
}

/// Perform -= operation for dimensional quantities with equal dimension formula, except ones containing absolute temperature
impl<
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
        Storage: Num,
    > SubAssign<QuantityGeneric<L, M, T, I, TH, N, LUM, D_TH, A, SA, INFO, Storage>>
    for QuantityGeneric<L, M, T, I, TH, N, LUM, D_TH, A, SA, INFO, Storage>
where
    QuantityGeneric<L, M, T, I, TH, N, LUM, D_TH, A, SA, INFO, Storage>: Sized,
    Storage: SubAssign,
    Assert<{ TH != 1 }>: IsTrue,
{
    fn sub_assign(
        &mut self,
        rhs: QuantityGeneric<L, M, T, I, TH, N, LUM, D_TH, A, SA, INFO, Storage>,
    ) {
        self.0 -= rhs.0
    }
}

/// Addition of absolute temperature T with temperature interval ΔT, result is absolute temperature
impl<
        const L: i64,
        const M: i64,
        const T: i64,
        const I: i64,
        // const TH: i64,
        const N: i64,
        const LUM: i64,
        const A: i64,
        const SA: i64,
        const INFO: i64,
        // const D_TH: i64,
        Storage: Num,
    > Add<QuantityGeneric<L, M, T, I, 0, N, LUM, 1, A, SA, INFO, Storage>>
    for QuantityGeneric<L, M, T, I, 1, N, LUM, 0, A, SA, INFO, Storage>
where
    QuantityGeneric<L, M, T, I, 1, N, LUM, 0, A, SA, INFO, Storage>: Sized,
{
    type Output = QuantityGeneric<L, M, T, I, 1, N, LUM, 0, A, SA, INFO, Storage>;

    fn add(
        self,
        rhs: QuantityGeneric<L, M, T, I, 0, N, LUM, 1, A, SA, INFO, Storage>,
    ) -> QuantityGeneric<L, M, T, I, 1, N, LUM, 0, A, SA, INFO, Storage> {
        let x = self.0;
        let y = rhs.0;
        QuantityGeneric::<L, M, T, I, 1, N, LUM, 0, A, SA, INFO, Storage>(x + y)
    }
}

/// Implement += operation for absolute temperature T and temperature interval ΔT
impl<
        const L: i64,
        const M: i64,
        const T: i64,
        const I: i64,
        // const TH: i64,
        const N: i64,
        const LUM: i64,
        const A: i64,
        const SA: i64,
        const INFO: i64,
        // const D_TH: i64,
        Storage: Num,
    > AddAssign<QuantityGeneric<L, M, T, I, 0, N, LUM, 1, A, SA, INFO, Storage>>
    for QuantityGeneric<L, M, T, I, 1, N, LUM, 0, A, SA, INFO, Storage>
where
    QuantityGeneric<L, M, T, I, 1, N, LUM, 0, A, SA, INFO, Storage>: Sized,
    Storage: AddAssign,
{
    fn add_assign(&mut self, rhs: QuantityGeneric<L, M, T, I, 0, N, LUM, 1, A, SA, INFO, Storage>) {
        self.0 += rhs.0
    }
}

/// Subtraction of temperature interval ΔT from absolute temperature T, result is absolute temperature
impl<
        const L: i64,
        const M: i64,
        const T: i64,
        const I: i64,
        // const TH: i64,
        const N: i64,
        const LUM: i64,
        const A: i64,
        const SA: i64,
        const INFO: i64,
        // const D_TH: i64,
        Storage: Num,
    > Sub<QuantityGeneric<L, M, T, I, 0, N, LUM, 1, A, SA, INFO, Storage>>
    for QuantityGeneric<L, M, T, I, 1, N, LUM, 0, A, SA, INFO, Storage>
where
    QuantityGeneric<L, M, T, I, 1, N, LUM, 0, A, SA, INFO, Storage>: Sized,
{
    type Output = QuantityGeneric<L, M, T, I, 1, N, LUM, 0, A, SA, INFO, Storage>;

    fn sub(
        self,
        rhs: QuantityGeneric<L, M, T, I, 0, N, LUM, 1, A, SA, INFO, Storage>,
    ) -> QuantityGeneric<L, M, T, I, 1, N, LUM, 0, A, SA, INFO, Storage> {
        let x = self.0;
        let y = rhs.0;
        QuantityGeneric::<L, M, T, I, 1, N, LUM, 0, A, SA, INFO, Storage>(x - y)
    }
}

/// Implement -= operator for absolute temperature T and temperature interval ΔT.
impl<
        const L: i64,
        const M: i64,
        const T: i64,
        const I: i64,
        // const TH: i64,
        const N: i64,
        const LUM: i64,
        const A: i64,
        const SA: i64,
        const INFO: i64,
        // const D_TH: i64,
        Storage: Num,
    > SubAssign<QuantityGeneric<L, M, T, I, 0, N, LUM, 1, A, SA, INFO, Storage>>
    for QuantityGeneric<L, M, T, I, 1, N, LUM, 0, A, SA, INFO, Storage>
where
    QuantityGeneric<L, M, T, I, 1, N, LUM, 0, A, SA, INFO, Storage>: Sized,
    Storage: SubAssign,
{
    fn sub_assign(&mut self, rhs: QuantityGeneric<L, M, T, I, 0, N, LUM, 1, A, SA, INFO, Storage>) {
        self.0 -= rhs.0
    }
}

/// Subtracting two absolute temperatures T, result is temperature interval ΔT
impl<
        const L: i64,
        const M: i64,
        const T: i64,
        const I: i64,
        // const TH: i64,
        const N: i64,
        const LUM: i64,
        const A: i64,
        const SA: i64,
        const INFO: i64,
        // const D_TH: i64,
        Storage: Num,
    > Sub<QuantityGeneric<L, M, T, I, 1, N, LUM, 0, A, SA, INFO, Storage>>
    for QuantityGeneric<L, M, T, I, 1, N, LUM, 0, A, SA, INFO, Storage>
where
    QuantityGeneric<L, M, T, I, 0, N, LUM, 1, A, SA, INFO, Storage>: Sized,
{
    type Output = QuantityGeneric<L, M, T, I, 0, N, LUM, 1, A, SA, INFO, Storage>;

    fn sub(
        self,
        rhs: QuantityGeneric<L, M, T, I, 1, N, LUM, 0, A, SA, INFO, Storage>,
    ) -> QuantityGeneric<L, M, T, I, 0, N, LUM, 1, A, SA, INFO, Storage> {
        let x = self.0;
        let y = rhs.0;
        QuantityGeneric::<L, M, T, I, 0, N, LUM, 1, A, SA, INFO, Storage>(x - y)
    }
}

impl<
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
        Storage: Num + Copy,
    > QuantityGeneric<L, M, T, I, TH, N, LUM, D_TH, A, SA, INFO, Storage>
where
    QuantityGeneric<L, M, T, I, TH, N, LUM, D_TH, A, SA, INFO, Storage>: Sized,
{
    /// Create new dimensional quantity with generic storage type.
    /// x -- amount in default (SI) unit
    pub const fn new(
        x: Storage,
    ) -> QuantityGeneric<L, M, T, I, TH, N, LUM, D_TH, A, SA, INFO, Storage> {
        QuantityGeneric::<L, M, T, I, TH, N, LUM, D_TH, A, SA, INFO, Storage>(x)
    }
    /// Create a new dimensional quantity with generic storage type from the given value and measurement unit.
    pub fn new_with_unit(
        x: Storage,
        unit: QuantityGeneric<L, M, T, I, TH, N, LUM, D_TH, A, SA, INFO, Storage>,
    ) -> QuantityGeneric<L, M, T, I, TH, N, LUM, D_TH, A, SA, INFO, Storage> {
        QuantityGeneric::<L, M, T, I, TH, N, LUM, D_TH, A, SA, INFO, Storage>(x * unit.0)
    }

    /// Returns dimensional formula array of a quantity
    /// [L, M, T, I, TH, N, LUM, D_TH, A, SA, INFO]
    pub const fn dim(&self) -> [i64; 11] {
        [L, M, T, I, TH, N, LUM, D_TH, A, SA, INFO]
    }
    /// Retrieve the value of the dimensional quantity in the default \[SI\] measurement unit.
    pub const fn get_with_si_unit(&self) -> Storage {
        self.0
    }
    /// Retrieve the value of the dimensional quantity in the given measurement unit.
    pub fn get_with_unit(
        &self,
        unit: QuantityGeneric<L, M, T, I, TH, N, LUM, D_TH, A, SA, INFO, Storage>,
    ) -> Storage {
        self.0 / unit.0
    }

    #[cfg(feature = "std")]
    /// Return units-of-measure string  
    /// ```rust
    /// #![feature(generic_const_exprs)]
    /// use dimensional_quantity::si::extended::f64::quantities::LuminousEfficacy;
    /// let l = LuminousEfficacy::new(1.0);
    /// assert_eq!(l.si_uom_str(), "m⁻²⋅kg⁻¹⋅s³⋅cd⋅sr");
    /// assert_eq!(l.powi::<5>().si_uom_str(),"m⁻¹⁰⋅kg⁻⁵⋅s¹⁵⋅cd⁵⋅sr⁵");
    /// ```
    pub fn si_uom_str(&self) -> String {
        use crate::format::dim_to_string;
        use std::fmt::Write;

        let mut output = String::new();
        if let Some(dim) = dim_to_string(L) {
            let _ = write!(output, "m{}⋅", dim);
        }
        if let Some(dim) = dim_to_string(M) {
            let _ = write!(output, "kg{}⋅", dim);
        }
        if let Some(dim) = dim_to_string(T) {
            let _ = write!(output, "s{}⋅", dim);
        }
        if let Some(dim) = dim_to_string(I) {
            let _ = write!(output, "A{}⋅", dim);
        }
        if let Some(dim) = dim_to_string(TH) {
            let _ = write!(output, "K{}⋅", dim);
        }
        if let Some(dim) = dim_to_string(D_TH) {
            let _ = write!(output, "ΔK{}⋅", dim);
        }
        if let Some(dim) = dim_to_string(N) {
            let _ = write!(output, "mol{}⋅", dim);
        }
        if let Some(dim) = dim_to_string(LUM) {
            let _ = write!(output, "cd{}⋅", dim);
        }
        if let Some(dim) = dim_to_string(A) {
            let _ = write!(output, "rad{}⋅", dim);
        }
        if let Some(dim) = dim_to_string(SA) {
            let _ = write!(output, "sr{}⋅", dim);
        }
        if let Some(dim) = dim_to_string(INFO) {
            let _ = write!(output, "bit{}⋅", dim);
        }

        let mut output = output.chars();
        output.next_back();
        output.collect()
    }
}

impl<Storage: Num + Copy> QuantityGeneric<0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, Storage> {
    /// Convert dimensionless Ratio into underlying storage type
    pub fn into_number(&self) -> Storage {
        self.0
    }
}

impl<
        const L: i64,
        const M: i64,
        const T: i64,
        const I: i64,
        const TH: i64,
        const N: i64,
        const LUM: i64,
        const A: i64,
        const SA: i64,
        const D_TH: i64,
        const INFO: i64,
        Storage: Num,
    > QuantityGeneric<L, M, T, I, TH, N, LUM, D_TH, A, SA, INFO, Storage>
where
    QuantityGeneric<L, M, T, I, TH, N, LUM, D_TH, A, SA, INFO, Storage>: Sized,
    Storage: Float,
{
    /// Raises a dimensional quantity to an integer power.
    pub fn powi<const POWER: i64>(
        &self,
    ) -> QuantityGeneric<
        { L * POWER },
        { M * POWER },
        { T * POWER },
        { I * POWER },
        { TH * POWER },
        { N * POWER },
        { LUM * POWER },
        { D_TH * POWER },
        { A * POWER },
        { SA * POWER },
        { INFO * POWER },
        Storage,
    > {
        QuantityGeneric::<
            { L * POWER },
            { M * POWER },
            { T * POWER },
            { I * POWER },
            { TH * POWER },
            { N * POWER },
            { LUM * POWER },
            { D_TH * POWER },
            { A * POWER },
            { SA * POWER },
            { INFO * POWER },
            Storage,
        >(self.0.powi(POWER as i32))
    }
}

impl<
        const L: i64,
        const M: i64,
        const T: i64,
        const I: i64,
        const TH: i64,
        const N: i64,
        const LUM: i64,
        const A: i64,
        const SA: i64,
        const D_TH: i64,
        const INFO: i64,
        Storage: Num,
    > QuantityGeneric<L, M, T, I, TH, N, LUM, D_TH, A, SA, INFO, Storage>
where
    QuantityGeneric<L, M, T, I, TH, N, LUM, D_TH, A, SA, INFO, Storage>: Sized,
    Storage: Float,
    Assert<{ L % 2 == 0 }>: IsTrue,
    Assert<{ M % 2 == 0 }>: IsTrue,
    Assert<{ T % 2 == 0 }>: IsTrue,
    Assert<{ I % 2 == 0 }>: IsTrue,
    Assert<{ TH % 2 == 0 }>: IsTrue,
    Assert<{ N % 2 == 0 }>: IsTrue,
    Assert<{ LUM % 2 == 0 }>: IsTrue,
    Assert<{ A % 2 == 0 }>: IsTrue,
    Assert<{ SA % 2 == 0 }>: IsTrue,
    Assert<{ D_TH % 2 == 0 }>: IsTrue,
    Assert<{ INFO % 2 == 0 }>: IsTrue,
{
    /// Square root.
    /// ```
    /// #![feature(generic_const_exprs)]
    /// use dimensional_quantity::si::extended::f64::quantities::{Area, Length};
    /// let a:Area = Area::new(100.0);
    /// let l: Length = a.sqrt();
    ///
    /// assert_eq!(l, Length::new(10.0));
    /// ```
    ///
    /// ```compile_fail
    /// #![feature(generic_const_exprs)]
    /// use dimensional_quantity::si::extended::f64::quantities::{Volume, Length};
    /// let v: Volume = Volume::new(100.0);
    /// let x = v.sqrt();
    ///
    ///
    /// ```
    #[cfg(feature = "std")]
    pub fn sqrt(
        &self,
    ) -> QuantityGeneric<
        { L / 2 },
        { M / 2 },
        { T / 2 },
        { I / 2 },
        { TH / 2 },
        { N / 2 },
        { LUM / 2 },
        { D_TH / 2 },
        { A / 2 },
        { SA / 2 },
        { INFO / 2 },
        Storage,
    > {
        QuantityGeneric::<
            { L / 2 },
            { M / 2 },
            { T / 2 },
            { I / 2 },
            { TH / 2 },
            { N / 2 },
            { LUM / 2 },
            { D_TH / 2 },
            { A / 2 },
            { SA / 2 },
            { INFO / 2 },
            Storage,
        >(self.0.sqrt())
    }
}

impl<
        const L: i64,
        const M: i64,
        const T: i64,
        const I: i64,
        const TH: i64,
        const N: i64,
        const LUM: i64,
        const A: i64,
        const SA: i64,
        const D_TH: i64,
        const INFO: i64,
        Storage: Num,
    > QuantityGeneric<L, M, T, I, TH, N, LUM, D_TH, A, SA, INFO, Storage>
where
    QuantityGeneric<L, M, T, I, TH, N, LUM, D_TH, A, SA, INFO, Storage>: Sized,
    Storage: Float,
    Assert<{ L % 3 == 0 }>: IsTrue,
    Assert<{ M % 3 == 0 }>: IsTrue,
    Assert<{ T % 3 == 0 }>: IsTrue,
    Assert<{ I % 3 == 0 }>: IsTrue,
    Assert<{ TH % 3 == 0 }>: IsTrue,
    Assert<{ N % 3 == 0 }>: IsTrue,
    Assert<{ LUM % 3 == 0 }>: IsTrue,
    Assert<{ A % 3 == 0 }>: IsTrue,
    Assert<{ SA % 3 == 0 }>: IsTrue,
    Assert<{ D_TH % 3 == 0 }>: IsTrue,
    Assert<{ INFO % 3 == 0 }>: IsTrue,
{
    /// Cubic root.
    /// ```
    /// #![feature(generic_const_exprs)]
    /// use dimensional_quantity::si::extended::f64::quantities::{Volume, Length};
    /// let v: Volume = Volume::new(1000.0);
    /// let l: Length = v.cbrt();
    ///
    /// assert_eq!(l, Length::new(10.0));
    /// ```
    ///
    /// ```compile_fail
    /// #![feature(generic_const_exprs)]
    /// use dimensional_quantity::si::extended::f64::quantities::{Area, Length};
    /// let a: Area = Area::new(100.0);
    /// let x = a.cbrt();
    ///
    ///
    /// ```
    ///
    #[cfg(feature = "std")]
    pub fn cbrt(
        &self,
    ) -> QuantityGeneric<
        { L / 3 },
        { M / 3 },
        { T / 3 },
        { I / 3 },
        { TH / 3 },
        { N / 3 },
        { LUM / 3 },
        { D_TH / 3 },
        { A / 3 },
        { SA / 3 },
        { INFO / 3 },
        Storage,
    > {
        QuantityGeneric::<
            { L / 3 },
            { M / 3 },
            { T / 3 },
            { I / 3 },
            { TH / 3 },
            { N / 3 },
            { LUM / 3 },
            { D_TH / 3 },
            { A / 3 },
            { SA / 3 },
            { INFO / 3 },
            Storage,
        >(self.0.cbrt())
    }
}

/// Divide f64 by QuantityGeneric with f64 storage type
impl<
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
    > Div<QuantityGeneric<L, M, T, I, TH, N, LUM, D_TH, A, SA, INFO, f64>> for f64
where
    QuantityGeneric<
        { -L },
        { -M },
        { -T },
        { -I },
        { -TH },
        { -N },
        { -LUM },
        { -D_TH },
        { -A },
        { -SA },
        { -INFO },
        f64,
    >: Sized,
{
    type Output = QuantityGeneric<
        { -L },
        { -M },
        { -T },
        { -I },
        { -TH },
        { -N },
        { -LUM },
        { -D_TH },
        { -A },
        { -SA },
        { -INFO },
        f64,
    >;

    fn div(
        self,
        rhs: QuantityGeneric<L, M, T, I, TH, N, LUM, D_TH, A, SA, INFO, f64>,
    ) -> QuantityGeneric<
        { -L },
        { -M },
        { -T },
        { -I },
        { -TH },
        { -N },
        { -LUM },
        { -D_TH },
        { -A },
        { -SA },
        { -INFO },
        f64,
    > {
        let rhs = rhs.0;
        QuantityGeneric::<
            { -L },
            { -M },
            { -T },
            { -I },
            { -TH },
            { -N },
            { -LUM },
            { -D_TH },
            { -A },
            { -SA },
            { -INFO },
            f64,
        >(self / rhs)
    }
}

/// Multiply f64 by QuantityGeneric with f64 storage type
impl<
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
    > Mul<QuantityGeneric<L, M, T, I, TH, N, LUM, D_TH, A, SA, INFO, f64>> for f64
{
    type Output = QuantityGeneric<L, M, T, I, TH, N, LUM, D_TH, A, SA, INFO, f64>;

    fn mul(
        self,
        rhs: QuantityGeneric<L, M, T, I, TH, N, LUM, D_TH, A, SA, INFO, f64>,
    ) -> QuantityGeneric<L, M, T, I, TH, N, LUM, D_TH, A, SA, INFO, f64> {
        let x = rhs.0;
        QuantityGeneric::<L, M, T, I, TH, N, LUM, D_TH, A, SA, INFO, f64>(self * x)
    }
}

/// Divide f32 by QuantityGeneric with f32 storage type
impl<
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
    > Div<QuantityGeneric<L, M, T, I, TH, N, LUM, D_TH, A, SA, INFO, f32>> for f32
where
    QuantityGeneric<
        { -L },
        { -M },
        { -T },
        { -I },
        { -TH },
        { -N },
        { -LUM },
        { -D_TH },
        { -A },
        { -SA },
        { -INFO },
        f32,
    >: Sized,
{
    type Output = QuantityGeneric<
        { -L },
        { -M },
        { -T },
        { -I },
        { -TH },
        { -N },
        { -LUM },
        { -D_TH },
        { -A },
        { -SA },
        { -INFO },
        f32,
    >;

    fn div(
        self,
        rhs: QuantityGeneric<L, M, T, I, TH, N, LUM, D_TH, A, SA, INFO, f32>,
    ) -> QuantityGeneric<
        { -L },
        { -M },
        { -T },
        { -I },
        { -TH },
        { -N },
        { -LUM },
        { -D_TH },
        { -A },
        { -SA },
        { -INFO },
        f32,
    > {
        let rhs = rhs.0;
        QuantityGeneric::<
            { -L },
            { -M },
            { -T },
            { -I },
            { -TH },
            { -N },
            { -LUM },
            { -D_TH },
            { -A },
            { -SA },
            { -INFO },
            f32,
        >(self / rhs)
    }
}

/// Multiply f32 by QuantityGeneric with f32 storage type
impl<
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
    > Mul<QuantityGeneric<L, M, T, I, TH, N, LUM, D_TH, A, SA, INFO, f32>> for f32
{
    type Output = QuantityGeneric<L, M, T, I, TH, N, LUM, D_TH, A, SA, INFO, f32>;

    fn mul(
        self,
        rhs: QuantityGeneric<L, M, T, I, TH, N, LUM, D_TH, A, SA, INFO, f32>,
    ) -> QuantityGeneric<L, M, T, I, TH, N, LUM, D_TH, A, SA, INFO, f32> {
        let x = rhs.0;
        QuantityGeneric::<L, M, T, I, TH, N, LUM, D_TH, A, SA, INFO, f32>(self * x)
    }
}
impl<Storage: Num> From<Storage> for QuantityGeneric<0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, Storage> {
    fn from(value: Storage) -> Self {
        QuantityGeneric::<0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, Storage>(value)
    }
}
impl From<QuantityGeneric<0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, f64>> for f64 {
    fn from(value: QuantityGeneric<0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, f64>) -> Self {
        value.0
    }
}

impl From<QuantityGeneric<0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, f32>> for f32 {
    fn from(value: QuantityGeneric<0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, f32>) -> Self {
        value.0
    }
}
