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
    let mut _0 = vec![vec![vec![1.0, 2.0], vec![3.0, 4.0]]];
    let mut _1: f64 = 0.0;
    let _2: f64 = len(&_0);
    loop {
        if _1 >= _2 {break};
        {
            let mut _2: f64 = 0.0;
            let _3: f64 = len(&_0[index::ind(_1)]);
            loop {
                if _2 >= _3 {break};
                {
                    let mut _3: f64 = 0.0;
                    let _4: f64 = len(&_0[index::ind(_1)][index::ind(_2)]);
                    loop {
                        if _3 >= _4 {break};
                        {
                            println(&_0[index::ind(_1)][index::ind(_2)][index::ind(_3)]);
                        }
                        _3 += 1.0;
                    };
                }
                _2 += 1.0;
            };
        }
        _1 += 1.0;
    };
}
