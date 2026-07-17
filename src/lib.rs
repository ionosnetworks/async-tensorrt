#![recursion_limit = "256"]

pub mod builder;
pub mod engine;
pub mod error;
pub mod ffi;
pub mod optimization_profile;
pub mod runtime;

#[cfg(test)]
mod tests;

pub use builder::Builder;
pub use engine::{Engine, ExecutionContext};
pub use error::Error;
pub use ffi::builder_config::BuilderConfig;
pub use ffi::memory::HostBuffer;
pub use ffi::network::{NetworkDefinition, NetworkDefinitionCreationFlags, Tensor};
pub use ffi::parser::Parser;
pub use optimization_profile::OptimizationProfile;
pub use runtime::Runtime;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DataType {
    Float = 0,
    Half = 1,
    Int8 = 2,
    Int32 = 3,
    Bool = 4,
    Uint8 = 5,
    Fp8 = 6,
    Bf16 = 7,
    Int64 = 8,
    Int4 = 9,
}
