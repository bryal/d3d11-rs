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

type D3D11_RECT = RECT;

#[repr(C)] pub struct D3D11_RENDER_TARGET_BLEND_DESC {
	BlendEnable: BOOL,
	SrcBlend: D3D11_BLEND,
	DestBlend: D3D11_BLEND,
	BlendOp: D3D11_BLEND_OP,
	SrcBlendAlpha: D3D11_BLEND,
	DestBlendAlpha: D3D11_BLEND,
	BlendOpAlpha: D3D11_BLEND_OP,
	RenderTargetWriteMask: UINT8,
}

#[repr(C)] pub struct D3D11_RENDER_TARGET_BLEND_DESC1 {
	BlendEnable: BOOL,
	LogicOpEnable: BOOL,
	SrcBlend: D3D11_BLEND,
	DestBlend: D3D11_BLEND,
	BlendOp: D3D11_BLEND_OP,
	SrcBlendAlpha: D3D11_BLEND,
	DestBlendAlpha: D3D11_BLEND,
	BlendOpAlpha: D3D11_BLEND_OP,
	LogicOp: D3D11_LOGIC_OP,
	RenderTargetWriteMask: UINT8,
}

#[repr(C)] pub struct D3D11_BOX {
	left: UINT,
	top: UINT,
	front: UINT,
	right: UINT,
	bottom: UINT,
	back: UINT,
}

#[repr(C)] pub struct D3D11_COUNTER_DESC {
	Counter: D3D11_COUNTER,
	MiscFlags: UINT,
}

#[repr(C)] pub struct D3D11_COUNTER_INFO {
	LastDeviceDependentCounter: D3D11_COUNTER,
	NumSimultaneousCounters: UINT,
	NumDetectableParallelUnits: UINT8,
}

#[repr(C)] pub struct D3D11_DEPTH_STENCIL_DESC {
	DepthEnable: BOOL,
	DepthWriteMask: D3D11_DEPTH_WRITE_MASK,
	DepthFunc: D3D11_COMPARISON_FUNC,
	StencilEnable: BOOL,
	StencilReadMask: UINT8,
	StencilWriteMask: UINT8,
	FrontFace: D3D11_DEPTH_STENCILOP_DESC,
	BackFace: D3D11_DEPTH_STENCILOP_DESC,
}

#[repr(C)] pub struct D3D11_DEPTH_STENCILOP_DESC {
	StencilFailOp: D3D11_STENCIL_OP,
	StencilDepthFailOp: D3D11_STENCIL_OP,
	StencilPassOp: D3D11_STENCIL_OP,
	StencilFunc: D3D11_COMPARISON_FUNC,
}

#[repr(C)] pub struct D3D11_FEATURE_DATA_ARCHITECTURE_INFO {
	TileBasedDeferredRenderer: BOOL,
}

#[repr(C)] pub struct D3D11_FEATURE_DATA_D3D9_OPTIONS {
	FullNonPow2TextureSupport: BOOL,
}

#[repr(C)] pub struct D3D11_FEATURE_DATA_D3D9_SHADOW_SUPPORT {
	SupportsDepthAsTextureWithLessEqualComparisonFilter: BOOL,
}

#[repr(C)] pub struct D3D11_FEATURE_DATA_D3D9_SIMPLE_INSTANCING_SUPPORT {
	SimpleInstancingSupported: BOOL,
}

#[repr(C)] pub struct D3D11_FEATURE_DATA_D3D10_X_HARDWARE_OPTIONS {
	ComputeShaders_Plus_RawAndStructuredBuffers_Via_Shader_4_x: BOOL,
}

#[repr(C)] pub struct D3D11_FEATURE_DATA_D3D11_OPTIONS {
	OutputMergerLogicOp: BOOL,
	UAVOnlyRenderingForcedSampleCount: BOOL,
	DiscardAPIsSeenByDriver: BOOL,
	FlagsForUpdateAndCopySeenByDriver: BOOL,
	ClearView: BOOL,
	CopyWithOverlap: BOOL,
	ConstantBufferPartialUpdate: BOOL,
	ConstantBufferOffsetting: BOOL,
	MapNoOverwriteOnDynamicConstantBuffer: BOOL,
	MapNoOverwriteOnDynamicBufferSRV: BOOL,
	MultisampleRTVWithForcedSampleCountOne: BOOL,
	SAD4ShaderInstructions: BOOL,
	ExtendedDoublesShaderInstructions: BOOL,
	ExtendedResourceSharing: BOOL,
}

#[repr(C)] pub struct D3D11_FEATURE_DATA_D3D11_OPTIONS1 {
	TiledResourcesTier: D3D11_TILED_RESOURCES_TIER,
	MinMaxFiltering: BOOL,
	ClearViewAlsoSupportsDepthOnlyFormats: BOOL,
	MapOnDefaultBuffers: BOOL,
}

#[repr(C)] pub struct D3D11_FEATURE_DATA_DOUBLES {
	DoublePrecisionFloatShaderOps: BOOL,
}

#[repr(C)] pub struct D3D11_FEATURE_DATA_FORMAT_SUPPORT {
	InFormat: DXGI_FORMAT,
	OutFormatSupport: UINT,
	}struct D3D11_FEATURE_DATA_FORMAT_SUPPORT2
	{
	InFormat: DXGI_FORMAT,
	OutFormatSupport2: UINT,
}

#[repr(C)] pub struct D3D11_FEATURE_DATA_MARKER_SUPPORT {
	Profile: BOOL,
	}
	D3D11_SHADER_MIN_PRECISION_SUPPORT
	{
		D3D11_SHADER_MIN_PRECISION_10_BIT	=0x1: ,
		D3D11_SHADER_MIN_PRECISION_16_BIT	= 0x2
}

#[repr(C)] pub struct D3D11_FEATURE_DATA_THREADING {
	DriverConcurrentCreates: BOOL,
	DriverCommandLists: BOOL,
}

#[repr(C)] pub struct D3D11_INPUT_ELEMENT_DESC {
	SemanticName: LPCSTR,
	SemanticIndex: UINT,
	Format: DXGI_FORMAT,
	InputSlot: UINT,
	AlignedByteOffset: UINT,
	InputSlotClass: D3D11_INPUT_CLASSIFICATION,
	InstanceDataStepRate: UINT,
}

#[repr(C)] pub struct D3D11_QUERY_DATA_PIPELINE_STATISTICS {
	IAVertices: UINT64,
	IAPrimitives: UINT64,
	VSInvocations: UINT64,
	GSInvocations: UINT64,
	GSPrimitives: UINT64,
	CInvocations: UINT64,
	CPrimitives: UINT64,
	PSInvocations: UINT64,
	HSInvocations: UINT64,
	DSInvocations: UINT64,
	CSInvocations: UINT64,
}

#[repr(C)] pub struct D3D11_QUERY_DATA_SO_STATISTICS {
	NumPrimitivesWritten: UINT64,
	PrimitivesStorageNeeded: UINT64,
}

#[repr(C)] pub struct D3D11_QUERY_DATA_TIMESTAMP_DISJOINT {
	Frequency: UINT64,
	Disjoint: BOOL,
}

#[repr(C)] pub struct D3D11_QUERY_DESC {
	Query: D3D11_QUERY,
	MiscFlags: UINT,
}

#[repr(C)] pub struct D3D11_RASTERIZER_DESC {
	FillMode: D3D11_FILL_MODE,
	CullMode: D3D11_CULL_MODE,
	FrontCounterClockwise: BOOL,
	DepthBias: INT,
	DepthBiasClamp: FLOAT,
	SlopeScaledDepthBias: FLOAT,
	DepthClipEnable: BOOL,
	ScissorEnable: BOOL,
	MultisampleEnable: BOOL,
	AntialiasedLineEnable: BOOL,
}

#[repr(C)] pub struct D3D11_RASTERIZER_DESC1 {
	FillMode: D3D11_FILL_MODE,
	CullMode: D3D11_CULL_MODE,
	FrontCounterClockwise: BOOL,
	DepthBias: INT,
	DepthBiasClamp: FLOAT,
	SlopeScaledDepthBias: FLOAT,
	DepthClipEnable: BOOL,
	ScissorEnable: BOOL,
	MultisampleEnable: BOOL,
	AntialiasedLineEnable: BOOL,
	ForcedSampleCount: UINT,
}

#[repr(C)] pub struct D3D11_RENDER_TARGET_BLEND_DESC {
	BlendEnable: BOOL,
	SrcBlend: D3D11_BLEND,
	DestBlend: D3D11_BLEND,
	BlendOp: D3D11_BLEND_OP,
	SrcBlendAlpha: D3D11_BLEND,
	DestBlendAlpha: D3D11_BLEND,
	BlendOpAlpha: D3D11_BLEND_OP,
	RenderTargetWriteMask: UINT8,
}

#[repr(C)] pub struct D3D11_RENDER_TARGET_BLEND_DESC1 {
	BlendEnable: BOOL,
	LogicOpEnable: BOOL,
	SrcBlend: D3D11_BLEND,
	DestBlend: D3D11_BLEND,
	BlendOp: D3D11_BLEND_OP,
	SrcBlendAlpha: D3D11_BLEND,
	DestBlendAlpha: D3D11_BLEND,
	BlendOpAlpha: D3D11_BLEND_OP,
	LogicOp: D3D11_LOGIC_OP,
	RenderTargetWriteMask: UINT8,
}

#[repr(C)] pub struct D3D11_SAMPLER_DESC {
	Filter: D3D11_FILTER,
	AddressU: D3D11_TEXTURE_ADDRESS_MODE,
	AddressV: D3D11_TEXTURE_ADDRESS_MODE,
	AddressW: D3D11_TEXTURE_ADDRESS_MODE,
	MipLODBias: FLOAT,
	MaxAnisotropy: UINT,
	ComparisonFunc: D3D11_COMPARISON_FUNC,
	FLOAT BorderColor[ 4 ],
	MinLOD: FLOAT,
	MaxLOD: FLOAT,
}

#[repr(C)] pub struct D3D11_SO_DECLARATION_ENTRY {
	Stream: UINT,
	SemanticName: LPCSTR,
	SemanticIndex: UINT,
	StartComponent: BYTE,
	ComponentCount: BYTE,
	OutputSlot: BYTE,
}

#[repr(C)] pub struct D3D11_VIEWPORT {
	TopLeftX: FLOAT,
	TopLeftY: FLOAT,
	Width: FLOAT,
	Height: FLOAT,
	MinDepth: FLOAT,
	MaxDepth: FLOAT,
}