#![allow(unused_imports)]
#![allow(unreachable_code)]

extern crate dyon;
extern crate dyon_to_rust;

use std::sync::Arc;
use std::collections::HashMap;

use dyon::{Variable, Object};
use dyon_to_rust::intrinsics::*;
use dyon_to_rust::*;

fn main() {
    let mut _0 = |_0| {
            binop::add(&_0, &1.0)
        };
    println(&foo(&_0));
}
fn foo(_0: &Fn(f64) -> f64) -> f64 {
    return (_0)(0.0)
}
