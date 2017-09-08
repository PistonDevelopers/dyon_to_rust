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
    println(&[1.0, 0.0, 0.0, 0.0]);
    println(&[1.0, 2.0, 0.0, 0.0]);
    println(&[1.0, 2.0, 3.0, 0.0]);
    println(&[1.0, 2.0, 3.0, 4.0]);
}
