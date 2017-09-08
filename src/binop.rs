//! Binary operators.

pub trait Add<Rhs = Self> {
    type Output;

    fn add(&self, other: &Rhs) -> Self::Output;
}

impl<'a, T: Add> Add for &'a T {
    type Output = T::Output;

    fn add(&self, other: &Self) -> T::Output {
        (*self).add(*other)
    }
}

impl<'a, T: Add> Add<T> for &'a T {
    type Output = T::Output;

    fn add(&self, other: &T) -> T::Output {
        (*self).add(other)
    }
}

impl Add for f64 {
    type Output = f64;

    fn add(&self, other: &Self) -> Self {
        self + other
    }
}

impl Add for [f32; 4] {
    type Output = [f32; 4];

    fn add(&self, other: &Self) -> Self {
        [
            self[0] + other[0],
            self[1] + other[1],
            self[2] + other[2],
            self[3] + other[3]
        ]
    }
}

impl Add<f64> for [f32; 4] {
    type Output = [f32; 4];

    fn add(&self, other: &f64) -> Self {
        let rhs = *other as f32;
        [
            self[0] + rhs,
            self[1] + rhs,
            self[2] + rhs,
            self[3] + rhs
        ]
    }
}

impl Add<[f32; 4]> for f64 {
    type Output = [f32; 4];

    fn add(&self, other: &[f32; 4]) -> [f32; 4] {
        let lhs = *self as f32;
        [
            lhs + other[0],
            lhs + other[1],
            lhs + other[2],
            lhs + other[3]
        ]
    }
}

impl Add for bool {
    type Output = bool;

    fn add(&self, other: &bool) -> bool {
        *self || *other
    }
}

pub fn add<T: Add<U>, U>(a: &T, b: &U) -> T::Output {
    a.add(b)
}

pub trait Mul<Rhs = Self> {
    type Output;

    fn mul(&self, other: &Rhs) -> Self::Output;
}

impl<'a, T: Mul> Mul for &'a T {
    type Output = T::Output;

    fn mul(&self, other: &Self) -> T::Output {
        (*self).mul(*other)
    }
}

impl<'a, T: Mul> Mul<T> for &'a T {
    type Output = T::Output;

    fn mul(&self, other: &T) -> T::Output {
        (*self).mul(other)
    }
}

impl Mul for f64 {
    type Output = f64;

    fn mul(&self, other: &Self) -> Self {
        self * other
    }
}

impl Mul for [f32; 4] {
    type Output = [f32; 4];

    fn mul(&self, other: &Self) -> Self {
        [
            self[0] * other[0],
            self[1] * other[1],
            self[2] * other[2],
            self[3] * other[3]
        ]
    }
}

impl Mul<f64> for [f32; 4] {
    type Output = [f32; 4];

    fn mul(&self, other: &f64) -> Self {
        let rhs = *other as f32;
        [
            self[0] * rhs,
            self[1] * rhs,
            self[2] * rhs,
            self[3] * rhs
        ]
    }
}

impl Mul<[f32; 4]> for f64 {
    type Output = [f32; 4];

    fn mul(&self, other: &[f32; 4]) -> [f32; 4] {
        let lhs = *self as f32;
        [
            lhs * other[0],
            lhs * other[1],
            lhs * other[2],
            lhs * other[3]
        ]
    }
}

impl Mul for bool {
    type Output = bool;

    fn mul(&self, other: &bool) -> bool {
        *self && *other
    }
}

pub fn mul<T: Mul<U>, U>(a: &T, b: &U) -> T::Output {
    a.mul(b)
}

pub trait Div<Rhs = Self> {
    type Output;

    fn div(&self, other: &Rhs) -> Self::Output;
}

impl<'a, T: Div> Div for &'a T {
    type Output = T::Output;

    fn div(&self, other: &Self) -> T::Output {
        (*self).div(*other)
    }
}

impl<'a, T: Div<T>> Div<T> for &'a T {
    type Output = T::Output;

    fn div(&self, other: &T) -> T::Output {
        (*self).div(other)
    }
}

impl Div for f64 {
    type Output = f64;

    fn div(&self, other: &Self) -> Self {
        self / other
    }
}

impl Div for [f32; 4] {
    type Output = [f32; 4];

    fn div(&self, other: &Self) -> Self {
        [
            self[0] / other[0],
            self[1] / other[1],
            self[2] / other[2],
            self[3] / other[3]
        ]
    }
}

impl Div<f64> for [f32; 4] {
    type Output = [f32; 4];

    fn div(&self, other: &f64) -> Self {
        let rhs = *other as f32;
        [
            self[0] / rhs,
            self[1] / rhs,
            self[2] / rhs,
            self[3] / rhs
        ]
    }
}

impl Div<[f32; 4]> for f64 {
    type Output = [f32; 4];

    fn div(&self, other: &[f32; 4]) -> [f32; 4] {
        let lhs = *self as f32;
        [
            lhs / other[0],
            lhs / other[1],
            lhs / other[2],
            lhs / other[3]
        ]
    }
}

pub fn div<T: Div<U>, U>(a: &T, b: &U) -> T::Output {
    a.div(b)
}

pub trait Sub<Rhs = Self> {
    type Output;

    fn sub(&self, other: &Rhs) -> Self::Output;
}

impl<'a, T: Sub> Sub for &'a T {
    type Output = T::Output;

    fn sub(&self, other: &Self) -> T::Output {
        (*self).sub(*other)
    }
}

impl<'a, T: Sub> Sub<T> for &'a T {
    type Output = T::Output;

    fn sub(&self, other: &T) -> T::Output {
        (*self).sub(other)
    }
}

impl Sub for f64 {
    type Output = f64;

    fn sub(&self, other: &Self) -> Self {
        self - other
    }
}

impl Sub for [f32; 4] {
    type Output = [f32; 4];

    fn sub(&self, other: &Self) -> Self {
        [
            self[0] - other[0],
            self[1] - other[1],
            self[2] - other[2],
            self[3] - other[3]
        ]
    }
}

impl Sub<f64> for [f32; 4] {
    type Output = [f32; 4];

    fn sub(&self, other: &f64) -> Self {
        [
            self[0] - *other as f32,
            self[1] - *other as f32,
            self[2] - *other as f32,
            self[3] - *other as f32
        ]
    }
}

impl Sub<[f32; 4]> for f64 {
    type Output = [f32; 4];

    fn sub(&self, other: &[f32; 4]) -> [f32; 4] {
        let lhs = *self as f32;
        [
            lhs - other[0],
            lhs - other[1],
            lhs - other[2],
            lhs - other[3]
        ]
    }
}

pub fn sub<T: Sub<U>, U>(a: &T, b: &U) -> T::Output {
    a.sub(b)
}

pub trait Rem<Rhs = Self> {
    type Output;

    fn rem(&self, other: &Rhs) -> Self::Output;
}

impl<'a, T: Rem> Rem for &'a T {
    type Output = T::Output;

    fn rem(&self, other: &Self) -> T::Output {
        (*self).rem(*other)
    }
}

impl<'a, T: Rem> Rem<T> for &'a T {
    type Output = T::Output;

    fn rem(&self, other: &T) -> T::Output {
        (*self).rem(other)
    }
}

impl Rem for f64 {
    type Output = f64;

    fn rem(&self, other: &Self) -> Self {
        self % other
    }
}

impl Rem for [f32; 4] {
    type Output = [f32; 4];

    fn rem(&self, other: &Self) -> Self {
        [
            self[0] % other[0],
            self[1] % other[1],
            self[2] % other[2],
            self[3] % other[3]
        ]
    }
}

impl Rem<f64> for [f32; 4] {
    type Output = [f32; 4];

    fn rem(&self, other: &f64) -> Self {
        let rhs = *other as f32;
        [
            self[0] % rhs,
            self[1] % rhs,
            self[2] % rhs,
            self[3] % rhs
        ]
    }
}

impl Rem<[f32; 4]> for f64 {
    type Output = [f32; 4];

    fn rem(&self, other: &[f32; 4]) -> [f32; 4] {
        let lhs = *self as f32;
        [
            lhs % other[0],
            lhs % other[1],
            lhs % other[2],
            lhs % other[3]
        ]
    }
}

pub fn rem<T: Rem<U>, U>(a: &T, b: &U) -> T::Output {
    a.rem(b)
}

pub fn dot(a: &[f32; 4], b: &[f32; 4]) -> f64 {
    (a[0] * b[0] + a[1] * b[1] + a[2] * b[2] + a[3] * b[3]) as f64
}

pub fn cross(a: &[f32; 4], b: &[f32; 4]) -> [f32; 4] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
        0.0
    ]
}

pub trait Pow<Rhs = Self> {
    type Output;

    fn pow(&self, other: &Rhs) -> Self::Output;
}

impl<'a, T: Pow> Pow for &'a T {
    type Output = T::Output;

    fn pow(&self, other: &Self) -> T::Output {
        (*self).pow(*other)
    }
}

impl<'a, T: Pow> Pow<T> for &'a T {
    type Output = T::Output;

    fn pow(&self, other: &T) -> T::Output {
        (*self).pow(other)
    }
}

impl Pow for f64 {
    type Output = f64;

    fn pow(&self, other: &f64) -> f64 {
        self.powf(*other)
    }
}

impl Pow<f64> for [f32; 4] {
    type Output = [f32; 4];

    fn pow(&self, other: &f64) -> [f32; 4] {
        let rhs = *other as f32;
        [
            self[0].powf(rhs),
            self[1].powf(rhs),
            self[2].powf(rhs),
            self[3].powf(rhs)
        ]
    }
}

pub fn pow<T: Pow<U>, U>(a: &T, b: &U) -> T::Output {
    a.pow(b)
}
