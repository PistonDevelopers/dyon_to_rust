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
    let _1: f64 = 3.0;
    loop {
        if _0 >= _1 {break};
        {
            let mut _1: f64 = 0.0;
            let _2: f64 = 4.0;
            loop {
                if _1 >= _2 {break};
                {
                    println(&binop::add(&_0, &_1));
                }
                _1 += 1.0;
            };
        }
        _0 += 1.0;
    };
}
