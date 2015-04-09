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

#![allow(non_snake_case)]

use winapi::minwindef::*;
use winapi::basetsd::*;
use winapi::c_void;
use dxgi::{ DXGI_FORMAT, DXGI_SAMPLE_DESC };

use resource::enumerations::*;

#[repr(C)] pub struct D3D11_BUFFER_DESC {
	pub ByteWidth: UINT,
	pub Usage: D3D11_USAGE,
	pub BindFlags: UINT,
	pub CPUAccessFlags: UINT,
	pub MiscFlags: UINT,
	pub StructureByteStride: UINT,
}
#[repr(C)] pub struct D3D11_BUFFER_RTV {
	pub union_0: UINT,
	pub union_1: UINT,
}
#[repr(C)] pub struct D3D11_BUFFER_SRV {
	pub union_0: UINT,
	pub union_1: UINT,
}
#[repr(C)] pub struct D3D11_BUFFER_UAV {
	pub FirstElement: UINT,
	pub NumElements: UINT,
	pub Flags: UINT,
}
#[repr(C)] pub struct D3D11_BUFFEREX_SRV {
	pub FirstElement: UINT,
	pub NumElements: UINT,
	pub Flags: UINT,
}
#[repr(C)] pub struct D3D11_DEPTH_STENCIL_VIEW_DESC {
	pub Format: DXGI_FORMAT,
	pub ViewDimension: D3D11_DSV_DIMENSION,
	pub Flags: UINT,
	pub union_0: D3D11_TEX1D_ARRAY_DSV,
}
#[repr(C)] pub struct D3D11_MAPPED_SUBRESOURCE {
	pub data: *mut c_void,
	pub RowPitch: UINT,
	pub DepthPitch: UINT,
}
#[repr(C)] pub struct D3D11_PACKED_MIP_DESC {
	pub NumStandardMips: UINT8,
	pub NumPackedMips: UINT8,
	pub NumTilesForPackedMips: UINT,
	pub StartTileIndexInOverallResource: UINT,
}
#[repr(C)] pub struct D3D11_RENDER_TARGET_VIEW_DESC {
	pub Format: DXGI_FORMAT,
	pub ViewDimension: D3D11_RTV_DIMENSION,
	pub union_0: D3D11_TEX1D_ARRAY_RTV,
}
#[repr(C)] pub struct D3D11_SHADER_RESOURCE_VIEW_DESC {
	pub Format: DXGI_FORMAT,
	pub ViewDimension: D3D11_SRV_DIMENSION,
	pub union_0: D3D11_TEX1D_ARRAY_SRV,
}
#[repr(C)] pub struct D3D11_SUBRESOURCE_DATA {
	pub pSysMem: *const c_void,
	pub SysMemPitch: UINT,
	pub SysMemSlicePitch: UINT,
}
#[repr(C)] pub struct D3D11_SUBRESOURCE_TILING {
	pub WidthInTiles: UINT,
	pub HeightInTiles: UINT16,
	pub DepthInTiles: UINT16,
	pub StartTileIndexInOverallResource: UINT,
}
#[repr(C)] pub struct D3D11_TEX1D_ARRAY_DSV {
	pub MipSlice: UINT,
	pub FirstArraySlice: UINT,
	pub ArraySize: UINT,
}
#[repr(C)] pub struct D3D11_TEX1D_ARRAY_RTV {
	pub MipSlice: UINT,
	pub FirstArraySlice: UINT,
	pub ArraySize: UINT,
}
#[repr(C)] pub struct D3D11_TEX1D_ARRAY_SRV {
	pub MostDetailedMip: UINT,
	pub MipLevels: UINT,
	pub FirstArraySlice: UINT,
	pub ArraySize: UINT,
}
#[repr(C)] pub struct D3D11_TEX1D_ARRAY_UAV {
	pub MipSlice: UINT,
	pub FirstArraySlice: UINT,
	pub ArraySize: UINT,
}
#[repr(C)] pub struct D3D11_TEX1D_DSV {
	pub MipSlice: UINT,
}
#[repr(C)] pub struct D3D11_TEX1D_RTV {
	pub MipSlice: UINT,
}
#[repr(C)] pub struct D3D11_TEX1D_SRV {
	pub MostDetailedMip: UINT,
	pub MipLevels: UINT,
}
#[repr(C)] pub struct D3D11_TEX1D_UAV {
	pub MipSlice: UINT,
}
#[repr(C)] pub struct D3D11_TEX2D_ARRAY_DSV {
	pub MipSlice: UINT,
	pub FirstArraySlice: UINT,
	pub ArraySize: UINT,
}
#[repr(C)] pub struct D3D11_TEX2D_ARRAY_RTV {
	pub MipSlice: UINT,
	pub FirstArraySlice: UINT,
	pub ArraySize: UINT,
}
#[repr(C)] pub struct D3D11_TEX2D_ARRAY_SRV {
	pub MostDetailedMip: UINT,
	pub MipLevels: UINT,
	pub FirstArraySlice: UINT,
	pub ArraySize: UINT,
}
#[repr(C)] pub struct D3D11_TEX2D_ARRAY_UAV {
	pub MipSlice: UINT,
	pub FirstArraySlice: UINT,
	pub ArraySize: UINT,
}
#[repr(C)] pub struct D3D11_TEX2D_DSV {
	pub MipSlice: UINT,
}
#[repr(C)] pub struct D3D11_TEX2D_RTV {
	pub MipSlice: UINT,
}
#[repr(C)] pub struct D3D11_TEX2D_UAV {
	pub MipSlice: UINT,
}
#[repr(C)] pub struct D3D11_TEX2DMS_ARRAY_DSV {
	pub FirstArraySlice: UINT,
	pub ArraySize: UINT,
}
#[repr(C)] pub struct D3D11_TEX2DMS_ARRAY_RTV {
	pub FirstArraySlice: UINT,
	pub ArraySize: UINT,
}
#[repr(C)] pub struct D3D11_TEX2DMS_ARRAY_SRV {
	pub FirstArraySlice: UINT,
	pub ArraySize: UINT,
}
#[repr(C)] pub struct D3D11_TEX3D_RTV {
	pub MipSlice: UINT,
	pub FirstWSlice: UINT,
	pub WSize: UINT,
}
#[repr(C)] pub struct D3D11_TEX3D_SRV {
	pub MostDetailedMip: UINT,
	pub MipLevels: UINT,
}
#[repr(C)] pub struct D3D11_TEX3D_UAV {
	pub MipSlice: UINT,
	pub FirstWSlice: UINT,
	pub WSize: UINT,
}
#[repr(C)] pub struct D3D11_TEXCUBE_ARRAY_SRV {
	pub MostDetailedMip: UINT,
	pub MipLevels: UINT,
	pub First2DArrayFace: UINT,
	pub NumCubes: UINT,
}
#[repr(C)] pub struct D3D11_TEXCUBE_SRV {
	pub MostDetailedMip: UINT,
	pub MipLevels: UINT,
}
#[repr(C)] pub struct D3D11_TEXTURE1D_DESC {
	pub Width: UINT,
	pub MipLevels: UINT,
	pub ArraySize: UINT,
	pub Format: DXGI_FORMAT,
	pub Usage: D3D11_USAGE,
	pub BindFlags: UINT,
	pub CPUAccessFlags: UINT,
	pub MiscFlags: UINT,
}
#[repr(C)] pub struct D3D11_TEXTURE2D_DESC {
	pub Width: UINT,
	pub Height: UINT,
	pub MipLevels: UINT,
	pub ArraySize: UINT,
	pub Format: DXGI_FORMAT,
	pub SampleDesc: DXGI_SAMPLE_DESC,
	pub Usage: D3D11_USAGE,
	pub BindFlags: UINT,
	pub CPUAccessFlags: UINT,
	pub MiscFlags: UINT,
}
#[repr(C)] pub struct D3D11_TEXTURE3D_DESC {
	pub Width: UINT,
	pub Height: UINT,
	pub Depth: UINT,
	pub MipLevels: UINT,
	pub Format: DXGI_FORMAT,
	pub Usage: D3D11_USAGE,
	pub BindFlags: UINT,
	pub CPUAccessFlags: UINT,
	pub MiscFlags: UINT,
}
#[repr(C)] pub struct D3D11_TILE_REGION_SIZE {
	pub NumTiles: UINT,
	pub bUseBox: BOOL,
	pub Width: UINT,
	pub Height: UINT16,
	pub Depth: UINT16,
}
#[repr(C)] pub struct D3D11_TILED_RESOURCE_COORDINATE {
	pub X: UINT,
	pub Y: UINT,
	pub Z: UINT,
	pub Subresource: UINT,
}
#[repr(C)] pub struct D3D11_TILE_SHAPE {
	pub WidthInTexels: UINT,
	pub HeightInTexels: UINT,
	pub DepthInTexels: UINT,
}
#[repr(C)] pub struct D3D11_UNORDERED_ACCESS_VIEW_DESC {
	pub Format: DXGI_FORMAT,
	pub ViewDimension: D3D11_UAV_DIMENSION,
	pub union_0: D3D11_BUFFER_UAV,
}