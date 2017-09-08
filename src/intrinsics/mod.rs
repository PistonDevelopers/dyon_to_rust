//! Maps functionality of the Dyon standard library.

pub use self::print::print;
pub use self::print::println;

use Secret;

mod print;

pub fn len<T>(arr: &Vec<T>) -> f64 {
    arr.len() as f64
}

pub fn why<A: Clone>(sec: &Secret<bool, A>) -> Vec<A> {
    let mut res = sec.secret.clone();
    res.reverse();
    res
}

pub fn where_<A: Clone>(sec: &Secret<f64, A>) -> Vec<A> {
    let mut res = sec.secret.clone();
    res.reverse();
    res
}

pub fn clone<T: Clone>(a: &T) -> T {
    a.clone()
}
