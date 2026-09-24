// SPDX-License-Identifier: MPL-2.0
//! Creusot proof obligations for @@PROJECT_NAME@@.
//!
//! This crate contains no code of its own. It `include!`s the parent crate's
//! `src/impl.rs`, and under `creusot-rustc` the `#[cfg(creusot)]` attributes
//! in that file come alive as Creusot specifications. The implementations
//! being verified are therefore the *real* ones, not a mirrored copy: there
//! is nothing here that can drift out of sync with `../src/`.
//!
//! Run `just proof` from the repository root to translate and discharge the
//! obligations. See `../README.adoc` for the toolchain setup.

include!("../../src/impl.rs");
