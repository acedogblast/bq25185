//! A basic and simple BQ25185 battey charging IC driver for Embedded Rust.
//!
//! This crate provide a basic interface to read the status pins and optionally set the charge enable pin of the BQ25185 standalone linear battery charger IC from Texas Insturments.
//! See datasheet for the proper usage of this charger IC.

#![cfg_attr(not(test), no_std)]

pub mod driver;
pub mod error;

pub use driver::{Bq25185, Status};
pub use error::Bq25185Error;
