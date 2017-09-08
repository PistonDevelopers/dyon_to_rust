use std::ops::{BitOrAssign, BitAndAssign};

/// Wraps a value with a secret.
pub struct Secret<T, A> {
    pub val: T,
    pub secret: Vec<A>,
}

impl<A> Secret<bool, A> {
    pub fn new_bool(val: bool) -> Secret<bool, A> {
        Secret {val: val, secret: vec![]}
    }
}

impl<A> Secret<f64, A> {
    pub fn new_f64(val: f64) -> Secret<f64, A> {
        Secret {val: val, secret: vec![]}
    }
}

impl<A> From<bool> for Secret<bool, A> {
    fn from(val: bool) -> Secret<bool, A> {Secret::new_bool(val)}
}

impl<A> From<f64> for Secret<f64, A> {
    fn from(val: f64) -> Secret<f64, A> {Secret::new_f64(val)}
}

impl<A> BitOrAssign<bool> for Secret<bool, A> {
    fn bitor_assign(&mut self, rhs: bool) {
        self.val |= rhs;
    }
}

impl<A> BitAndAssign<bool> for Secret<bool, A> {
    fn bitand_assign(&mut self, rhs: bool) {
        self.val &= rhs;
    }
}

/// Implemented by types that might have secrets.
pub trait SecretValue {
    type ValueType;

    /// Returns the value of the type (not the secret).
    fn value(&self) -> Self::ValueType;
}

impl<T: Copy, A> SecretValue for Secret<T, A> {
    type ValueType = T;

    fn value(&self) -> T {self.val}
}

impl SecretValue for f64 {
    type ValueType = f64;

    fn value(&self) -> f64 {*self}
}

impl SecretValue for bool {
    type ValueType = bool;

    fn value(&self) -> bool {*self}
}
