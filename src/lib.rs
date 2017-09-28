//! # Dyon to Rust transpiler
//!
//! For more information about Dyon, visit http://www.piston.rs/dyon-tutorial/
//!
//! *Notice: This transpiler is in early development
//! and will contain bugs and missing features!*
//!
//! ### Motivation
//!
//! Dyon has no garbage collector, but uses a lifetime checker.
//! Like Rust, this design choice has the potential to improve runtime performance.
//! Unlike Rust, Dyon has no borrowing semantics, but uses copy-on-write.
//! Dyon also has a mutability checker which makes translating into Rust easier.
//!
//! Dyon is designed for scripting ergonomics and has a syntax and object model
//! similar to Javascript, `go` coroutines (like Go) and optional type checking,
//! and error handling with `?` syntax (like Rust).
//!
//! In addition Dyon has a lot of features for logic (mathematical loops),
//! problem solving (proof composition tracking using secrets),
//! fast generation of text and efficient memory usage (link structure),
//! 4D vectors and html hex colors, closures and current objects (something better than globals).
//!
//! There is a strong motivation to translate Dyon code into Rust,
//! because a lot of code might be prototyped in Dyon during a project.
//! In later stages when the code is tested, performance starts to matter more.
//! Instead of rewriting in Rust for performance, a transpiler makes it easier
//! to automate some parts of this process.
//!
//! ### Goals
//!
//! Assumingly, the Dyon-to-Rust transpiler will never work perfectly.
//! Therefore, the focus will be on a useful subset of Dyon.
//!
//! - In the short term, developers will focus on making translation of snippets work
//! - In the medium term, developers will focus on high performance of snippets code
//! - In the long term, developers will try to support as many language features as
//!   possible, and integrate the transpiler runtime with the Dyon standard library
//! - In very long term, you might be able to transpile a whole module created by
//!   a loader script and expect it to work without problems
//!
//! For example, the transpiler might make assumptions about types in your code,
//! in a way that generates efficient code, but might not pass the Rust compiler.
//! In general, if the transpiler "can not prove it's wrong then it will do it".
//! With other words, it will be optimistic and hope it turns out OK in the end.
//! It is not as dangerous as it sounds, because the Rust compiler is very strict.
//! These assumptions are not meant to allow unsafe code, but only the transpiler's choice of Rust types.
//!
//! By this assumption, the transpiler will become useful at earlier stages,
//! and exploit the similarity between Dyon and Rust to offload some of the work of
//! type checking to the Rust compiler.
//! It will also be easier for people to contribute since the code is mostly
//! translating directly from Dyon AST (Abstract Syntax Tree).
//!
//! ### Design
//!
//! This library serves two roles:
//!
//! 1. As a transpiler
//! 2. Runtime environment for transpiled code
//!
//! A runtime is required because:
//!
//! - Some features in Dyon are different enough that the transpiled code needs
//!   helper methods to look up the right functions.
//! - Some features require interaction with the Dyon library.
//!
//! ### Working on the transpiler
//!
//! The "source" folder contains a pair of ".dyon" and ".rs" files with same name.
//! Files ending with ".dyon" contains the original Dyon source,
//! and files ending with ".rs" contains the translated code in Rust.
//!
//! The source files are checked when typing `cargo test` in the Terminal window.
//! An error will be reported if there are any character mismatch.
//! Therefore, when making changes to the transpiler,
//! you have to go through all failed cases and check that the code turns out right.
//!
//! This workflow is very strict, but it helps the confidence that some changes
//! will not affect translation of existing code too much.
//!
//! There are two special files in the "source" folder:
//!
//! 1. "test.dyon" - used to write some test code.
//! 2. "test.rs" - generated Rust code
//!
//! The "tests::test" unit test overwrites "test.rs" by default.
//! You can change this behavior with a flag in the unit test code.
//!
//! To compile, type `rustc source/test.rs -L target/debug/deps` in the Terminal window.
//!
//! To run, type `./test`.
//!
//! ### Behind the scenes
//!
//! The transpiler is really just a huge function generating Rust code (single file) from a Dyon module.
//!
//! The Dyon mode contains the AST (Abstract Syntax Tree) that is used when executing Dyon.
//! It contains all information required to run Dyon code, except for functions.
//! Functions are stored in the module and have different kinds depending on
//! whether they are intrinsics (Dyon standard library),
//! loaded functions (Dyon script) or external functions (Rust).
//!
//! The AST contains static ids resolved upfront that tells where variables live on the stack.
//! This means that the transpiler only needs to keep track of the length of the stack.
//! In the code, this is passed as the `stack_len` parameter.
//!
//! The correct usage of stack length tracking is determined from Dyon's runtime behavior.
//! Therefore, the transpiler is mirroring the behavior of how Dyon executes.
//! Variables with overlapping indices are kept in separate scopes using Rust blocks.
//!
//! Function calls uses relative indices because Dyon modules can be composed dynamically.
//! This means that extra testing is required when a module depends on other modules.
//!
//! ### Functionality
//!
//! Currently, due to very early stage, there is no map of supported language features.
//!
//! At the moment, the generated Rust code only uses indices.
//!
//! For example, you will not see the variable names:
//!
//! ```ignore
//! let mut _0 = 2.0;
//! foo(&mut _0);
//! ```
//!
//! In the future you might be able to tell the code generator
//! to use variable names from Dyon through a `CodeSettings` struct.

#![feature(specialization)]

extern crate dyon;
extern crate piston_meta;

use dyon::Module;
use std::io::{self, Write};

pub use secret::{Secret, SecretValue};
pub use cond::cond_eval as cond;
pub use variable::to_variable as variable;
pub use assign::set_assign as assign;

pub mod intrinsics;
pub mod binop;
pub mod compop;
pub mod unop;
pub mod index;

mod cond;
mod secret;
mod variable;
mod assign;

/// Generates code from a Dyon module.
pub fn generate_code<W: Write>(w: &mut W, module: &Module) -> io::Result<()> {
    use dyon::ast::*;
    use dyon::ty::Type;

    fn generate_tabs<W: Write>(w: &mut W, tabs: u16) -> io::Result<()> {
        for _ in 0..4 * tabs {
            write!(w, " ")?;
        }

        Ok(())
    }

    fn generate_call<W: Write>(
        w: &mut W,
        tabs: u16,
        stack_len: usize,
        call: &Call,
        module: &Module
    ) -> io::Result<()> {
        use dyon::FnIndex;

        if &*call.name == "where" {
            write!(w, "{}_(", call.name)?;
        } else {
            if let Some(ind) = call.name.find('(') {
                write!(w, "{}(", &call.name[..ind])?;
            } else {
                write!(w, "{}(", call.name)?;
            }
        }

        let return_var = match call.f_index.get() {
            FnIndex::Loaded(f_index) => {
                // TODO: Should this be computed relative somehow?
                let new_index = f_index as usize;
                if module.functions[new_index].returns() {1} else {0}
            }
            // TODO: Check other cases.
            _ => 0
        };
        let mut mutable_args = vec![];
        if let Some(ind) = call.name.find('(') {
            let len = call.name.len();
            let mut_info = &call.name[ind + 1..len - 1];
            for (i, arg) in mut_info.split(',').enumerate() {
                if arg == "mut" {mutable_args.push(i)};
            }
        }
        let n = call.args.len();
        for (i, exp) in call.args.iter().enumerate() {
            let mutable = mutable_args.iter().any(|&j| j == i);
            if mutable {
                write!(w, "&mut ")?;
            } else {
                write!(w, "&")?;
            }
            generate_expression(w, tabs, stack_len + return_var, exp, module)?;
            if (i + 1) != n {
                write!(w, ", ")?;
            }
        }
        write!(w, ")")?;

        Ok(())
    }

    fn generate_text<W: Write>(w: &mut W, text: &Text) -> io::Result<()> {
        use piston_meta::json;

        json::write_string(w, &text.text)
    }

    fn generate_for<W: Write>(
        w: &mut W,
        tabs: u16,
        stack_len: usize,
        for_expr: &For,
        module: &Module
    ) -> io::Result<()> {
        generate_expression(w, tabs, stack_len, &for_expr.init, module)?;
        writeln!(w, ";")?;
        generate_tabs(w, tabs)?;
        if let Some(ref label) = for_expr.label {
            writeln!(w, "'{}: loop {{", label)?;
        } else {
            writeln!(w, "loop {{")?;
        }
        generate_tabs(w, tabs + 1)?;
        write!(w, "if !cond(&")?;
        generate_expression(w, tabs + 2, stack_len + 1, &for_expr.cond, module)?;
        writeln!(w, ") {{break}};")?;
        generate_tabs(w, tabs + 1)?;
        writeln!(w, "{{")?;
        generate_block(w, tabs + 2, stack_len + 1, &for_expr.block, module)?;
        writeln!(w, ";")?;
        generate_tabs(w, tabs + 1)?;
        writeln!(w, "}}")?;
        generate_tabs(w, tabs + 1)?;
        generate_expression(w, tabs + 1, stack_len, &for_expr.step, module)?;
        writeln!(w, ";")?;
        generate_tabs(w, tabs)?;
        write!(w, "}}")?;
        Ok(())
    }

    fn generate_for_n<W: Write>(
        w: &mut W,
        tabs: u16,
        stack_len: usize,
        for_n: &ForN,
        module: &Module
    ) -> io::Result<()> {
        let id = stack_len;
        write!(w, "let mut _{}: f64 = ", id)?;
        if let Some(ref exp) = for_n.start {
            generate_expression(w, tabs, stack_len, exp, module)?;
        } else {
            write!(w, "0.0")?;
        }
        writeln!(w, ";")?;

        let n_id = stack_len + 1;
        generate_tabs(w, tabs)?;
        write!(w, "let _{}: f64 = ", n_id)?;
        generate_expression(w, tabs, stack_len, &for_n.end, module)?;
        writeln!(w, ";")?;

        generate_tabs(w, tabs)?;
        if let Some(ref label) = for_n.label {
            writeln!(w, "'{}: loop {{", label)?;
        } else {
            writeln!(w, "loop {{")?;
        }

        generate_tabs(w, tabs + 1)?;
        writeln!(w, "if _{} >= _{} {{break}};", id, n_id)?;

        generate_tabs(w, tabs + 1)?;
        writeln!(w, "{{")?;
        generate_block(w, tabs + 2, stack_len + 1, &for_n.block, module)?;
        writeln!(w, ";")?;
        generate_tabs(w, tabs + 1)?;
        writeln!(w, "}}")?;

        generate_tabs(w, tabs + 1)?;
        writeln!(w, "_{} += 1.0;", id)?;

        generate_tabs(w, tabs)?;
        write!(w, "}}")?;

        Ok(())
    }

    fn generate_all_n<W: Write>(
        w: &mut W,
        tabs: u16,
        stack_len: usize,
        for_n: &ForN,
        module: &Module
    ) -> io::Result<()> {
        writeln!(w, "{{")?;

        let id = stack_len;
        generate_tabs(w, tabs + 1)?;
        write!(w, "let mut _{}: f64 = ", id)?;
        if let Some(ref exp) = for_n.start {
            generate_expression(w, tabs, stack_len, exp, module)?;
        } else {
            write!(w, "0.0")?;
        }
        writeln!(w, ";")?;

        let all_id = stack_len + 1;
        generate_tabs(w, tabs + 1)?;
        writeln!(w, "let mut _{}: Secret<bool, f64> = Secret::new_bool(true);", all_id)?;

        let n_id = stack_len + 2;
        generate_tabs(w, tabs + 1)?;
        write!(w, "let _{}: f64 = ", n_id)?;
        generate_expression(w, tabs, stack_len, &for_n.end, module)?;
        writeln!(w, ";")?;

        generate_tabs(w, tabs + 1)?;
        writeln!(w, "loop {{")?;

        generate_tabs(w, tabs + 2)?;
        writeln!(w, "if _{} >= _{} {{break}};", id, n_id)?;

        generate_tabs(w, tabs + 2)?;
        writeln!(w, "_{} &= {{", all_id)?;
        generate_block(w, tabs + 3, stack_len + 1, &for_n.block, module)?;
        writeln!(w, "")?;
        generate_tabs(w, tabs + 2)?;
        writeln!(w, "}};")?;

        generate_tabs(w, tabs + 2)?;
        writeln!(w, "if !cond(&_{}) {{", all_id)?;
        generate_tabs(w, tabs + 3)?;
        writeln!(w, "_{}.secret.push(_{});", all_id, id)?;
        generate_tabs(w, tabs + 3)?;
        writeln!(w, "break;")?;
        generate_tabs(w, tabs + 2)?;
        writeln!(w, "}}")?;

        generate_tabs(w, tabs + 2)?;
        writeln!(w, "_{} += 1.0;", id)?;

        generate_tabs(w, tabs + 1)?;
        writeln!(w, "}}")?;

        generate_tabs(w, tabs + 1)?;
        writeln!(w, "_{}", all_id)?;
        generate_tabs(w, tabs)?;
        write!(w, "}}")?;

        Ok(())
    }

    fn generate_any_n<W: Write>(
        w: &mut W,
        tabs: u16,
        stack_len: usize,
        for_n: &ForN,
        module: &Module
    ) -> io::Result<()> {
        writeln!(w, "{{")?;

        let id = stack_len;
        generate_tabs(w, tabs + 1)?;
        write!(w, "let mut _{}: f64 = ", id)?;
        if let Some(ref exp) = for_n.start {
            generate_expression(w, tabs, stack_len, exp, module)?;
        } else {
            write!(w, "0.0")?;
        }
        writeln!(w, ";")?;

        let any_id = stack_len + 1;
        generate_tabs(w, tabs + 1)?;
        writeln!(w, "let mut _{}: Secret<bool, f64> = Secret::new_bool(false);", any_id)?;

        let n_id = stack_len + 2;
        generate_tabs(w, tabs + 1)?;
        write!(w, "let _{}: f64 = ", n_id)?;
        generate_expression(w, tabs, stack_len, &for_n.end, module)?;
        writeln!(w, ";")?;

        generate_tabs(w, tabs + 1)?;
        writeln!(w, "loop {{")?;

        generate_tabs(w, tabs + 2)?;
        writeln!(w, "if _{} >= _{} {{break}};", id, n_id)?;

        generate_tabs(w, tabs + 2)?;
        writeln!(w, "_{} |= {{", any_id)?;
        generate_block(w, tabs + 3, stack_len + 1, &for_n.block, module)?;
        writeln!(w, "")?;
        generate_tabs(w, tabs + 2)?;
        writeln!(w, "}};")?;

        generate_tabs(w, tabs + 2)?;
        writeln!(w, "if cond(&_{}) {{", any_id)?;
        generate_tabs(w, tabs + 3)?;
        writeln!(w, "_{}.secret.push(_{});", any_id, id)?;
        generate_tabs(w, tabs + 3)?;
        writeln!(w, "break;")?;
        generate_tabs(w, tabs + 2)?;
        writeln!(w, "}}")?;

        generate_tabs(w, tabs + 2)?;
        writeln!(w, "_{} += 1.0;", id)?;

        generate_tabs(w, tabs + 1)?;
        writeln!(w, "}}")?;

        generate_tabs(w, tabs + 1)?;
        writeln!(w, "_{}", any_id)?;
        generate_tabs(w, tabs)?;
        write!(w, "}}")?;

        Ok(())
    }

    fn generate_sum<W: Write>(
        w: &mut W,
        tabs: u16,
        stack_len: usize,
        for_n: &ForN,
        module: &Module
    ) -> io::Result<()> {
        writeln!(w, "{{")?;

        let id = stack_len;
        generate_tabs(w, tabs + 1)?;
        write!(w, "let mut _{}: f64 = ", id)?;
        if let Some(ref exp) = for_n.start {
            generate_expression(w, tabs, stack_len, exp, module)?;
        } else {
            write!(w, "0.0")?;
        }
        writeln!(w, ";")?;

        let sum_id = stack_len + 1;
        generate_tabs(w, tabs + 1)?;
        writeln!(w, "let mut _{}: f64 = 0.0;", sum_id)?;

        generate_tabs(w, tabs + 1)?;
        writeln!(w, "loop {{")?;

        generate_tabs(w, tabs + 2)?;
        write!(w, "if _{} >= ", id)?;
        generate_expression(w, tabs, stack_len, &for_n.end, module)?;
        writeln!(w, " {{break}};")?;

        generate_tabs(w, tabs + 2)?;
        writeln!(w, "_{} += {{", sum_id)?;
        generate_block(w, tabs + 3, stack_len + 1, &for_n.block, module)?;
        writeln!(w, "")?;
        generate_tabs(w, tabs + 2)?;
        writeln!(w, "}};")?;

        generate_tabs(w, tabs + 2)?;
        writeln!(w, "_{} += 1.0;", id)?;

        generate_tabs(w, tabs + 1)?;
        writeln!(w, "}}")?;

        generate_tabs(w, tabs + 1)?;
        writeln!(w, "_{}", sum_id)?;
        generate_tabs(w, tabs)?;
        write!(w, "}}")?;

        Ok(())
    }

    fn generate_prod<W: Write>(
        w: &mut W,
        tabs: u16,
        stack_len: usize,
        for_n: &ForN,
        module: &Module
    ) -> io::Result<()> {
        writeln!(w, "{{")?;

        let id = stack_len;
        generate_tabs(w, tabs + 1)?;
        write!(w, "let mut _{}: f64 = ", id)?;
        if let Some(ref exp) = for_n.start {
            generate_expression(w, tabs, stack_len, exp, module)?;
        } else {
            write!(w, "0.0")?;
        }
        writeln!(w, ";")?;

        let prod_id = stack_len + 1;
        generate_tabs(w, tabs + 1)?;
        writeln!(w, "let mut _{}: f64 = 1.0;", prod_id)?;

        generate_tabs(w, tabs + 1)?;
        writeln!(w, "loop {{")?;

        generate_tabs(w, tabs + 2)?;
        write!(w, "if _{} >= ", id)?;
        generate_expression(w, tabs, stack_len, &for_n.end, module)?;
        writeln!(w, " {{break}};")?;

        generate_tabs(w, tabs + 2)?;
        writeln!(w, "_{} *= {{", prod_id)?;
        generate_block(w, tabs + 3, stack_len + 1, &for_n.block, module)?;
        writeln!(w, "")?;
        generate_tabs(w, tabs + 2)?;
        writeln!(w, "}};")?;

        generate_tabs(w, tabs + 2)?;
        writeln!(w, "_{} += 1.0;", id)?;

        generate_tabs(w, tabs + 1)?;
        writeln!(w, "}}")?;

        generate_tabs(w, tabs + 1)?;
        writeln!(w, "_{}", prod_id)?;
        generate_tabs(w, tabs)?;
        write!(w, "}}")?;

        Ok(())
    }

    fn generate_max_n<W: Write>(
        w: &mut W,
        tabs: u16,
        stack_len: usize,
        for_n: &ForN,
        module: &Module
    ) -> io::Result<()> {
        writeln!(w, "{{")?;

        let id = stack_len;
        generate_tabs(w, tabs + 1)?;
        write!(w, "let mut _{}: f64 = ", id)?;
        if let Some(ref exp) = for_n.start {
            generate_expression(w, tabs, stack_len, exp, module)?;
        } else {
            write!(w, "0.0")?;
        }
        writeln!(w, ";")?;

        let max_id = stack_len + 1;
        generate_tabs(w, tabs + 1)?;
        writeln!(w, "let mut _{}: Secret<f64, f64> = Secret::new_f64(::std::f64::NAN);", max_id)?;

        let track_id = stack_len + 2;
        generate_tabs(w, tabs + 1)?;
        writeln!(w, "let mut _{}: Option<f64> = None;", track_id)?;

        let n_id = stack_len + 3;
        generate_tabs(w, tabs + 1)?;
        write!(w, "let _{}: f64 = ", n_id)?;
        generate_expression(w, tabs, stack_len, &for_n.end, module)?;
        writeln!(w, ";")?;

        generate_tabs(w, tabs + 1)?;
        writeln!(w, "loop {{")?;

        generate_tabs(w, tabs + 2)?;
        writeln!(w, "if _{} >= _{} {{break}};", id, n_id)?;

        generate_tabs(w, tabs + 2)?;
        let res_id = stack_len + 3;
        writeln!(w, "let _{} = {{", res_id)?;
        generate_block(w, tabs + 3, stack_len + 1, &for_n.block, module)?;
        writeln!(w, "")?;
        generate_tabs(w, tabs + 2)?;
        writeln!(w, "}};")?;
        generate_tabs(w, tabs + 2)?;
        writeln!(w, "if _{}.val.is_nan() || _{}.value() > _{}.value() {{", max_id, res_id, max_id)?;
        generate_tabs(w, tabs + 3)?;
        writeln!(w, "_{} = _{}.into();", max_id, res_id)?;
        generate_tabs(w, tabs + 3)?;
        writeln!(w, "_{} = Some(_{});", track_id, id)?;
        generate_tabs(w, tabs + 2)?;
        writeln!(w, "}}")?;

        generate_tabs(w, tabs + 2)?;
        writeln!(w, "_{} += 1.0;", id)?;

        generate_tabs(w, tabs + 1)?;
        writeln!(w, "}}")?;

        generate_tabs(w, tabs + 1)?;
        writeln!(w, "if let Some(_{}) = _{} {{", id, track_id)?;
        generate_tabs(w, tabs + 2)?;
        writeln!(w, "_{}.secret.push(_{});", max_id, id)?;
        generate_tabs(w, tabs + 1)?;
        writeln!(w, "}}")?;
        generate_tabs(w, tabs + 1)?;
        writeln!(w, "_{}", max_id)?;
        generate_tabs(w, tabs)?;
        write!(w, "}}")?;

        Ok(())
    }

    fn generate_min_n<W: Write>(
        w: &mut W,
        tabs: u16,
        stack_len: usize,
        for_n: &ForN,
        module: &Module
    ) -> io::Result<()> {
        writeln!(w, "{{")?;

        let id = stack_len;
        generate_tabs(w, tabs + 1)?;
        write!(w, "let mut _{}: f64 = ", id)?;
        if let Some(ref exp) = for_n.start {
            generate_expression(w, tabs, stack_len, exp, module)?;
        } else {
            write!(w, "0.0")?;
        }
        writeln!(w, ";")?;

        let min_id = stack_len + 1;
        generate_tabs(w, tabs + 1)?;
        writeln!(w, "let mut _{}: Secret<f64, f64> = Secret::new_f64(::std::f64::NAN);", min_id)?;

        let track_id = stack_len + 2;
        generate_tabs(w, tabs + 1)?;
        writeln!(w, "let mut _{}: Option<f64> = None;", track_id)?;

        let n_id = stack_len + 3;
        generate_tabs(w, tabs + 1)?;
        write!(w, "let _{}: f64 = ", n_id)?;
        generate_expression(w, tabs, stack_len, &for_n.end, module)?;
        writeln!(w, ";")?;

        generate_tabs(w, tabs + 1)?;
        writeln!(w, "loop {{")?;

        generate_tabs(w, tabs + 2)?;
        writeln!(w, "if _{} >= _{} {{break}};", id, n_id)?;

        generate_tabs(w, tabs + 2)?;
        let res_id = stack_len + 3;
        writeln!(w, "let _{} = {{", res_id)?;
        generate_block(w, tabs + 3, stack_len + 1, &for_n.block, module)?;
        generate_tabs(w, tabs + 2)?;
        writeln!(w, "}};")?;
        generate_tabs(w, tabs + 2)?;
        writeln!(w, "if _{}.val.is_nan() || _{}.value() < _{}.value() {{", min_id, res_id, min_id)?;
        generate_tabs(w, tabs + 3)?;
        writeln!(w, "_{} = _{}.into();", min_id, res_id)?;
        generate_tabs(w, tabs + 3)?;
        writeln!(w, "_{} = Some(_{});", track_id, id)?;
        generate_tabs(w, tabs + 2)?;
        writeln!(w, "}}")?;

        generate_tabs(w, tabs + 2)?;
        writeln!(w, "_{} += 1.0;", id)?;

        generate_tabs(w, tabs + 1)?;
        writeln!(w, "}}")?;

        generate_tabs(w, tabs + 1)?;
        writeln!(w, "if let Some(_{}) = _{} {{", id, track_id)?;
        generate_tabs(w, tabs + 2)?;
        writeln!(w, "_{}.secret.push(_{});", min_id, id)?;
        generate_tabs(w, tabs + 1)?;
        writeln!(w, "}}")?;
        generate_tabs(w, tabs + 1)?;
        writeln!(w, "_{}", min_id)?;
        generate_tabs(w, tabs)?;
        write!(w, "}}")?;

        Ok(())
    }

    fn generate_sift_n<W: Write>(
        w: &mut W,
        tabs: u16,
        stack_len: usize,
        for_n: &ForN,
        module: &Module
    ) -> io::Result<()> {
        writeln!(w, "{{")?;

        let id = stack_len;
        generate_tabs(w, tabs + 1)?;
        write!(w, "let mut _{}: f64 = ", id)?;
        if let Some(ref exp) = for_n.start {
            generate_expression(w, tabs, stack_len, exp, module)?;
        } else {
            write!(w, "0.0")?;
        }
        writeln!(w, ";")?;

        let sift_id = stack_len + 1;
        generate_tabs(w, tabs + 1)?;
        writeln!(w, "let mut _{} = vec![];", sift_id)?;

        generate_tabs(w, tabs + 1)?;
        writeln!(w, "loop {{")?;

        generate_tabs(w, tabs + 2)?;
        write!(w, "if _{} >= ", id)?;
        generate_expression(w, tabs, stack_len, &for_n.end, module)?;
        writeln!(w, " {{break}};")?;

        generate_tabs(w, tabs + 2)?;
        let res_id = stack_len + 3;
        writeln!(w, "let _{} = {{", res_id)?;
        generate_block(w, tabs + 3, stack_len + 1, &for_n.block, module)?;
        writeln!(w, "")?;
        generate_tabs(w, tabs + 2)?;
        writeln!(w, "}};")?;
        generate_tabs(w, tabs + 2)?;
        writeln!(w, "_{}.push(_{});", sift_id, res_id)?;

        generate_tabs(w, tabs + 2)?;
        writeln!(w, "_{} += 1.0;", id)?;

        generate_tabs(w, tabs + 1)?;
        writeln!(w, "}}")?;
        generate_tabs(w, tabs + 1)?;
        writeln!(w, "_{}", sift_id)?;
        generate_tabs(w, tabs)?;
        write!(w, "}}")?;
        Ok(())
    }

    fn generate_number<W: Write>(w: &mut W, number: &Number) -> io::Result<()> {
        write!(w, "{}", number.num)?;
        if number.num % 1.0 == 0.0 {
            write!(w, ".0")?;
        }

        Ok(())
    }

    fn generate_bool<W: Write>(w: &mut W, b: &Bool) -> io::Result<()> {
        write!(w, "{}", b.val)
    }

    fn generate_item<W: Write>(
        w: &mut W,
        tabs: u16,
        stack_len: usize,
        item: &Item,
        module: &Module
    ) -> io::Result<()> {
        if let Some(ref stack_id) = item.static_stack_id.get() {
            write!(w, "_{}", stack_len - stack_id)?;
        }
        for (i, id) in item.ids.iter().enumerate() {
            write!(w, "[")?;
            match *id {
                Id::String(_, ref text) => {
                    write!(w, "&Arc::new({:?}.into())", text)?;
                }
                Id::F64(_, val) => {
                    write!(w, "{}", val as usize)?;
                }
                Id::Expression(ref expr) => {
                    write!(w, "index::ind(")?;
                    generate_expression(w, tabs, stack_len + i, expr, module)?;
                    write!(w, ")")?;
                }
            }
            write!(w, "]")?;
        }

        Ok(())
    }

    fn generate_binop<W: Write>(
        w: &mut W,
        tabs: u16,
        stack_len: usize,
        binop: &BinOpExpression,
        module: &Module
    ) -> io::Result<()> {
        use dyon::ast::BinOp as B;

        match binop.op {
            B::Add => write!(w, "binop::add")?,
            B::Mul => write!(w, "binop::mul")?,
            B::Div => write!(w, "binop::div")?,
            B::Sub => write!(w, "binop::sub")?,
            B::Rem => write!(w, "binop::rem")?,
            B::Dot => write!(w, "binop::dot")?,
            B::Cross => write!(w, "binop::cross")?,
            B::Pow => write!(w, "binop::pow")?,
            B::AndAlso => {
                write!(w, "(")?;
                generate_expression(w, tabs, stack_len, &binop.left, module)?;
                write!(w, " && ")?;
                generate_expression(w, tabs, stack_len, &binop.right, module)?;
                write!(w, ")")?;
                return Ok(())
            }
            B::OrElse => {
                write!(w, "(")?;
                generate_expression(w, tabs, stack_len, &binop.left, module)?;
                write!(w, " || ")?;
                generate_expression(w, tabs, stack_len, &binop.right, module)?;
                write!(w, ")")?;
                return Ok(())
            }
        }
        write!(w, "(&")?;
        generate_expression(w, tabs, stack_len, &binop.left, module)?;
        write!(w, ", &")?;
        generate_expression(w, tabs, stack_len, &binop.right, module)?;
        write!(w, ")")?;

        Ok(())
    }

    fn generate_compare<W: Write>(
        w: &mut W,
        tabs: u16,
        stack_len: usize,
        compare: &Compare,
        module: &Module
    ) -> io::Result<()> {
        use dyon::ast::CompareOp as C;

        match compare.op {
            C::Less => write!(w, "compop::less")?,
            C::LessOrEqual => write!(w, "compop::less_or_equal")?,
            C::Greater => write!(w, "compop::greater")?,
            C::GreaterOrEqual => write!(w, "compop::greater_or_equal")?,
            C::Equal => write!(w, "compop::equal")?,
            C::NotEqual => write!(w, "compop::not_equal")?,
        }
        write!(w, "(&")?;
        generate_expression(w, tabs, stack_len, &compare.left, module)?;
        write!(w, ", &")?;
        generate_expression(w, tabs, stack_len, &compare.right, module)?;
        write!(w, ")")?;
        Ok(())
    }

    fn generate_array<W: Write>(
        w: &mut W,
        tabs: u16,
        stack_len: usize,
        array: &Array,
        module: &Module
    ) -> io::Result<()> {
        // Used to infer types from array.
        //
        // If there are provably different types,
        // then use a Dyon variable inside the array.
        #[derive(PartialEq, Eq, Clone, Debug)]
        enum ArrayType {
            Bool,
            F64,
            Str,
            Vec4,
            Link,
            Variable,
            Object,
            Array(Option<Box<ArrayType>>),
        }

        impl ArrayType {
            fn infer(expr: &Expression) -> Option<Self> {
                use dyon::ast::Expression as E;

                match *expr {
                    E::Number(_) |
                    E::Sum(_) |
                    E::Prod(_) => Some(ArrayType::F64),
                    E::Text(_) => Some(ArrayType::Str),
                    E::Bool(_) => Some(ArrayType::Bool),
                    E::Vec4(_) => Some(ArrayType::Vec4),
                    E::Link(_) => Some(ArrayType::Link),
                    E::Object(_) => Some(ArrayType::Object),
                    E::Array(ref array) => {
                        let mut ty: Option<ArrayType> = None;
                        let n = array.items.len();
                        for i in 0..n {
                            if let Some(ref mut ty) = ty {
                                ty.refine(&array.items[i]);
                            } else {
                                ty = ArrayType::infer(&array.items[i]);
                            }
                        }
                        Some(ArrayType::Array(ty.map(|ty| Box::new(ty))))
                    }
                    _ => None,
                }
            }

            fn refine(&mut self, expr: &Expression) {
                if let Some(array_type) = ArrayType::infer(expr) {
                    // println!("TEST self {:?} array_type {:?}", self, array_type);
                    *self = self.refine_match(&array_type);
                    // println!("TEST new self {:?}", self);
                }
            }

            fn refine_match(&self, other: &ArrayType) -> ArrayType {
                match (&*self, other) {
                    (&ArrayType::Array(None), &ArrayType::Array(Some(_))) => other.clone(),
                    (&ArrayType::Array(Some(_)), &ArrayType::Array(None)) => self.clone(),
                    (&ArrayType::Array(Some(ref a)), &ArrayType::Array(Some(ref b))) => {
                        let res = a.refine_match(b);
                        if res == ArrayType::Variable &&
                           (**a != ArrayType::Variable || **b != ArrayType::Variable)
                        {
                            ArrayType::Variable
                        } else {
                            ArrayType::Array(Some(Box::new(res)))
                        }
                    }
                    (x, y) if x == y => x.clone(),
                    (_, _) => ArrayType::Variable,
                }
            }
        }

        write!(w, "vec![")?;
        let n = array.items.len();

        let mut ty: Option<ArrayType> = None;
        for i in 0..n {
            if let Some(ref mut ty) = ty {
                ty.refine(&array.items[i]);
            } else {
                ty = ArrayType::infer(&array.items[i]);
            }
        }

        if let Some(ArrayType::Variable) = ty {
            // println!("TEST array {:?}", array);
            // println!("TEST ty {:?}", ty);
            for (i, it) in array.items.iter().enumerate() {
                generate_variable(w, tabs, stack_len, it, module)?;
                if (i + 1) != n {
                    write!(w, ", ")?;
                }
            }
        } else {
            for (i, it) in array.items.iter().enumerate() {
                generate_expression(w, tabs, stack_len, it, module)?;
                if (i + 1) != n {
                    write!(w, ", ")?;
                }
            }
        }
        write!(w, "]")?;

        Ok(())
    }

    fn generate_vec4<W: Write>(
        w: &mut W,
        tabs: u16,
        stack_len: usize,
        vec4: &Vec4,
        module: &Module
    ) -> io::Result<()> {
        let n = vec4.args.len();

        // Sum the number of contracted coordinates.
        let mut swizzles = 0;
        for expr in &vec4.args {
            if let &Expression::Swizzle(ref swizzle) = expr {
                swizzles += 1;
                if swizzle.sw2.is_some() {
                    swizzles += 1;
                }
                if swizzle.sw3.is_some() {
                    swizzles += 1;
                }
            }
        }

        if swizzles > 0 {
            writeln!(w, "{{")?;
            let mut swizzle_id: Vec<usize> = vec![];
            let mut swizzles_recount = 0;
            for expr in &vec4.args {
                if let &Expression::Swizzle(ref swizzle) = expr {
                    let id = stack_len + swizzle_id.len();
                    generate_tabs(w, tabs + 1)?;
                    write!(w, "let ref _{} = ", id)?;
                    generate_expression(w, tabs + 2, stack_len + swizzles_recount,
                                        &swizzle.expr, module)?;
                    writeln!(w, ";")?;
                    swizzle_id.push(id);

                    swizzles_recount += 2;
                    if swizzle.sw2.is_some() {
                        swizzles_recount += 1;
                    }
                    if swizzle.sw3.is_some() {
                        swizzles_recount += 1;
                    }
                }
            }
            generate_tabs(w, tabs + 1)?;
            write!(w, "[")?;
            let mut swizzle_ind = 0;
            for (i, exp) in vec4.args.iter().enumerate() {
                if let &Expression::Swizzle(ref swizzle) = exp {
                    write!(w, "index::vec4_look_up(_{}, {})", swizzle_id[swizzle_ind], swizzle.sw0)?;
                    write!(w, ", ")?;
                    write!(w, "index::vec4_look_up(_{}, {})", swizzle_id[swizzle_ind], swizzle.sw1)?;
                    if let Some(sw2) = swizzle.sw2 {
                        write!(w, ", ")?;
                        write!(w, "index::vec4_look_up(_{}, {})", swizzle_id[swizzle_ind], sw2)?;
                        if let Some(sw3) = swizzle.sw3 {
                            write!(w, ", ")?;
                            write!(w, "index::vec4_look_up(_{}, {})", swizzle_id[swizzle_ind], sw3)?;
                        }
                    }
                    swizzle_ind += 1;
                } else {
                    generate_expression(w, tabs + 2, stack_len, exp, module)?;
                }
                // Skip zeroes that are removed because of swizzling.
                if (i + 1 + swizzles) >= n {break;}
                if (i + 1) != n {
                    write!(w, ", ")?;
                }
            }
            writeln!(w, "]")?;
            generate_tabs(w, tabs)?;
            write!(w, "}}")?;
        } else {
            write!(w, "[")?;
            for (i, exp) in vec4.args.iter().enumerate() {
                generate_expression(w, tabs, stack_len, exp, module)?;
                // Skip zeroes that are removed because of swizzling.
                if (i + 1) != n {
                    write!(w, ", ")?;
                }
            }
            write!(w, "]")?;
        }

        Ok(())
    }

    fn generate_assign<W: Write>(
        w: &mut W,
        tabs: u16,
        stack_len: usize,
        assign: &Assign,
        module: &Module
    ) -> io::Result<()> {
        use dyon::ast::AssignOp as A;

        if let Expression::Item(ref item) = assign.left {
            if item.ids.len() == 0 {
                match assign.op {
                    A::Assign => {
                        write!(w, "let mut _{} = ", stack_len)?;
                    }
                    A::Set => {
                        write!(w, "assign(&mut ")?;
                        generate_expression(w, tabs, stack_len, &assign.left, module)?;
                        write!(w, ", &")?;
                        generate_expression(w, tabs, stack_len, &assign.right, module)?;
                        write!(w, ")")?;
                        return Ok(())
                    }
                    A::Add => {
                        generate_expression(w, tabs, stack_len, &assign.left, module)?;
                        write!(w, " += ")?;
                    }
                    A::Sub => {
                        generate_expression(w, tabs, stack_len, &assign.left, module)?;
                        write!(w, " -= ")?;
                    }
                    A::Mul => {
                        generate_expression(w, tabs, stack_len, &assign.left, module)?;
                        write!(w, " *= ")?;
                    }
                    A::Div => {
                        generate_expression(w, tabs, stack_len, &assign.left, module)?;
                        write!(w, " /= ")?;
                    }
                    A::Rem => {
                        generate_expression(w, tabs, stack_len, &assign.left, module)?;
                        write!(w, " %= ")?;
                    }
                    A::Pow => {
                        generate_expression(w, tabs, stack_len, &assign.left, module)?;
                        write!(w, " = binop::pow(&, ")?;
                        generate_expression(w, tabs, stack_len, &assign.left, module)?;
                        write!(w, ", ")?;
                        generate_expression(w, tabs, stack_len, &assign.right, module)?;
                        write!(w, ")")?;
                        return Ok(())
                    }
                }
                generate_expression(w, tabs, stack_len, &assign.right, module)?;
            } else {
                generate_expression(w, tabs, stack_len, &assign.left, module)?;
                write!(w, " = ")?;
                generate_expression(w, tabs, stack_len, &assign.right, module)?;
            }
        }
        Ok(())
    }

    fn generate_if<W: Write>(
        w: &mut W,
        tabs: u16,
        stack_len: usize,
        if_expr: &If,
        module: &Module
    ) -> io::Result<()> {
        write!(w, "if cond(&")?;
        generate_expression(w, tabs, stack_len, &if_expr.cond, module)?;
        writeln!(w, ") {{")?;
        generate_block(w, tabs + 1, stack_len, &if_expr.true_block, module)?;
        writeln!(w, "")?;
        generate_tabs(w, tabs)?;
        write!(w, "}}")?;
        for (else_if_cond, else_if_block) in if_expr.else_if_conds.iter()
            .zip(if_expr.else_if_blocks.iter())
        {
            write!(w, " else if cond(&")?;
            generate_expression(w, tabs + 1, stack_len, else_if_cond, module)?;
            writeln!(w, ") {{")?;
            generate_block(w, tabs + 1, stack_len, &else_if_block, module)?;
            writeln!(w, "")?;
            generate_tabs(w, tabs)?;
            write!(w, "}}")?;
        }
        if let Some(ref else_block) = if_expr.else_block {
            writeln!(w, " else {{")?;
            generate_block(w, tabs + 1, stack_len, &else_block, module)?;
            writeln!(w, "")?;
            generate_tabs(w, tabs)?;
            write!(w, "}}")?;
        }
        Ok(())
    }

    fn generate_unop<W: Write>(
        w: &mut W,
        tabs: u16,
        stack_len: usize,
        unop_expr: &UnOpExpression,
        module: &Module
    ) -> io::Result<()> {
        use dyon::ast::UnOp as U;

        match unop_expr.op {
            U::Not => write!(w, "unop::not")?,
            U::Neg => write!(w, "unop::neg")?,
        }
        write!(w, "(&")?;
        generate_expression(w, tabs, stack_len, &unop_expr.expr, module)?;
        write!(w, ")")?;
        Ok(())
    }

    fn generate_break<W: Write>(
        w: &mut W,
        _tabs: u16,
        _stack_len: usize,
        br: &Break
    ) -> io::Result<()> {
        if let Some(ref label) = br.label {
            write!(w, "break '{}", label)?;
        } else {
            write!(w, "break")?;
        }
        Ok(())
    }

    fn generate_continue<W: Write>(
        w: &mut W,
        _tabs: u16,
        _stack_len: usize,
        c: &Continue
    ) -> io::Result<()> {
        if let Some(ref label) = c.label {
            write!(w, "continue '{}", label)?;
        } else {
            write!(w, "continue")?;
        }
        Ok(())
    }

    /// Generates code that evaluates to a Dyon variable.
    fn generate_variable<W: Write>(
        w: &mut W,
        tabs: u16,
        stack_len: usize,
        expr: &Expression,
        module: &Module
    ) -> io::Result<()> {
        write!(w, "variable(&")?;
        generate_expression(w, tabs, stack_len, expr, module)?;
        write!(w, ")")?;
        Ok(())
    }

    fn generate_object<W: Write>(
        w: &mut W,
        tabs: u16,
        stack_len: usize,
        obj: &Object,
        module: &Module
    ) -> io::Result<()> {
        writeln!(w, "{{")?;
        let id = stack_len;
        generate_tabs(w, tabs + 1)?;
        writeln!(w, "let mut _{}: HashMap<Arc<String>, Variable> = HashMap::new();", id)?;
        for &(ref key, ref value) in &obj.key_values {
            generate_tabs(w, tabs + 1)?;
            write!(w, "_{}.insert(Arc::new({:?}.into()), ", id, key)?;
            generate_variable(w, tabs + 1, stack_len, value, module)?;
            writeln!(w, ");")?;
        }
        generate_tabs(w, tabs + 1)?;
        writeln!(w, "Arc::new(_{})", id)?;
        generate_tabs(w, tabs)?;
        write!(w, "}}")?;
        Ok(())
    }

    fn generate_call_closure<W: Write>(
        w: &mut W,
        tabs: u16,
        stack_len: usize,
        call_closure: &CallClosure,
        module: &Module
    ) -> io::Result<()> {
        write!(w, "(")?;
        generate_item(w, tabs + 1, stack_len, &call_closure.item, module)?;
        write!(w, ")(")?;
        let n = call_closure.args.len();
        for (i, arg) in call_closure.args.iter().enumerate() {
            generate_expression(w, tabs + 1, stack_len, arg, module)?;
            if (i + 1) < n {
                write!(w, ", ")?;
            }
        }
        write!(w, ")")?;
        Ok(())
    }

    fn generate_closure<W: Write>(
        w: &mut W,
        tabs: u16,
        stack_len: usize,
        closure: &Closure,
        module: &Module
    ) -> io::Result<()> {
        write!(w, "|")?;
        let n = closure.args.len();
        for (i, _) in closure.args.iter().enumerate() {
            write!(w, "_{}", stack_len + i)?;
            if (i + 1) < n {
                write!(w, ", ")?;
            }
        }
        writeln!(w, "| {{")?;
        generate_tabs(w, tabs + 1)?;
        generate_expression(w, tabs + 1, stack_len + n, &closure.expr, module)?;
        writeln!(w, "")?;
        generate_tabs(w, tabs)?;
        write!(w, "}}")?;
        Ok(())
    }

    fn generate_expression<W: Write>(
        w: &mut W,
        tabs: u16,
        stack_len: usize,
        exp: &Expression,
        module: &Module,
    ) -> io::Result<()> {
        use dyon::ast::Expression as E;

        match *exp {
            E::Call(ref call) => generate_call(w, tabs, stack_len, call, module)?,
            E::Text(ref text) => generate_text(w, text)?,
            E::For(ref for_expr) => generate_for(w, tabs, stack_len, for_expr, module)?,
            E::ForN(ref for_n) => generate_for_n(w, tabs, stack_len, for_n, module)?,
            E::Sum(ref sum) => generate_sum(w, tabs, stack_len, sum, module)?,
            E::Prod(ref prod) => generate_prod(w, tabs, stack_len, prod, module)?,
            E::Number(ref number) => generate_number(w, number)?,
            E::Bool(ref b) => generate_bool(w, b)?,
            E::Item(ref item) => generate_item(w, tabs, stack_len, item, module)?,
            E::BinOp(ref binop) => generate_binop(w, tabs, stack_len, binop, module)?,
            E::Vec4(ref vec4) => generate_vec4(w, tabs, stack_len, vec4, module)?,
            E::Array(ref array) => generate_array(w, tabs, stack_len, array, module)?,
            E::Assign(ref assign) => generate_assign(w, tabs, stack_len, assign, module)?,
            E::All(ref for_n) => generate_all_n(w, tabs, stack_len, for_n, module)?,
            E::Any(ref for_n) => generate_any_n(w, tabs, stack_len, for_n, module)?,
            E::Compare(ref compare) => generate_compare(w, tabs, stack_len, compare, module)?,
            E::If(ref if_expr) => generate_if(w, tabs, stack_len, if_expr, module)?,
            E::UnOp(ref unop) => generate_unop(w, tabs, stack_len, unop, module)?,
            E::Max(ref for_n) => generate_max_n(w, tabs, stack_len, for_n, module)?,
            E::Min(ref for_n) => generate_min_n(w, tabs, stack_len, for_n, module)?,
            E::Sift(ref for_n) => generate_sift_n(w, tabs, stack_len, for_n, module)?,
            E::Block(ref block) => {
                writeln!(w, "{{")?;
                generate_block(w, tabs + 1, stack_len, block, module)?;
                generate_tabs(w, tabs)?;
                write!(w, "}}")?;
            }
            E::Break(ref br) => generate_break(w, tabs, stack_len, br)?,
            E::Continue(ref c) => generate_continue(w, tabs, stack_len, c)?,
            E::Object(ref obj) => generate_object(w, tabs, stack_len, obj, module)?,
            E::Return(ref expr) => {
                write!(w, "return ")?;
                generate_expression(w, tabs + 1, stack_len, expr, module)?;
            }
            E::Closure(ref closure) => generate_closure(w, tabs + 1, stack_len, closure, module)?,
            E::CallClosure(ref call_closure) => generate_call_closure(w, tabs + 1, stack_len,
                                                                      call_closure, module)?,
            ref x => unimplemented!("{:?}", x),
        }

        Ok(())
    }

    fn generate_block<W: Write>(
        w: &mut W,
        tabs: u16,
        stack_len: usize,
        block: &Block,
        module: &Module,
    ) -> io::Result<()> {
        let n = block.expressions.len();
        let mut offset = 0;
        for (i, exp) in block.expressions.iter().enumerate() {
            generate_tabs(w, tabs)?;
            generate_expression(w, tabs, stack_len + offset, exp, module)?;
            if (i + 1) != n {
                writeln!(w, ";")?;
            }
            if let &Expression::Assign(ref assign) = exp {
                if let Expression::Item(ref item) = assign.left {
                    if item.ids.len() == 0 {
                        offset += 1;
                    }
                }
            }
        }

        Ok(())
    }

    fn generate_type<W: Write>(
        w: &mut W,
        ty: &Type
    ) -> io::Result<()> {
        match *ty {
            Type::F64 => write!(w, "f64")?,
            Type::Bool => write!(w, "bool")?,
            Type::Vec4 => write!(w, "[f32; 4]")?,
            Type::Text => write!(w, "str")?,
            Type::Array(ref ty) => {
                write!(w, "Vec<")?;
                if let Type::Text = **ty {
                    write!(w, "&")?;
                }
                generate_type(w, ty)?;
                write!(w, ">")?;
            }
            Type::Secret(ref inner_ty) => {
                write!(w, "Secret<")?;
                generate_type(w, inner_ty)?;
                write!(w, ", f64>")?;
            }
            Type::Closure(ref dfn) => {
                write!(w, "Fn(")?;
                let n = dfn.tys.len();
                for (i, ty) in dfn.tys.iter().enumerate() {
                    generate_type(w, ty)?;
                    if (i + 1) < n {
                        write!(w, ", ")?;
                    }
                }
                write!(w, ")")?;
                if let Type::Void = dfn.ret {}
                else {
                    write!(w, " -> ")?;
                    generate_type(w, &dfn.ret)?;
                }
            }
            _ => {}
        }
        Ok(())
    }

    writeln!(w, "#![allow(unused_imports)]")?;
    writeln!(w, "#![allow(unreachable_code)]")?;
    writeln!(w, "")?;
    writeln!(w, "extern crate dyon;")?;
    writeln!(w, "extern crate dyon_to_rust;")?;
    writeln!(w, "")?;
    writeln!(w, "use std::sync::Arc;")?;
    writeln!(w, "use std::collections::HashMap;")?;
    writeln!(w, "")?;
    writeln!(w, "use dyon::{{Variable, Object}};")?;
    writeln!(w, "use dyon_to_rust::intrinsics::*;")?;
    writeln!(w, "use dyon_to_rust::*;")?;
    writeln!(w, "")?;

    for f in &module.functions {
        if let Some(ind) = f.name.find('(') {
            write!(w, "fn {}(", &f.name[..ind])?;
        } else {
            write!(w, "fn {}(", f.name)?;
        }
        let mut offset = 0;
        let n = f.args.len();
        for (i, arg) in f.args.iter().enumerate() {
            if arg.mutable {
                write!(w, "mut _{}: &mut ", offset)?;
            } else {
                write!(w, "_{}: &", offset)?;
            }
            generate_type(w, &arg.ty)?;
            if (i + 1) != n {
                write!(w, ", ")?;
            }
            offset += 1;
        }
        if let Type::Void = f.ret {
            writeln!(w, ") {{")?;
        } else {
            write!(w, ") -> ")?;
            generate_type(w, &f.ret)?;
            writeln!(w, " {{")?;
        }
        generate_block(w, 1, offset, &f.block, module)?;
        if let Type::Void = f.ret {
            writeln!(w, ";")?;
            writeln!(w, "}}")?;
        } else {
            writeln!(w, "")?;
            writeln!(w, "}}")?;
        }
    }

    Ok(())
}

/// Generates code as a string from a Dyon module.
pub fn generate_code_string(module: &Module) -> String {
    let mut buf: Vec<u8> = vec![];
    generate_code(&mut buf, &module).unwrap();
    let txt = String::from_utf8(buf).unwrap();
    txt
}

#[cfg(test)]
mod tests {
    use super::*;
    use dyon::{load, Module};

    #[test]
    fn hello_world() {
        let mut module = Module::new();
        load("source/hello_world.dyon", &mut module).unwrap();
        let code = generate_code_string(&module);
        println!("{}", code);
        assert_eq!(code, include_str!("../source/hello_world.rs"));
    }

    #[test]
    fn count_to_ten() {
        let mut module = Module::new();
        load("source/count_to_ten.dyon", &mut module).unwrap();
        let code = generate_code_string(&module);
        println!("{}", code);
        assert_eq!(code, include_str!("../source/count_to_ten.rs"));
    }

    #[test]
    fn count_double() {
        let mut module = Module::new();
        load("source/count_double.dyon", &mut module).unwrap();
        let code = generate_code_string(&module);
        println!("{}", code);
        assert_eq!(code, include_str!("../source/count_double.rs"));
    }

    #[test]
    fn binops() {
        let mut module = Module::new();
        load("source/binops.dyon", &mut module).unwrap();
        let code = generate_code_string(&module);
        println!("{}", code);
        assert_eq!(code, include_str!("../source/binops.rs"));
    }

    #[test]
    fn vector4d() {
        let mut module = Module::new();
        load("source/vector4d.dyon", &mut module).unwrap();
        let code = generate_code_string(&module);
        println!("{}", code);
        assert_eq!(code, include_str!("../source/vector4d.rs"));
    }

    #[test]
    fn sum() {
        let mut module = Module::new();
        load("source/sum.dyon", &mut module).unwrap();
        let code = generate_code_string(&module);
        println!("{}", code);
        assert_eq!(code, include_str!("../source/sum.rs"));
    }

    #[test]
    fn prod() {
        let mut module = Module::new();
        load("source/prod.dyon", &mut module).unwrap();
        let code = generate_code_string(&module);
        println!("{}", code);
        assert_eq!(code, include_str!("../source/prod.rs"));
    }

    #[test]
    fn array() {
        let mut module = Module::new();
        load("source/array.dyon", &mut module).unwrap();
        let code = generate_code_string(&module);
        println!("{}", code);
        assert_eq!(code, include_str!("../source/array.rs"));
    }

    #[test]
    fn array2() {
        let mut module = Module::new();
        load("source/array2.dyon", &mut module).unwrap();
        let code = generate_code_string(&module);
        println!("{}", code);
        assert_eq!(code, include_str!("../source/array2.rs"));
    }

    #[test]
    fn array3() {
        let mut module = Module::new();
        load("source/array3.dyon", &mut module).unwrap();
        let code = generate_code_string(&module);
        println!("{}", code);
        assert_eq!(code, include_str!("../source/array3.rs"));
    }

    #[test]
    fn assign() {
        let mut module = Module::new();
        load("source/assign.dyon", &mut module).unwrap();
        let code = generate_code_string(&module);
        println!("{}", code);
        assert_eq!(code, include_str!("../source/assign.rs"));
    }

    #[test]
    fn index() {
        let mut module = Module::new();
        load("source/index.dyon", &mut module).unwrap();
        let code = generate_code_string(&module);
        println!("{}", code);
        assert_eq!(code, include_str!("../source/index.rs"));
    }

    #[test]
    fn triple_index() {
        let mut module = Module::new();
        load("source/triple_index.dyon", &mut module).unwrap();
        let code = generate_code_string(&module);
        println!("{}", code);
        assert_eq!(code, include_str!("../source/triple_index.rs"));
    }

    #[test]
    fn call() {
        let mut module = Module::new();
        load("source/call.dyon", &mut module).unwrap();
        let code = generate_code_string(&module);
        println!("{}", code);
        assert_eq!(code, include_str!("../source/call.rs"));
    }

    #[test]
    fn compare() {
        let mut module = Module::new();
        load("source/compare.dyon", &mut module).unwrap();
        let code = generate_code_string(&module);
        println!("{}", code);
        assert_eq!(code, include_str!("../source/compare.rs"));
    }

    #[test]
    fn if_expr() {
        let mut module = Module::new();
        load("source/if.dyon", &mut module).unwrap();
        let code = generate_code_string(&module);
        println!("{}", code);
        assert_eq!(code, include_str!("../source/if.rs"));
    }

    #[test]
    fn unop() {
        let mut module = Module::new();
        load("source/unop.dyon", &mut module).unwrap();
        let code = generate_code_string(&module);
        println!("{}", code);
        assert_eq!(code, include_str!("../source/unop.rs"));
    }

    #[test]
    fn secret() {
        let mut module = Module::new();
        load("source/secret.dyon", &mut module).unwrap();
        let code = generate_code_string(&module);
        println!("{}", code);
        assert_eq!(code, include_str!("../source/secret.rs"));
    }

    #[test]
    fn loop_() {
        let mut module = Module::new();
        load("source/loop.dyon", &mut module).unwrap();
        let code = generate_code_string(&module);
        println!("{}", code);
        assert_eq!(code, include_str!("../source/loop.rs"));
    }

    #[test]
    fn swizzle() {
        let mut module = Module::new();
        load("source/swizzle.dyon", &mut module).unwrap();
        let code = generate_code_string(&module);
        println!("{}", code);
        assert_eq!(code, include_str!("../source/swizzle.rs"));
    }

    #[test]
    fn return_() {
        let mut module = Module::new();
        load("source/return.dyon", &mut module).unwrap();
        let code = generate_code_string(&module);
        println!("{}", code);
        assert_eq!(code, include_str!("../source/return.rs"));
    }

    #[test]
    fn mutate() {
        let mut module = Module::new();
        load("source/mutate.dyon", &mut module).unwrap();
        let code = generate_code_string(&module);
        println!("{}", code);
        assert_eq!(code, include_str!("../source/mutate.rs"));
    }

    #[test]
    fn test() {
        use std::fs::File;
        use std::io::Write;

        let mut module = Module::new();
        load("source/test.dyon", &mut module).unwrap();
        let code = generate_code_string(&module);
        println!("{}", code);

        let run = true;
        if run {
            let mut file = File::create("source/test.rs").unwrap();
            write!(file, "{}", code).unwrap();
        } else {
            assert_eq!(code, include_str!("../source/test.rs"));
        }
    }
}
