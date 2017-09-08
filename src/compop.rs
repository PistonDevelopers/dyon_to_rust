//! Compare operators.

pub trait Less<Rhs = Self> {
    type Output;

    fn less(&self, other: &Rhs) -> Self::Output;
}

impl Less for f64 {
    type Output = bool;

    fn less(&self, other: &f64) -> bool {
        self < other
    }
}

pub fn less<T: Less<U>, U>(a: &T, b: &U) -> T::Output {
    a.less(b)
}

pub trait LessOrEqual<Rhs = Self> {
    type Output;

    fn less_or_equal(&self, other: &Rhs) -> Self::Output;
}

impl LessOrEqual for f64 {
    type Output = bool;

    fn less_or_equal(&self, other: &f64) -> bool {
        self <= other
    }
}

pub fn less_or_equal<T: LessOrEqual<U>, U>(a: &T, b: &U) -> T::Output {
    a.less_or_equal(b)
}

pub trait Greater<Rhs = Self> {
    type Output;

    fn greater(&self, other: &Rhs) -> Self::Output;
}

impl Greater for f64 {
    type Output = bool;

    fn greater(&self, other: &f64) -> bool {
        self > other
    }
}

pub fn greater<T: Greater<U>, U>(a: &T, b: &U) -> T::Output {
    a.greater(b)
}

pub trait GreaterOrEqual<Rhs = Self> {
    type Output;

    fn greater_or_equal(&self, other: &Rhs) -> Self::Output;
}

impl GreaterOrEqual for f64 {
    type Output = bool;

    fn greater_or_equal(&self, other: &f64) -> bool {
        self >= other
    }
}

pub fn greater_or_equal<T: GreaterOrEqual<U>, U>(a: &T, b: &U) -> T::Output {
    a.greater_or_equal(b)
}

pub trait Equal<Rhs = Self> {
    type Output;

    fn equal(&self, other: &Rhs) -> Self::Output;
}

impl Equal for f64 {
    type Output = bool;

    fn equal(&self, other: &f64) -> bool {
        self == other
    }
}

pub fn equal<T: Equal<U>, U>(a: &T, b: &U) -> T::Output {
    a.equal(b)
}

pub trait NotEqual<Rhs = Self> {
    type Output;

    fn not_equal(&self, other: &Rhs) -> Self::Output;
}

impl NotEqual for f64 {
    type Output = bool;

    fn not_equal(&self, other: &f64) -> bool {
        self != other
    }
}

pub fn not_equal<T: NotEqual<U>, U>(a: &T, b: &U) -> T::Output {
    a.not_equal(b)
}
