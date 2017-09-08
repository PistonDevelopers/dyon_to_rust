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
    foo_str(&"hi");
    foo_f64(&4.0);
    foo_bool(&true);
    foo_arr_f64(&vec![1.0, 2.0, 3.0]);
    foo_arr_bool(&vec![true, true, false]);
    foo_arr_str(&vec!["hi"]);
    foo_vec4(&[1.0, 2.0, 0.0, 0.0]);
    foo_arr_vec4(&vec![[1.0, 2.0, 0.0, 0.0]]);
}
fn foo_str(_0: &str) {
    println(&_0);
}
fn foo_f64(_0: &f64) {
    println(&_0);
}
fn foo_bool(_0: &bool) {
    println(&_0);
}
fn foo_arr_f64(_0: &Vec<f64>) {
    println(&_0);
}
fn foo_arr_bool(_0: &Vec<bool>) {
    println(&_0);
}
fn foo_arr_str(_0: &Vec<&str>) {
    println(&_0);
}
fn foo_vec4(_0: &[f32; 4]) {
    println(&_0);
}
fn foo_arr_vec4(_0: &Vec<[f32; 4]>) {
    println(&_0);
}
