use std::sync::Arc;

use dyon::Variable;

pub trait ToVariable {
    fn to_variable(&self) -> Variable;
}

impl ToVariable for bool {
    fn to_variable(&self) -> Variable {
        Variable::bool(*self)
    }
}

impl ToVariable for f64 {
    fn to_variable(&self) -> Variable {
        Variable::f64(*self)
    }
}

impl ToVariable for str {
    fn to_variable(&self) -> Variable {
        Variable::Text(Arc::new(self.into()))
    }
}

impl ToVariable for [f32; 4] {
    fn to_variable(&self) -> Variable {
        Variable::Vec4(*self)
    }
}

impl<'a, T: ToVariable + ?Sized> ToVariable for &'a T {
    fn to_variable(&self) -> Variable {
        (*self).to_variable()
    }
}

impl<T: ToVariable> ToVariable for Vec<T> {
    fn to_variable(&self) -> Variable {
        Variable::Array(Arc::new(self.iter().map(|v| v.to_variable()).collect()))
    }
}

impl ToVariable for Variable {
    fn to_variable(&self) -> Variable {
        self.clone()
    }
}

/// Helps converting a type into a dynamic Dyon variable.
pub fn to_variable<T: ToVariable>(a: &T) -> Variable {
    a.to_variable()
}
