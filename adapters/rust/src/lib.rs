//! Idiomatic Rust SDK for Authfil.
//!
//! Unlike the other adapters, there's no FFI boundary to cross: this crate
//! wraps `authfil-core`'s ports in an ergonomic Rust API for apps (e.g.
//! Axum, Actix) that use Authfil directly. No security decisions are made
//! here.
