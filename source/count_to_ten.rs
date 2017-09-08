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
    let mut _0: f64 = 0.0;
    let _1: f64 = 10.0;
    loop {
        if _0 >= _1 {break};
        {
            println(&_0);
        }
        _0 += 1.0;
    };
}
