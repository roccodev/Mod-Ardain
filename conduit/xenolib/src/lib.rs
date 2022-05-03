//! **THIS CRATE IS FOR INTERNAL USE ONLY!**
//!
//! This is a **very raw** wrapper over the functions exported by the various Xenoblade games.
//! None of the functions are guaranteed to exist at any given time.
//!
//! For external use, please use the [`conduit`](../conduit/index.html) crate instead.

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

pub use root::*;
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
