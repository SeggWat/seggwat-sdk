//! SeggWat public board API types, plus (feature `client`) a cached
//! server-side client and end-user token signing.
//!
//! The framework crates (`seggwat-topcoat`, `seggwat-dioxus`) render these
//! types. Your server fetches with [`Seggwat`] and passes the result in; vote
//! buttons then talk to SeggWat from the visitor's browser with the token
//! [`Seggwat::voting`] signed for them.

mod types;
pub mod ui;

pub use types::*;

#[cfg(feature = "client")]
mod client;
#[cfg(feature = "client")]
pub use client::{Error, Seggwat, Viewer};
