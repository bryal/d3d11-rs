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

//! Structures provided by D3D11
//!
//! # References
//! [D3D11 Structures, MSDN]
//! (https://msdn.microsoft.com/en-us/library/windows/desktop/ff476155(v=vs.85).aspx)

#![allow(non_snake_case, non_camel_case_types)]

use winapi::{ BOOL, UINT, FLOAT,
	BYTE, LPCSTR, UINT8,
	UINT64, INT, RECT};
use dxgi::DXGI_FORMAT;

use super::enumerations::*;

pub type D3D11_RECT = RECT;

#[repr(C)] pub struct D3D11_BLEND_DESC {
	pub AlphaToCoverageEnable: BOOL,
	pub IndependentBlendEnable: BOOL,
	pub RenderTarget: [D3D11_RENDER_TARGET_BLEND_DESC; 8],
}

#[repr(C)] pub struct D3D11_BLEND_DESC1 {
	pub AlphaToCoverageEnable: BOOL,
	pub IndependentBlendEnable: BOOL,
	pub RenderTarget: [D3D11_RENDER_TARGET_BLEND_DESC1; 8],
}

#[repr(C)] pub struct D3D11_BOX {
	pub left: UINT,
	pub top: UINT,
	pub front: UINT,
	pub right: UINT,
	pub bottom: UINT,
	pub back: UINT,
}

#[repr(C)] pub struct D3D11_COUNTER_DESC {
	pub Counter: D3D11_COUNTER,
	pub MiscFlags: UINT,
}

#[repr(C)] pub struct D3D11_COUNTER_INFO {
	pub LastDeviceDependentCounter: D3D11_COUNTER,
	pub NumSimultaneousCounters: UINT,
	pub NumDetectableParallelUnits: UINT8,
}

#[repr(C)] pub struct D3D11_DEPTH_STENCIL_DESC {
	pub DepthEnable: BOOL,
	pub DepthWriteMask: D3D11_DEPTH_WRITE_MASK,
	pub DepthFunc: D3D11_COMPARISON_FUNC,
	pub StencilEnable: BOOL,
	pub StencilReadMask: UINT8,
	pub StencilWriteMask: UINT8,
	pub FrontFace: D3D11_DEPTH_STENCILOP_DESC,
	pub BackFace: D3D11_DEPTH_STENCILOP_DESC,
}

#[repr(C)] pub struct D3D11_DEPTH_STENCILOP_DESC {
	pub StencilFailOp: D3D11_STENCIL_OP,
	pub StencilDepthFailOp: D3D11_STENCIL_OP,
	pub StencilPassOp: D3D11_STENCIL_OP,
	pub StencilFunc: D3D11_COMPARISON_FUNC,
}

#[repr(C)] pub struct D3D11_FEATURE_DATA_ARCHITECTURE_INFO {
	pub TileBasedDeferredRenderer: BOOL,
}

#[repr(C)] pub struct D3D11_FEATURE_DATA_D3D9_OPTIONS {
	pub FullNonPow2TextureSupport: BOOL,
}

#[repr(C)] pub struct D3D11_FEATURE_DATA_D3D9_SHADOW_SUPPORT {
	pub SupportsDepthAsTextureWithLessEqualComparisonFilter: BOOL,
}

#[repr(C)] pub struct D3D11_FEATURE_DATA_D3D9_SIMPLE_INSTANCING_SUPPORT {
	pub SimpleInstancingSupported: BOOL,
}

#[repr(C)] pub struct D3D11_FEATURE_DATA_D3D10_X_HARDWARE_OPTIONS {
	pub ComputeShaders_Plus_RawAndStructuredBuffers_Via_Shader_4_x: BOOL,
}

#[repr(C)] pub struct D3D11_FEATURE_DATA_D3D11_OPTIONS {
	pub OutputMergerLogicOp: BOOL,
	pub UAVOnlyRenderingForcedSampleCount: BOOL,
	pub DiscardAPIsSeenByDriver: BOOL,
	pub FlagsForUpdateAndCopySeenByDriver: BOOL,
	pub ClearView: BOOL,
	pub CopyWithOverlap: BOOL,
	pub ConstantBufferPartialUpdate: BOOL,
	pub ConstantBufferOffsetting: BOOL,
	pub MapNoOverwriteOnDynamicConstantBuffer: BOOL,
	pub MapNoOverwriteOnDynamicBufferSRV: BOOL,
	pub MultisampleRTVWithForcedSampleCountOne: BOOL,
	pub SAD4ShaderInstructions: BOOL,
	pub ExtendedDoublesShaderInstructions: BOOL,
	pub ExtendedResourceSharing: BOOL,
}

#[repr(C)] pub struct D3D11_FEATURE_DATA_D3D11_OPTIONS1 {
	pub TiledResourcesTier: D3D11_TILED_RESOURCES_TIER,
	pub MinMaxFiltering: BOOL,
	pub ClearViewAlsoSupportsDepthOnlyFormats: BOOL,
	pub MapOnDefaultBuffers: BOOL,
}

#[repr(C)] pub struct D3D11_FEATURE_DATA_DOUBLES {
	pub DoublePrecisionFloatShaderOps: BOOL,
}

#[repr(C)] pub struct D3D11_FEATURE_DATA_FORMAT_SUPPORT {
	pub InFormat: DXGI_FORMAT,
	pub OutFormatSupport: UINT,
}

pub struct D3D11_FEATURE_DATA_FORMAT_SUPPORT2 {
	pub InFormat: DXGI_FORMAT,
	pub OutFormatSupport2: UINT,
}

#[repr(C)] pub struct D3D11_FEATURE_DATA_MARKER_SUPPORT {
	pub Profile: BOOL,
}

#[repr(C)] pub struct D3D11_FEATURE_DATA_THREADING {
	pub DriverConcurrentCreates: BOOL,
	pub DriverCommandLists: BOOL,
}

#[repr(C)] pub struct D3D11_INPUT_ELEMENT_DESC {
	pub SemanticName: LPCSTR,
	pub SemanticIndex: UINT,
	pub Format: DXGI_FORMAT,
	pub InputSlot: UINT,
	pub AlignedByteOffset: UINT,
	pub InputSlotClass: D3D11_INPUT_CLASSIFICATION,
	pub InstanceDataStepRate: UINT,
}

#[repr(C)] pub struct D3D11_QUERY_DATA_PIPELINE_STATISTICS {
	pub IAVertices: UINT64,
	pub IAPrimitives: UINT64,
	pub VSInvocations: UINT64,
	pub GSInvocations: UINT64,
	pub GSPrimitives: UINT64,
	pub CInvocations: UINT64,
	pub CPrimitives: UINT64,
	pub PSInvocations: UINT64,
	pub HSInvocations: UINT64,
	pub DSInvocations: UINT64,
	pub CSInvocations: UINT64,
}

#[repr(C)] pub struct D3D11_QUERY_DATA_SO_STATISTICS {
	pub NumPrimitivesWritten: UINT64,
	pub PrimitivesStorageNeeded: UINT64,
}

#[repr(C)] pub struct D3D11_QUERY_DATA_TIMESTAMP_DISJOINT {
	pub Frequency: UINT64,
	pub Disjoint: BOOL,
}

#[repr(C)] pub struct D3D11_QUERY_DESC {
	pub Query: D3D11_QUERY,
	pub MiscFlags: UINT,
}

#[repr(C)] pub struct D3D11_RASTERIZER_DESC {
	pub FillMode: D3D11_FILL_MODE,
	pub CullMode: D3D11_CULL_MODE,
	pub FrontCounterClockwise: BOOL,
	pub DepthBias: INT,
	pub DepthBiasClamp: FLOAT,
	pub SlopeScaledDepthBias: FLOAT,
	pub DepthClipEnable: BOOL,
	pub ScissorEnable: BOOL,
	pub MultisampleEnable: BOOL,
	pub AntialiasedLineEnable: BOOL,
}

#[repr(C)] pub struct D3D11_RASTERIZER_DESC1 {
	pub FillMode: D3D11_FILL_MODE,
	pub CullMode: D3D11_CULL_MODE,
	pub FrontCounterClockwise: BOOL,
	pub DepthBias: INT,
	pub DepthBiasClamp: FLOAT,
	pub SlopeScaledDepthBias: FLOAT,
	pub DepthClipEnable: BOOL,
	pub ScissorEnable: BOOL,
	pub MultisampleEnable: BOOL,
	pub AntialiasedLineEnable: BOOL,
	pub ForcedSampleCount: UINT,
}

#[repr(C)] pub struct D3D11_RENDER_TARGET_BLEND_DESC {
	pub BlendEnable: BOOL,
	pub SrcBlend: D3D11_BLEND,
	pub DestBlend: D3D11_BLEND,
	pub BlendOp: D3D11_BLEND_OP,
	pub SrcBlendAlpha: D3D11_BLEND,
	pub DestBlendAlpha: D3D11_BLEND,
	pub BlendOpAlpha: D3D11_BLEND_OP,
	pub RenderTargetWriteMask: UINT8,
}

#[repr(C)] pub struct D3D11_RENDER_TARGET_BLEND_DESC1 {
	pub BlendEnable: BOOL,
	pub LogicOpEnable: BOOL,
	pub SrcBlend: D3D11_BLEND,
	pub DestBlend: D3D11_BLEND,
	pub BlendOp: D3D11_BLEND_OP,
	pub SrcBlendAlpha: D3D11_BLEND,
	pub DestBlendAlpha: D3D11_BLEND,
	pub BlendOpAlpha: D3D11_BLEND_OP,
	pub LogicOp: D3D11_LOGIC_OP,
	pub RenderTargetWriteMask: UINT8,
}

#[repr(C)] pub struct D3D11_SAMPLER_DESC {
	pub Filter: D3D11_FILTER,
	pub AddressU: D3D11_TEXTURE_ADDRESS_MODE,
	pub AddressV: D3D11_TEXTURE_ADDRESS_MODE,
	pub AddressW: D3D11_TEXTURE_ADDRESS_MODE,
	pub MipLODBias: FLOAT,
	pub MaxAnisotropy: UINT,
	pub ComparisonFunc: D3D11_COMPARISON_FUNC,
	pub BorderColor: [FLOAT; 4],
	pub MinLOD: FLOAT,
	pub MaxLOD: FLOAT,
}

#[repr(C)] pub struct D3D11_SO_DECLARATION_ENTRY {
	pub Stream: UINT,
	pub SemanticName: LPCSTR,
	pub SemanticIndex: UINT,
	pub StartComponent: BYTE,
	pub ComponentCount: BYTE,
	pub OutputSlot: BYTE,
}

#[repr(C)] pub struct D3D11_VIEWPORT {
	pub TopLeftX: FLOAT,
	pub TopLeftY: FLOAT,
	pub Width: FLOAT,
	pub Height: FLOAT,
	pub MinDepth: FLOAT,
	pub MaxDepth: FLOAT,
}