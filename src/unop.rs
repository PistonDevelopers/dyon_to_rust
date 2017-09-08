//! Unary operators.

use Secret;

pub trait Neg {
    type Output;

    fn neg(&self) -> Self::Output;
}

impl<'a, T: Neg> Neg for &'a T {
    type Output = T::Output;

    fn neg(&self) -> T::Output {(*self).neg()}
}

impl Neg for f64 {
    type Output = f64;

    fn neg(&self) -> f64 {
        -self
    }
}

pub fn neg<T: Neg>(a: &T) -> T::Output {
    a.neg()
}

pub trait Not {
    type Output;

    fn not(&self) -> Self::Output;
}

impl<'a, T: Not> Not for &'a T {
    type Output = <T as Not>::Output;

    fn not(&self) -> Self::Output {
        (*self).not()
    }
}

impl Not for bool {
    type Output = bool;

    fn not(&self) -> bool {
        !self
    }
}

impl<A: Clone> Not for Secret<bool, A> {
    type Output = Secret<bool, A>;

    fn not(&self) -> Secret<bool, A> {
        Secret {val: !self.val, secret: self.secret.clone()}
    }
}

pub fn not<T: Not>(a: &T) -> T::Output {
    a.not()
}
