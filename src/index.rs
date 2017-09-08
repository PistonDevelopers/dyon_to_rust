//! Indexing helper functions.

use dyon::Variable;

pub trait Index {
    type Output;

    fn index(self) -> Self::Output;
}

impl Index for f64 {
    type Output = usize;

    fn index(self) -> usize {self as usize}
}

impl<'a> Index for &'a str {
    type Output = &'a str;

    fn index(self) -> &'a str {self}
}

/// Converts a type for index lookup.
pub fn ind<T: Index>(a: T) -> T::Output {
    a.index()
}

pub trait Vec4LookUp<T> {
    fn vec4_look_up(&self, ind: T) -> f32;
}

impl Vec4LookUp<usize> for [f32; 4] {
    fn vec4_look_up(&self, ind: usize) -> f32 {
        self[ind]
    }
}

impl Vec4LookUp<usize> for Variable {
    fn vec4_look_up(&self, ind: usize) -> f32 {
        if let Variable::Vec4(a) = *self {
            a[ind]
        } else {
            panic!("Trying to look up in `vec4` but the variable type did not match.");
        }
    }
}

/// Look up in a 4D vector.
pub fn vec4_look_up<A: Vec4LookUp<T>, T>(a: &A, ind: T) -> f32 {
    a.vec4_look_up(ind)
}
