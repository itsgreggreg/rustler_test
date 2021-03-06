#[macro_use]
extern crate rustler;
#[macro_use]
extern crate rustler_codegen;
#[macro_use]
extern crate lazy_static;

use rustler::{Encoder, Env, NifResult, Term};

mod atoms {
    rustler_atoms! {
        atom ok;
        atom error;
        //atom __true__ = "true";
        //atom __false__ = "false";
    }
}

rustler_export_nifs! {
    "Elixir.MyRustler",
    [("add", 2, add),
     ("my_panic", 1, my_panic),
    ],
    None
}

fn add<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let num1: i64 = try!(args[0].decode());
    let num2: i64 = try!(args[1].decode());

    Ok((atoms::ok(), num1 + num2).encode(env))
}

fn my_panic<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let arg: bool = args[0].decode()?;
    if arg == true {
        panic!();
        Ok((atoms::error()).encode(env))
    } else {
        Ok((atoms::ok()).encode(env))
    }
}
