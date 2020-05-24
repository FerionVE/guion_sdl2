pub mod env;
pub mod stor;
//pub mod path;
pub mod valid;
//pub mod style;
pub mod ctx;
use super::*;
use guion::{id::standard::StdID, path::standard::SimplePath};
use env::SimpleEnv;
pub mod immediate_test;

pub type StandardPath = SimplePath<SimpleEnv,StdID>;