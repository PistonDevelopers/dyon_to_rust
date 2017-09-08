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
    let mut _0 = 0.0;
    let mut _1 = 1.0;
    println(&compop::less(&_0, &_1));
    println(&compop::less_or_equal(&_0, &_1));
    println(&compop::greater(&_0, &_1));
    println(&compop::greater_or_equal(&_0, &_1));
    println(&compop::equal(&_0, &_1));
    println(&compop::not_equal(&_0, &_1));
}
