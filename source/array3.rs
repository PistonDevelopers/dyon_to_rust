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
    let mut _0 = vec![variable(&vec![variable(&1.0), variable(&"hi")]), variable(&vec![1.0, 2.0])];
    println(&_0);
    let mut _1 = vec![vec![1.0, 2.0], vec![foo(), 4.0]];
    println(&_1);
    let mut _2 = vec![vec![1.0, 2.0], vec![]];
    println(&_2);
}
fn foo() -> f64 {
    return 3.0
}
