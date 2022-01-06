use extendr_api::prelude::*;
use nalgebra::{DMatrix, DVector, Scalar};

use std::io::BufReader;
use std::fs::File;
use std::str::FromStr;

/// Return string `"Hello world!"` to R.
/// @export
#[extendr]
fn hello_world() -> &'static str {
    "Hello world!"
}

/// return float add to R.
/// @export
#[extendr]
fn add(x:f64, y:f64) -> f64{
  x + y
}

/// linear regression to R
/// @export
#[extendr]
fn rust_lm(X, y){
  ///頑張って書く
  println!("hello world!")
}


/// return vector add to R.
/// @export
#[extendr]
fn mult(x: Vec<i32>, y: i32) -> Vec<i32> {
    x
     .iter()
     .map(|n| n * y)
     .collect::<Vec<_>>()
}

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod RextendRProto;
    fn hello_world;
    fn add;
    fn mult;
}

