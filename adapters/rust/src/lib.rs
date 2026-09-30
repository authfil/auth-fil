//! Idiomatic Rust SDK for Authloom.
//!
//! Unlike the other adapters, there's no FFI boundary to cross: this crate
//! wraps `authloom-core`'s ports in an ergonomic Rust API for apps (e.g.
//! Axum, Actix) that use Authloom directly. No security decisions are made
//! here.
