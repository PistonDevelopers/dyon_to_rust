//! Assignment helper methods.

pub trait SetAssign<T> {
    fn set_assign(&mut self, val: &T);
}

impl<'a, T, U> SetAssign<U> for &'a mut T
    where T: SetAssign<U>
{
    fn set_assign(&mut self, val: &U) {
        (*self).set_assign(val)
    }
}

impl SetAssign<f64> for f64 {
    fn set_assign(&mut self, val: &f64) {
        *self = *val
    }
}

/// Helps converting types when assigning a value.
pub fn set_assign<T: SetAssign<U>, U>(a: &mut T, b: &U) {
    a.set_assign(b)
}
