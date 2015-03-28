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

//! Resource Enumerations provided by D3D11
//!
//! # References
//! [D3D11 Resource Enumerations, MSDN]
//! (https://msdn.microsoft.com/en-us/library/windows/desktop/ff476170(v=vs.85).aspx)

#![allow(non_camel_case_types)]

use common_version::enumerations::D3D_SRV_DIMENSION;

pub type D3D11_SRV_DIMENSION = D3D_SRV_DIMENSION;

#[repr(C)] pub enum D3D11_BIND_FLAG {
	VERTEX_BUFFER = 0x1,
	INDEX_BUFFER = 0x2,
	CONSTANT_BUFFER = 0x4,
	SHADER_RESOURCE = 0x8,
	STREAM_OUTPUT = 0x10,
	RENDER_TARGET = 0x20,
	DEPTH_STENCIL = 0x40,
	UNORDERED_ACCESS = 0x80,
	DECODER = 0x200,
	VIDEO_ENCODER = 0x400
}
pub enum D3D11_BUFFEREX_SRV_FLAG {
	RAW = 0x1
}
#[repr(C)] pub enum D3D11_BUFFER_UAV_FLAG {
	RAW = 0x1,
	APPEND = 0x2,
	COUNTER = 0x4
}
pub enum D3D11_CHECK_MULTISAMPLE_QUALITY_LEVELS_FLAG {
	TILED_RESOURCE = 0x1
}
#[repr(C)] pub enum D3D11_CPU_ACCESS_FLAG {
	WRITE = 0x10000,
	READ = 0x20000
}
#[repr(C)] pub enum D3D11_DSV_DIMENSION {
	UNKNOWN = 0,
	TEXTURE1D = 1,
	TEXTURE1DARRAY = 2,
	TEXTURE2D = 3,
	TEXTURE2DARRAY = 4,
	TEXTURE2DMS = 5,
	TEXTURE2DMSARRAY = 6
}
#[repr(C)] pub enum D3D11_DSV_FLAG {
	DEPTH = 0x1,
	STENCIL = 0x2
}
#[repr(C)] pub enum D3D11_MAP {
	READ = 1,
	WRITE = 2,
	READ_WRITE = 3,
	WRITE_DISCARD = 4,
	WRITE_NO_OVERWRITE = 5
}
pub enum D3D11_MAP_FLAG {
	DO_NOT_WAIT = 0x100000
}
#[repr(C)] pub enum D3D11_RESOURCE_DIMENSION {
	UNKNOWN = 0,
	BUFFER = 1,
	TEXTURE1D = 2,
	TEXTURE2D = 3,
	TEXTURE3D = 4
}
#[repr(C)] pub enum D3D11_RESOURCE_MISC_FLAG {
	GENERATE_MIPS = 0x1,
	SHARED = 0x2,
	TEXTURECUBE = 0x4,
	DRAWINDIRECT_ARGS = 0x10,
	BUFFER_ALLOW_RAW_VIEWS = 0x20,
	BUFFER_STRUCTURED = 0x40,
	RESOURCE_CLAMP = 0x80,
	SHARED_KEYEDMUTEX = 0x100,
	GDI_COMPATIBLE = 0x200,
	SHARED_NTHANDLE = 0x800,
	RESTRICTED_CONTENT = 0x1000,
	RESTRICT_SHARED_RESOURCE = 0x2000,
	RESTRICT_SHARED_RESOURCE_DRIVER = 0x4000,
	GUARDED = 0x8000,
	TILE_POOL = 0x20000,
	TILED = 0x40000
}
#[repr(C)] pub enum D3D11_RTV_DIMENSION {
	UNKNOWN = 0,
	BUFFER = 1,
	TEXTURE1D = 2,
	TEXTURE1DARRAY = 3,
	TEXTURE2D = 4,
	TEXTURE2DARRAY = 5,
	TEXTURE2DMS = 6,
	TEXTURE2DMSARRAY = 7,
	TEXTURE3D = 8
}
#[repr(C)] pub enum D3D11_STANDARD_MULTISAMPLE_QUALITY_LEVELS {
	STANDARD_MULTISAMPLE_PATTERN = 0xffffffff,
	CENTER_MULTISAMPLE_PATTERN = 0xfffffffe
}
#[repr(C)] pub enum D3D11_TILE_COPY_FLAG {
	NO_OVERWRITE = 0x1,
	LINEAR_BUFFER_TO_SWIZZLED_TILED_RESOURCE = 0x2,
	SWIZZLED_TILED_RESOURCE_TO_LINEAR_BUFFER = 0x4
}
pub enum D3D11_TILE_MAPPING_FLAG {
	NO_OVERWRITE = 0x1
}
#[repr(C)] pub enum D3D11_TILE_RANGE_FLAG {
	NULL = 0x1,
	SKIP = 0x2,
	REUSE_SINGLE_TILE = 0x4
}
#[repr(C)] pub enum D3D11_UAV_DIMENSION {
	UNKNOWN = 0,
	BUFFER = 1,
	TEXTURE1D = 2,
	TEXTURE1DARRAY = 3,
	TEXTURE2D = 4,
	TEXTURE2DARRAY = 5,
	TEXTURE3D = 8
}
#[repr(C)] pub enum D3D11_USAGE {
	DEFAULT = 0,
	IMMUTABLE = 1,
	DYNAMIC = 2,
	STAGING = 3
}