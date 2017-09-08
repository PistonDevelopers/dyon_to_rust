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
    let mut _0 = vec![1.0, 2.0, 3.0];
    let mut _1: f64 = 0.0;
    let _2: f64 = len(&_0);
    loop {
        if _1 >= _2 {break};
        {
            println(&_0[index::ind(_1)]);
        }
        _1 += 1.0;
    };
}
