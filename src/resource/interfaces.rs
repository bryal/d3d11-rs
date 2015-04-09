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
use winapi::{ REFGUID, HRESULT, REFIID, c_void };
use dxgi::{ IUnknown, IUnknownT };

use core::interfaces::{ ID3D11DeviceChildT, ID3D11Device };
use resource::enumerations::D3D11_RESOURCE_DIMENSION;
use resource::structures::{ D3D11_UNORDERED_ACCESS_VIEW_DESC,
	D3D11_SHADER_RESOURCE_VIEW_DESC,
	D3D11_RENDER_TARGET_VIEW_DESC,
	D3D11_DEPTH_STENCIL_VIEW_DESC,
	D3D11_BUFFER_DESC,
	D3D11_TEXTURE1D_DESC,
	D3D11_TEXTURE2D_DESC,
	D3D11_TEXTURE3D_DESC };

#[repr(C)] pub struct ID3D11Buffer { pub vtable: *mut ID3D11BufferVtbl }
#[repr(C)] pub struct ID3D11Resource { pub vtable: *mut ID3D11ResourceVtbl }
#[repr(C)] pub struct ID3D11Texture1D { pub vtable: *mut ID3D11Texture1DVtbl }
#[repr(C)] pub struct ID3D11Texture2D { pub vtable: *mut ID3D11Texture2DVtbl }
#[repr(C)] pub struct ID3D11Texture3D { pub vtable: *mut ID3D11Texture3DVtbl }
#[repr(C)] pub struct ID3D11DepthStencilView { pub vtable: *mut ID3D11DepthStencilViewVtbl }
#[repr(C)] pub struct ID3D11RenderTargetView { pub vtable: *mut ID3D11RenderTargetViewVtbl }
#[repr(C)] pub struct ID3D11ShaderResourceView { pub vtable: *mut ID3D11ShaderResourceViewVtbl }
#[repr(C)] pub struct ID3D11UnorderedAccessView { pub vtable: *mut ID3D11UnorderedAccessViewVtbl }
#[repr(C)] pub struct ID3D11View { pub vtable: *mut ID3D11ViewVtbl }

c_vtable!{
	_ of (), trait IUnknownT {
		fn QueryInterface(riid: REFIID, object: *mut *mut c_void) -> HRESULT,
		fn AddRef() -> ULONG,
		fn Release() -> ULONG,
	} with heirs [
		_ of (), trait ID3D11DeviceChildT {
			fn GetDevice(device: *mut *mut ID3D11Device) -> (),
			fn GetPrivateData(guid: REFGUID, data_size: *mut UINT, data: *mut c_void) -> HRESULT,
			fn SetPrivateData(guid: REFGUID, data_size: UINT, data: *const c_void) -> HRESULT,
			fn SetPrivateDataInterface(guid: REFGUID, data: *const IUnknown) -> HRESULT,
		} with heirs [
			pub ID3D11ResourceVtbl of ID3D11Resource, pub trait ID3D11ResourceT {
				fn GetType(pResourceDimension: *mut D3D11_RESOURCE_DIMENSION) -> (),
				fn SetEvictionPriority(EvictionPriority: UINT) -> (),
				fn GetEvictionPriority() -> UINT,
			} with heirs [
				pub ID3D11BufferVtbl of ID3D11Buffer, pub trait ID3D11BufferT {
					fn GetDesc(pDesc: *mut D3D11_BUFFER_DESC) -> (),
				}
				pub ID3D11Texture1DVtbl of ID3D11Texture1D, pub trait ID3D11Texture1DT {
					fn GetDesc(pDesc: *mut D3D11_TEXTURE1D_DESC) -> (),
				}
				pub ID3D11Texture2DVtbl of ID3D11Texture2D, pub trait ID3D11Texture2DT {
					fn GetDesc(pDesc: *mut D3D11_TEXTURE2D_DESC) -> (),
				}
				pub ID3D11Texture3DVtbl of ID3D11Texture3D, pub trait ID3D11Texture3DT {
					fn GetDesc(pDesc: *mut D3D11_TEXTURE3D_DESC) -> (),
				}
			]
			pub ID3D11ViewVtbl of ID3D11View, pub trait ID3D11ViewT {
				fn GetResource(ppResource: *mut *mut ID3D11Resource) -> (),
			} with heirs [
				pub ID3D11DepthStencilViewVtbl of ID3D11DepthStencilView,
					pub trait ID3D11DepthStencilViewT
				{
					fn GetDesc(pDesc: *mut D3D11_DEPTH_STENCIL_VIEW_DESC) -> (),
				}
				pub ID3D11RenderTargetViewVtbl of ID3D11RenderTargetView,
					pub trait ID3D11RenderTargetViewT
				{
					fn GetDesc(pDesc: *mut D3D11_RENDER_TARGET_VIEW_DESC) -> (),
				}
				pub ID3D11ShaderResourceViewVtbl of ID3D11ShaderResourceView,
					pub trait ID3D11ShaderResourceViewT
				{
					fn GetDesc(pDesc: *mut D3D11_SHADER_RESOURCE_VIEW_DESC) -> (),
				}
				pub ID3D11UnorderedAccessViewVtbl of ID3D11UnorderedAccessView,
					pub trait ID3D11UnorderedAccessViewT
				{
					fn GetDesc(pDesc: *mut D3D11_UNORDERED_ACCESS_VIEW_DESC) -> (),
				}
			]
		]
	]
}