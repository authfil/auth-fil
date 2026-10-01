//! Sans-IO core of Authfil.
//!
//! Every security decision lives here. The core never touches the network,
//! a database, a clock or a random source directly: callers feed it inputs
//! and it returns effects for the adapter to carry out. This keeps the rules
//! identical across every language adapter and makes them testable as pure
//! state transitions.

#![forbid(unsafe_code)]
