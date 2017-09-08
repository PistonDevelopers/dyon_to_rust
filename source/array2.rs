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
    let mut _0 = vec![variable(&1.0), variable(&"hi")];
    println(&_0);
    let mut _1 = vec![variable(&1.0), variable(&false)];
    println(&_1);
    let mut _2 = vec![variable(&false), variable(&"hi")];
    println(&_2);
    let mut _3 = vec![variable(&[1.0, 3.0, 0.0, 0.0]), variable(&1.0)];
    println(&_3);
    let mut _4 = vec![1.0, 2.0];
    println(&_4);
    let mut _5 = vec![false, true];
    println(&_5);
    let mut _6 = vec!["hi", "hello"];
    println(&_6);
    let mut _7 = vec![[1.0, 2.0, 0.0, 0.0], [3.0, 4.0, 0.0, 0.0]];
    println(&_7);
}
