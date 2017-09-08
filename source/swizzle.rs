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
    let mut _0 = [1.0, 2.0, 0.0, 0.0];
    println(&{
        let ref _1 = _0;
        [index::vec4_look_up(_1, 1), index::vec4_look_up(_1, 0), 0.0, 0.0]
    });
    println(&{
        let ref _1 = _0;
        [index::vec4_look_up(_1, 0), index::vec4_look_up(_1, 0), 0.0, 0.0]
    });
    println(&{
        let ref _1 = _0;
        [index::vec4_look_up(_1, 0), index::vec4_look_up(_1, 2), 0.0, 0.0]
    });
    println(&{
        let ref _1 = _0;
        [index::vec4_look_up(_1, 0), index::vec4_look_up(_1, 2), 0.0, 0.0]
    });
    println(&{
        let ref _1 = _0;
        [index::vec4_look_up(_1, 2), index::vec4_look_up(_1, 1), 0.0, 0.0]
    });
    println(&{
        let ref _1 = _0;
        let ref _2 = _0;
        [index::vec4_look_up(_1, 0), index::vec4_look_up(_1, 1), index::vec4_look_up(_2, 1), index::vec4_look_up(_2, 0)]
    });
    println(&{
        let ref _1 = _0;
        [index::vec4_look_up(_1, 2), index::vec4_look_up(_1, 0), index::vec4_look_up(_1, 1), 0.0]
    });
}
