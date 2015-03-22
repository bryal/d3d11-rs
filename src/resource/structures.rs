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

//! Resource Structures provided by D3D11
//!
//! # References
//! [D3D11 Resource Structures, MSDN]
//! (https://msdn.microsoft.com/en-us/library/windows/desktop/ff476173(v=vs.85).aspx)

use libc::c_void;
use winapi::{ UINT, UINT8, UINT16, BOOL };
use dxgi::{ DXGI_FORMAT, DXGI_SAMPLE_DESC };

use resource::enumerations::{ D3D11_UAV_DIMENSION,
	D3D11_DSV_DIMENSION,
	D3D11_RTV_DIMENSION,
	D3D11_SRV_DIMENSION,
	D3D11_USAGE };

#[repr(C)] pub struct D3D11_BUFFER_DESC {
	ByteWidth: UINT,
	Usage: D3D11_USAGE,
	BindFlags: UINT,
	CPUAccessFlags: UINT,
	MiscFlags: UINT,
	StructureByteStride: UINT,
}
#[repr(C)] pub struct D3D11_BUFFER_RTV {
	union_0: UINT,
	union_1: UINT,
}
#[repr(C)] pub struct D3D11_BUFFER_SRV {
	union_0: UINT,
	union_1: UINT,
}
#[repr(C)] pub struct D3D11_BUFFER_UAV {
	FirstElement: UINT,
	NumElements: UINT,
	Flags: UINT,
}
#[repr(C)] pub struct D3D11_BUFFEREX_SRV {
	FirstElement: UINT,
	NumElements: UINT,
	Flags: UINT,
}
#[repr(C)] pub struct D3D11_DEPTH_STENCIL_VIEW_DESC {
	Format: DXGI_FORMAT,
	ViewDimension: D3D11_DSV_DIMENSION,
	Flags: UINT,
	union_0: D3D11_TEX1D_ARRAY_DSV,
}
#[repr(C)] pub struct D3D11_MAPPED_SUBRESOURCE {
	data: *mut c_void,
	RowPitch: UINT,
	DepthPitch: UINT,
}
#[repr(C)] pub struct D3D11_PACKED_MIP_DESC {
	NumStandardMips: UINT8,
	NumPackedMips: UINT8,
	NumTilesForPackedMips: UINT,
	StartTileIndexInOverallResource: UINT,
}
#[repr(C)] pub struct D3D11_RENDER_TARGET_VIEW_DESC {
	Format: DXGI_FORMAT,
	ViewDimension: D3D11_RTV_DIMENSION,
	union_0: D3D11_TEX1D_ARRAY_RTV,
}
#[repr(C)] pub struct D3D11_SHADER_RESOURCE_VIEW_DESC {
	Format: DXGI_FORMAT,
	ViewDimension: D3D11_SRV_DIMENSION,
	union_0: D3D11_TEX1D_ARRAY_SRV,
}
#[repr(C)] pub struct D3D11_SUBRESOURCE_DATA {
	pSysMem: *const c_void,
	SysMemPitch: UINT,
	SysMemSlicePitch: UINT,
}
#[repr(C)] pub struct D3D11_SUBRESOURCE_TILING {
	WidthInTiles: UINT,
	HeightInTiles: UINT16,
	DepthInTiles: UINT16,
	StartTileIndexInOverallResource: UINT,
}
#[repr(C)] pub struct D3D11_TEX1D_ARRAY_DSV {
	MipSlice: UINT,
	FirstArraySlice: UINT,
	ArraySize: UINT,
}
#[repr(C)] pub struct D3D11_TEX1D_ARRAY_RTV {
	MipSlice: UINT,
	FirstArraySlice: UINT,
	ArraySize: UINT,
}
#[repr(C)] pub struct D3D11_TEX1D_ARRAY_SRV {
	MostDetailedMip: UINT,
	MipLevels: UINT,
	FirstArraySlice: UINT,
	ArraySize: UINT,
}
#[repr(C)] pub struct D3D11_TEX1D_ARRAY_UAV {
	MipSlice: UINT,
	FirstArraySlice: UINT,
	ArraySize: UINT,
}
#[repr(C)] pub struct D3D11_TEX1D_DSV {
	MipSlice: UINT,
}
#[repr(C)] pub struct D3D11_TEX1D_RTV {
	MipSlice: UINT,
}
#[repr(C)] pub struct D3D11_TEX1D_SRV {
	MostDetailedMip: UINT,
	MipLevels: UINT,
}
#[repr(C)] pub struct D3D11_TEX1D_UAV {
	MipSlice: UINT,
}
#[repr(C)] pub struct D3D11_TEX2D_ARRAY_DSV {
	MipSlice: UINT,
	FirstArraySlice: UINT,
	ArraySize: UINT,
}
#[repr(C)] pub struct D3D11_TEX2D_ARRAY_RTV {
	MipSlice: UINT,
	FirstArraySlice: UINT,
	ArraySize: UINT,
}
#[repr(C)] pub struct D3D11_TEX2D_ARRAY_SRV {
	MostDetailedMip: UINT,
	MipLevels: UINT,
	FirstArraySlice: UINT,
	ArraySize: UINT,
}
#[repr(C)] pub struct D3D11_TEX2D_ARRAY_UAV {
	MipSlice: UINT,
	FirstArraySlice: UINT,
	ArraySize: UINT,
}
#[repr(C)] pub struct D3D11_TEX2D_DSV {
	MipSlice: UINT,
}
#[repr(C)] pub struct D3D11_TEX2D_RTV {
	MipSlice: UINT,
}
#[repr(C)] pub struct D3D11_TEX2D_UAV {
	MipSlice: UINT,
}
#[repr(C)] pub struct D3D11_TEX2DMS_ARRAY_DSV {
	FirstArraySlice: UINT,
	ArraySize: UINT,
}
#[repr(C)] pub struct D3D11_TEX2DMS_ARRAY_RTV {
	FirstArraySlice: UINT,
	ArraySize: UINT,
}
#[repr(C)] pub struct D3D11_TEX2DMS_ARRAY_SRV {
	FirstArraySlice: UINT,
	ArraySize: UINT,
}
#[repr(C)] pub struct D3D11_TEX3D_RTV {
	MipSlice: UINT,
	FirstWSlice: UINT,
	WSize: UINT,
}
#[repr(C)] pub struct D3D11_TEX3D_SRV {
	MostDetailedMip: UINT,
	MipLevels: UINT,
}
#[repr(C)] pub struct D3D11_TEX3D_UAV {
	MipSlice: UINT,
	FirstWSlice: UINT,
	WSize: UINT,
}
#[repr(C)] pub struct D3D11_TEXCUBE_ARRAY_SRV {
	MostDetailedMip: UINT,
	MipLevels: UINT,
	First2DArrayFace: UINT,
	NumCubes: UINT,
}
#[repr(C)] pub struct D3D11_TEXCUBE_SRV {
	MostDetailedMip: UINT,
	MipLevels: UINT,
}
#[repr(C)] pub struct D3D11_TEXTURE1D_DESC {
	Width: UINT,
	MipLevels: UINT,
	ArraySize: UINT,
	Format: DXGI_FORMAT,
	Usage: D3D11_USAGE,
	BindFlags: UINT,
	CPUAccessFlags: UINT,
	MiscFlags: UINT,
}
#[repr(C)] pub struct D3D11_TEXTURE2D_DESC {
	Width: UINT,
	Height: UINT,
	MipLevels: UINT,
	ArraySize: UINT,
	Format: DXGI_FORMAT,
	SampleDesc: DXGI_SAMPLE_DESC,
	Usage: D3D11_USAGE,
	BindFlags: UINT,
	CPUAccessFlags: UINT,
	MiscFlags: UINT,
}
#[repr(C)] pub struct D3D11_TEXTURE3D_DESC {
	Width: UINT,
	Height: UINT,
	Depth: UINT,
	MipLevels: UINT,
	Format: DXGI_FORMAT,
	Usage: D3D11_USAGE,
	BindFlags: UINT,
	CPUAccessFlags: UINT,
	MiscFlags: UINT,
}
#[repr(C)] pub struct D3D11_TILE_REGION_SIZE {
	NumTiles: UINT,
	bUseBox: BOOL,
	Width: UINT,
	Height: UINT16,
	Depth: UINT16,
}
#[repr(C)] pub struct D3D11_TILED_RESOURCE_COORDINATE {
	X: UINT,
	Y: UINT,
	Z: UINT,
	Subresource: UINT,
}
#[repr(C)] pub struct D3D11_TILE_SHAPE {
	WidthInTexels: UINT,
	HeightInTexels: UINT,
	DepthInTexels: UINT,
}
#[repr(C)] pub struct D3D11_UNORDERED_ACCESS_VIEW_DESC {
	Format: DXGI_FORMAT,
	ViewDimension: D3D11_UAV_DIMENSION,
	union_0: D3D11_BUFFER_UAV,
}