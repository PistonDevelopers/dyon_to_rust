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
    let mut _0 = 2.0;
    foo(&mut _0);
    println(&_0);
}
fn foo(mut _0: &mut f64) {
    assign(&mut _0, &3.0);
}
