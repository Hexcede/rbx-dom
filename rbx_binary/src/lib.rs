//! Super early, unstable binary format (rbxm and rbxl) serializer and
//! deserializer for rbx-dom.
//!
//! Both the serializer and deserializer are functioning for limited property
//! types. `String` and `Bool` (from the `rbx_dom_weak` crate) are the only
//! supported values. Unrecognized values will be ignored when deserializing,
//! and cause a panic when serializing.

mod core;
mod deserializer;
mod serializer;
mod types;

pub use crate::{
    deserializer::{decode, DecodeError},
    serializer::{encode, EncodeError},
};
