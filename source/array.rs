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
    println(&vec![1.0, 2.0, 3.0]);
    let mut _0 = vec![1.0];
    _0[0] = 2.0;
    println(&_0[0]);
}
