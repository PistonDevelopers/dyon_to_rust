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
        let mut _1: Secret<bool, f64> = Secret::new_bool(true);
        let _2: f64 = 3.0;
        loop {
            if _0 >= _2 {break};
            _1 &= {
                compop::greater(&_0, &1.0)
            };
            if !cond(&_1) {
                _1.secret.push(_0);
                break;
            }
            _0 += 1.0;
        }
        _1
    };
    if cond(&unop::not(&_0)) {
        let mut _1 = why(&unop::not(&_0));
        println(&_1)
    };
    let mut _1 = {
        let mut _1: f64 = 0.0;
        let mut _2: Secret<bool, f64> = Secret::new_bool(false);
        let _3: f64 = 3.0;
        loop {
            if _1 >= _3 {break};
            _2 |= {
                compop::greater(&_1, &1.0)
            };
            if cond(&_2) {
                _2.secret.push(_1);
                break;
            }
            _1 += 1.0;
        }
        _2
    };
    if cond(&_1) {
        let mut _2 = why(&_1);
        println(&_2)
    };
}
