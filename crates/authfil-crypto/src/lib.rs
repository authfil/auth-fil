//! Cryptographic building blocks for Authfil.
//!
//! Password hashing, secure token generation, constant-time comparison and
//! secret handling. Nothing here is novel cryptography: this crate wires
//! vetted crates together with safe defaults.

#![forbid(unsafe_code)]
