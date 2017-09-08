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
    let mut _0 = {
        let mut _0: f64 = 0.0;
        let mut _1: Secret<bool, f64> = Secret::new_bool(false);
        let _2: f64 = 10.0;
        loop {
            if _0 >= _2 {break};
            _1 |= {
                compop::greater(&_0, &3.0)
            };
            if cond(&_1) {
                _1.secret.push(_0);
                break;
            }
            _0 += 1.0;
        }
        _1
    };
    if cond(&false) {
        println(&"yes")
    } else if cond(&true) {
        println(&"maybe")
    } else {
        println(&"no")
    };
}
