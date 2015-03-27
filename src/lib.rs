// The MIT License (MIT)
//
// Copyright (c) 2015 Johan Johansson
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
// THE SOFTWARE.

//! Rust bindings for D3D11

#![cfg(windows)]
#![feature(libc)]

extern crate libc;
extern crate winapi;
#[macro_use]
extern crate dxgi;

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