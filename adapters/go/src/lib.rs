//! Go bindings for Authloom.
//!
//! A thin translation layer over `authloom-core`: validate what crosses the
//! FFI boundary, call the core, and hand effects back to Go. No security
//! decisions are made here.
