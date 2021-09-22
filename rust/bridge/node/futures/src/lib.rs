//
// Copyright 2020-2021 Signal Messenger, LLC.
// SPDX-License-Identifier: AGPL-3.0-only
//

//! Allows `async` blocks to be used to wait on JavaScript futures using [Neon][].
//!
//! Neon provides a way to expose *synchronous* JavaScript functions from Rust.
//! This means that if Rust wants to wait for the result of a JavaScript promise,
//! it can at best return a callback to continue its work when the promise settles.
//! This does not naturally compose with Rust's `async`, which works in terms of [Futures](trait@std::future::Future).
//!
//! This crate provides functionality for (1) wrapping JavaScript futures so they can be awaited on in Rust,
//! and (2) running Rust futures on the JavaScript event queue. It does so by resuming execution
//! of the Rust future whenever an awaited JavaScript promise is settled.
//!
//! To get started, look at the [ChannelEx] methods and the [JsFuture::from_promise] method.
//!
//! [Neon]: https://neon-bindings.com/

#![warn(missing_docs)]
#![warn(clippy::unwrap_used)]

mod executor;
pub use executor::{ChannelEx, ContextEx};

mod exception;
pub use exception::PersistentException;

mod future;
pub use future::{JsFuture, JsFutureBuilder};

mod result;
pub use result::JsPromiseResult;

mod util;
pub use util::call_method;
