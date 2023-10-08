use crate::FP;
use core::usize;
/// Fast evaluation of polynom using Horner method (https://en.wikipedia.org/wiki/Horner%27s_method)
/// This seems to be the fastest sequential way. (Estrin's scheme is maybe to complicated for this crate)
/// is not faster coefs.iter().rev().fold(0., |ret: f64, c| x * ret + c)
/// (I just wanted to flex my iterators)
/// Speed-wise, this :
/// ```
/// let mut ret = coefs[N_COEF - 1];
/// for i in (0..(N_COEF - 1)).rev() {
///     ret = x * ret + coefs[i];
/// }
/// ret
/// ```
///  is the same as this:
#[inline(always)]
pub fn polyval<const N_COEF: usize>(coefs: [FP; N_COEF], x: FP) -> FP {
    coefs
        .iter()
        .rev()
        .skip(1)
        .fold(coefs[N_COEF - 1], |ret: f64, c| x * ret + c)
}
