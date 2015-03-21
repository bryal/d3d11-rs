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

//! Functions provided by D3D11
//!
//! # References
//! [D3D11 Functions, MSDN]
//! (https://msdn.microsoft.com/en-us/library/windows/desktop/ff476153(v=vs.85).aspx)

use winapi::{ UINT, HRESULT, HMODULE };
use dxgi::{ IDXGIAdapter, IDXGISwapChain };

use common_version::enumerations::*;

#[link(name="d3d11")]
extern "C" {
	pub fn D3D11CreateDevice(adapter: *mut IDXGIAdapter,
		driver_type: D3D_DRIVER_TYPE,
		software: HMODULE,
		flags: UINT,
		feature_levels: *const D3D_FEATURE_LEVEL,
		num_feature_levels: UINT,
		sdk_version: UINT,
		device: *mut *mut ID3D11Device,
		feature_level: *mut D3D_FEATURE_LEVEL,
		immidiate_context: *mut *mut ID3D11DeviceContext) -> HRESULT;

	pub fn D3D11CreateDeviceAndSwapChain(adapter: *mut IDXGIAdapter,
		driver_type: D3D_DRIVER_TYPE,
		software: HMODULE,
		flags: UINT,
		feature_levels: *const D3D_FEATURE_LEVEL,
		num_feature_levels: UINT,
		sdk_version: UINT,
		swapchain_desc: *const DXGI_SWAP_CHAIN_DESC,
		swapchain: *mut *mut IDXGISwapChain,
		device: *mut *mut ID3D11Device,
		feature_level: *mut D3D_FEATURE_LEVEL,
		immidiate_context: *mut *mut ID3D11DeviceContext) -> HRESULT;
}