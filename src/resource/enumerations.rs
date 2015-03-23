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

use common_version::D3D_SRV_DIMENSION;

pub type D3D11_SRV_DIMENSION = D3D_SRV_DIMENSION;

#[repr(C)] pub enum D3D11_BIND_FLAG {
	D3D11_BIND_VERTEX_BUFFER = 0x1,
	D3D11_BIND_INDEX_BUFFER = 0x2,
	D3D11_BIND_CONSTANT_BUFFER = 0x4,
	D3D11_BIND_SHADER_RESOURCE = 0x8,
	D3D11_BIND_STREAM_OUTPUT = 0x10,
	D3D11_BIND_RENDER_TARGET = 0x20,
	D3D11_BIND_DEPTH_STENCIL = 0x40,
	D3D11_BIND_UNORDERED_ACCESS = 0x80,
	D3D11_BIND_DECODER = 0x200,
	D3D11_BIND_VIDEO_ENCODER = 0x400
}
#[repr(C)] pub enum D3D11_BUFFEREX_SRV_FLAG {
	D3D11_BUFFEREX_SRV_FLAG_RAW = 0x1
}
#[repr(C)] pub enum D3D11_BUFFER_UAV_FLAG {
	D3D11_BUFFER_UAV_FLAG_RAW = 0x1,
	D3D11_BUFFER_UAV_FLAG_APPEND = 0x2,
	D3D11_BUFFER_UAV_FLAG_COUNTER = 0x4
}
#[repr(C)] pub enum D3D11_CHECK_MULTISAMPLE_QUALITY_LEVELS_FLAG {
	D3D11_CHECK_MULTISAMPLE_QUALITY_LEVELS_TILED_RESOURCE = 0x1
}
#[repr(C)] pub enum D3D11_CPU_ACCESS_FLAG {
	D3D11_CPU_ACCESS_WRITE = 0x10000,
	D3D11_CPU_ACCESS_READ = 0x20000
}
#[repr(C)] pub enum D3D11_DSV_DIMENSION {
	D3D11_DSV_DIMENSION_UNKNOWN = 0,
	D3D11_DSV_DIMENSION_TEXTURE1D = 1,
	D3D11_DSV_DIMENSION_TEXTURE1DARRAY = 2,
	D3D11_DSV_DIMENSION_TEXTURE2D = 3,
	D3D11_DSV_DIMENSION_TEXTURE2DARRAY = 4,
	D3D11_DSV_DIMENSION_TEXTURE2DMS = 5,
	D3D11_DSV_DIMENSION_TEXTURE2DMSARRAY = 6
}
#[repr(C)] pub enum D3D11_DSV_FLAG {
	D3D11_DSV_READ_ONLY_DEPTH = 0x1,
	D3D11_DSV_READ_ONLY_STENCIL = 0x2
}
#[repr(C)] pub enum D3D11_MAP {
	D3D11_MAP_READ = 1,
	D3D11_MAP_WRITE = 2,
	D3D11_MAP_READ_WRITE = 3,
	D3D11_MAP_WRITE_DISCARD = 4,
	D3D11_MAP_WRITE_NO_OVERWRITE = 5
}
#[repr(C)] pub enum D3D11_MAP_FLAG {
	D3D11_MAP_FLAG_DO_NOT_WAIT = 0x100000
}
#[repr(C)] pub enum D3D11_RESOURCE_DIMENSION {
	D3D11_RESOURCE_DIMENSION_UNKNOWN = 0,
	D3D11_RESOURCE_DIMENSION_BUFFER = 1,
	D3D11_RESOURCE_DIMENSION_TEXTURE1D = 2,
	D3D11_RESOURCE_DIMENSION_TEXTURE2D = 3,
	D3D11_RESOURCE_DIMENSION_TEXTURE3D = 4
}
#[repr(C)] pub enum D3D11_RESOURCE_MISC_FLAG {
	D3D11_RESOURCE_MISC_GENERATE_MIPS = 0x1,
	D3D11_RESOURCE_MISC_SHARED = 0x2,
	D3D11_RESOURCE_MISC_TEXTURECUBE = 0x4,
	D3D11_RESOURCE_MISC_DRAWINDIRECT_ARGS = 0x10,
	D3D11_RESOURCE_MISC_BUFFER_ALLOW_RAW_VIEWS = 0x20,
	D3D11_RESOURCE_MISC_BUFFER_STRUCTURED = 0x40,
	D3D11_RESOURCE_MISC_RESOURCE_CLAMP = 0x80,
	D3D11_RESOURCE_MISC_SHARED_KEYEDMUTEX = 0x100,
	D3D11_RESOURCE_MISC_GDI_COMPATIBLE = 0x200,
	D3D11_RESOURCE_MISC_SHARED_NTHANDLE = 0x800,
	D3D11_RESOURCE_MISC_RESTRICTED_CONTENT = 0x1000,
	D3D11_RESOURCE_MISC_RESTRICT_SHARED_RESOURCE = 0x2000,
	D3D11_RESOURCE_MISC_RESTRICT_SHARED_RESOURCE_DRIVER = 0x4000,
	D3D11_RESOURCE_MISC_GUARDED = 0x8000,
	D3D11_RESOURCE_MISC_TILE_POOL = 0x20000,
	D3D11_RESOURCE_MISC_TILED = 0x40000
}
#[repr(C)] pub enum D3D11_RTV_DIMENSION {
	D3D11_RTV_DIMENSION_UNKNOWN = 0,
	D3D11_RTV_DIMENSION_BUFFER = 1,
	D3D11_RTV_DIMENSION_TEXTURE1D = 2,
	D3D11_RTV_DIMENSION_TEXTURE1DARRAY = 3,
	D3D11_RTV_DIMENSION_TEXTURE2D = 4,
	D3D11_RTV_DIMENSION_TEXTURE2DARRAY = 5,
	D3D11_RTV_DIMENSION_TEXTURE2DMS = 6,
	D3D11_RTV_DIMENSION_TEXTURE2DMSARRAY = 7,
	D3D11_RTV_DIMENSION_TEXTURE3D = 8
}
#[repr(C)] pub enum D3D11_STANDARD_MULTISAMPLE_QUALITY_LEVELS {
	D3D11_STANDARD_MULTISAMPLE_PATTERN = 0xffffffff,
	D3D11_CENTER_MULTISAMPLE_PATTERN = 0xfffffffe
}
#[repr(C)] pub enum D3D11_TILE_COPY_FLAG {
	D3D11_TILE_COPY_NO_OVERWRITE = 0x1,
	D3D11_TILE_COPY_LINEAR_BUFFER_TO_SWIZZLED_TILED_RESOURCE = 0x2,
	D3D11_TILE_COPY_SWIZZLED_TILED_RESOURCE_TO_LINEAR_BUFFER = 0x4
}
#[repr(C)] pub enum D3D11_TILE_MAPPING_FLAG {
	D3D11_TILE_MAPPING_NO_OVERWRITE = 0x1
}
#[repr(C)] pub enum D3D11_TILE_RANGE_FLAG {
	D3D11_TILE_RANGE_NULL = 0x1,
	D3D11_TILE_RANGE_SKIP = 0x2,
	D3D11_TILE_RANGE_REUSE_SINGLE_TILE = 0x4
}
#[repr(C)] pub enum D3D11_UAV_DIMENSION {
	D3D11_UAV_DIMENSION_UNKNOWN = 0,
	D3D11_UAV_DIMENSION_BUFFER = 1,
	D3D11_UAV_DIMENSION_TEXTURE1D = 2,
	D3D11_UAV_DIMENSION_TEXTURE1DARRAY = 3,
	D3D11_UAV_DIMENSION_TEXTURE2D = 4,
	D3D11_UAV_DIMENSION_TEXTURE2DARRAY = 5,
	D3D11_UAV_DIMENSION_TEXTURE3D = 8
}
#[repr(C)] pub enum D3D11_USAGE {
	D3D11_USAGE_DEFAULT = 0,
	D3D11_USAGE_IMMUTABLE = 1,
	D3D11_USAGE_DYNAMIC = 2,
	D3D11_USAGE_STAGING = 3
}