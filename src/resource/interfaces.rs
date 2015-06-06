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

//! Interfaces for the two basic types of resources: buffers and textures
//!
//! # References
//! [Resource Interfaces, MSDN]
//! (https://msdn.microsoft.com/en-us/library/windows/desktop/ff476172(v=vs.85).aspx)

#![allow(non_snake_case)]

use winapi::minwindef::*;
use winapi::GUID;
use dxgi::{ IUnknown, QueryIID, COMInterface };

use constants::*;
use core::interfaces::{ ID3D11DeviceChild, ID3D11DeviceChildVtbl };
use resource::enumerations::D3D11_RESOURCE_DIMENSION;
use resource::structures::{ D3D11_UNORDERED_ACCESS_VIEW_DESC,
	D3D11_SHADER_RESOURCE_VIEW_DESC,
	D3D11_RENDER_TARGET_VIEW_DESC,
	D3D11_DEPTH_STENCIL_VIEW_DESC,
	D3D11_BUFFER_DESC,
	D3D11_TEXTURE1D_DESC,
	D3D11_TEXTURE2D_DESC,
	D3D11_TEXTURE3D_DESC };

com_interface!{ ID3D11Resource(ID3D11ResourceVtbl): ID3D11DeviceChild(ID3D11DeviceChildVtbl) {
	fn GetType(&mut self, pResourceDimension: *mut D3D11_RESOURCE_DIMENSION) -> (),
	fn SetEvictionPriority(&mut self, EvictionPriority: UINT) -> (),
	fn GetEvictionPriority(&mut self) -> UINT
}}
com_interface!{ ID3D11Buffer(ID3D11BufferVtbl): ID3D11Resource(ID3D11ResourceVtbl) {
	fn GetDesc(&mut self, pDesc: *mut D3D11_BUFFER_DESC) -> ()
}}
com_interface!{ ID3D11Texture1D(ID3D11Texture1DVtbl): ID3D11Resource(ID3D11ResourceVtbl) {
	fn GetDesc(&mut self, pDesc: *mut D3D11_TEXTURE1D_DESC) -> ()
}}
com_interface!{ ID3D11Texture2D(ID3D11Texture2DVtbl): ID3D11Resource(ID3D11ResourceVtbl) {
	fn GetDesc(&mut self, pDesc: *mut D3D11_TEXTURE2D_DESC) -> ()
}}
com_interface!{ ID3D11Texture3D(ID3D11Texture3DVtbl): ID3D11Resource(ID3D11ResourceVtbl) {
	fn GetDesc(&mut self, pDesc: *mut D3D11_TEXTURE3D_DESC) -> ()
}}
com_interface!{ ID3D11View(ID3D11ViewVtbl): ID3D11DeviceChild(ID3D11DeviceChildVtbl) {
	fn GetResource(&mut self, ppResource: *mut *mut ID3D11Resource) -> ()
}}
com_interface!{ ID3D11DepthStencilView(ID3D11DepthStencilViewVtbl): ID3D11View(ID3D11ViewVtbl) {
	fn GetDesc(&mut self, pDesc: *mut D3D11_DEPTH_STENCIL_VIEW_DESC) -> ()
}}
com_interface!{ ID3D11RenderTargetView(ID3D11RenderTargetViewVtbl): ID3D11View(ID3D11ViewVtbl) {
	fn GetDesc(&mut self, pDesc: *mut D3D11_RENDER_TARGET_VIEW_DESC) -> ()
}}
com_interface!{ ID3D11ShaderResourceView(ID3D11ShaderResourceViewVtbl): ID3D11View(ID3D11ViewVtbl) {
	fn GetDesc(&mut self, pDesc: *mut D3D11_SHADER_RESOURCE_VIEW_DESC) -> ()
}}
com_interface!{ ID3D11UnorderedAccessView(ID3D11UnorderedAccessViewVtbl): ID3D11View(ID3D11ViewVtbl) {
	fn GetDesc(&mut self, pDesc: *mut D3D11_UNORDERED_ACCESS_VIEW_DESC) -> ()
}}

impl QueryIID for ID3D11Buffer { fn iid() -> GUID { IID_ID3D11Buffer } }
impl QueryIID for ID3D11Resource { fn iid() -> GUID { IID_ID3D11Resource } }
impl QueryIID for ID3D11Texture1D { fn iid() -> GUID { IID_ID3D11Texture1D } }
impl QueryIID for ID3D11Texture2D { fn iid() -> GUID { IID_ID3D11Texture2D } }
impl QueryIID for ID3D11Texture3D { fn iid() -> GUID { IID_ID3D11Texture3D } }
impl QueryIID for ID3D11DepthStencilView { fn iid() -> GUID { IID_ID3D11DepthStencilView } }
impl QueryIID for ID3D11RenderTargetView { fn iid() -> GUID { IID_ID3D11RenderTargetView } }
impl QueryIID for ID3D11ShaderResourceView { fn iid() -> GUID { IID_ID3D11ShaderResourceView } }
impl QueryIID for ID3D11UnorderedAccessView { fn iid() -> GUID { IID_ID3D11UnorderedAccessView } }
impl QueryIID for ID3D11View { fn iid() -> GUID { IID_ID3D11View } }
