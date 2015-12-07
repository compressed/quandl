#![deny(missing_docs)]
#![cfg_attr(all(test, feature = "nightly"), feature(test))]
#![cfg_attr(test, deny(warnings))]

//! Library to fetch data from the [Quandl v3 API](https://www.quandl.com/docs/api)
//! for financial and economic datasets.

extern crate curl;
extern crate serde_json;
extern crate url;
extern crate hyper;
#[macro_use] extern crate quick_error;
extern crate chrono;

pub use quandl_request::*;
pub use error::{Error, Result};

/// Handles building and sending requests to Quandl
pub mod quandl_request;
/// Errors
pub mod error;