use Secret;

pub trait Cond {
    fn cond(&self) -> bool;
}

impl Cond for bool {
    fn cond(&self) -> bool {*self}
}

impl<A> Cond for Secret<bool, A> {
    fn cond(&self) -> bool {self.val}
}

/// Helps converting a condition type into `bool`.
pub fn cond_eval<T: Cond>(a: &T) -> bool {
    a.cond()
}
