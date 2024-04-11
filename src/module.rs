// module.rs, Examples from the book "Rust Programming Language"
// Copyright (C) 2022-2024, Savio Sena <savio.sena@gmail.com>

// module `module`.
pub mod submodule;

pub use submodule::*;

pub fn func() {
    println!("create::module::func()");
}
