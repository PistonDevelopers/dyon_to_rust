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
    println(&binop::add(&0.0, &1.0));
    println(&binop::mul(&1.0, &2.0));
    println(&binop::div(&4.0, &2.0));
    println(&binop::sub(&5.0, &4.0));
    println(&binop::rem(&5.0, &4.0));
    println(&binop::add(&[1.0, 2.0, 0.0, 0.0], &[2.0, 3.0, 0.0, 0.0]));
    println(&binop::sub(&[1.0, 2.0, 0.0, 0.0], &[2.0, 3.0, 0.0, 0.0]));
    println(&binop::mul(&[1.0, 2.0, 0.0, 0.0], &[2.0, 3.0, 0.0, 0.0]));
    println(&binop::div(&[2.0, 4.0, 6.0, 8.0], &[2.0, 2.0, 2.0, 2.0]));
    println(&binop::rem(&[3.0, 4.0, 0.0, 0.0], &[2.0, 2.0, 1.0, 1.0]));
    println(&binop::mul(&[1.0, 2.0, 0.0, 0.0], &2.0));
    println(&binop::sub(&[1.0, 2.0, 0.0, 0.0], &2.0));
    println(&binop::add(&[1.0, 2.0, 0.0, 0.0], &2.0));
    println(&binop::rem(&[3.0, 1.0, 0.0, 0.0], &2.0));
    println(&binop::div(&[2.0, 3.0, 0.0, 0.0], &2.0));
    println(&binop::add(&1.0, &[1.0, 2.0, 0.0, 0.0]));
    println(&binop::sub(&1.0, &[1.0, 2.0, 0.0, 0.0]));
    println(&binop::mul(&1.0, &[1.0, 2.0, 0.0, 0.0]));
    println(&binop::rem(&4.0, &[1.0, 2.0, 3.0, 4.0]));
    println(&binop::div(&4.0, &[1.0, 2.0, 3.0, 4.0]));
    println(&binop::dot(&[1.0, 2.0, 0.0, 0.0], &[2.0, 1.0, 0.0, 0.0]));
    println(&binop::cross(&[1.0, 2.0, 0.0, 0.0], &[2.0, 1.0, 0.0, 0.0]));
    println(&binop::pow(&2.0, &3.0));
    println(&binop::pow(&[1.0, 2.0, 0.0, 0.0], &2.0));
    println(&binop::mul(&true, &false));
    println(&binop::add(&true, &false));
    println(&(true && false));
    println(&(true || false));
}
