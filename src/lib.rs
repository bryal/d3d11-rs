//! Rust bindings for D3D11

#![cfg(windows)]

extern crate winapi;
#[macro_use]
extern crate dxgi_win as dxgi;

pub use core::enumerations::*;
pub use core::structures::*;
pub use core::functions::*;
pub use core::interfaces::*;

pub use common_version::enumerations::*;
pub use common_version::structures::*;
pub use common_version::interfaces::*;

pub use resource::enumerations::*;
pub use resource::structures::*;
pub use resource::interfaces::*;
pub use resource::functions::*;

pub use shader::enumerations::*;
pub use shader::structures::*;
pub use shader::interfaces::*;

pub use constants::*;

pub mod core;
pub mod common_version;
pub mod resource;
pub mod shader;
pub mod constants;

pub mod interfaces {
    pub use core::interfaces::*;
    pub use common_version::interfaces::*;
    pub use resource::interfaces::*;
    pub use shader::interfaces::*;
}

#[cfg(test)]
mod test;
