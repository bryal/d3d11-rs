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

//! Interfaces provided by D3D11
//!
//! # References
//! [D3D11 Interfaces, MSDN]
//! (https://msdn.microsoft.com/en-us/library/windows/desktop/ff476154(v=vs.85).aspx)

use libc::c_void;
use winapi::{ UINT, REFGUID, SIZE_T, HRESULT, REFIID,
	ULONG, HANDLE, FLOAT, BOOL, UINT8,
	LPSTR, LPCWSTR, DWORD, INT, UINT64 };
use dxgi::{ IUnknown, IUnknownT, DXGI_FORMAT };

use core::structures::{ D3D11_SAMPLER_DESC, D3D11_INPUT_ELEMENT_DESC,
	D3D11_COUNTER_INFO, D3D11_BLEND_DESC1,
	D3D11_DEPTH_STENCIL_DESC, D3D11_RASTERIZER_DESC,
	D3D11_RASTERIZER_DESC1, D3D11_RECT,
	D3D11_BOX, D3D11_SO_DECLARATION_ENTRY,
	D3D11_VIEWPORT, D3D11_COUNTER_DESC,
	D3D11_QUERY_DESC, D3D11_BLEND_DESC };
use core::enumerations::{ D3D11_DEVICE_CONTEXT_TYPE, D3D11_FEATURE,D3D11_COUNTER_TYPE,
	D3D11_PRIMITIVE_TOPOLOGY };
use common_version::enumerations::{ D3D_DRIVER_TYPE, D3D_FEATURE_LEVEL };
use resource::enumerations::{ D3D11_MAP };
use resource::structures::{ D3D11_BUFFER_DESC, D3D11_SHADER_RESOURCE_VIEW_DESC,
	D3D11_SUBRESOURCE_DATA, D3D11_UNORDERED_ACCESS_VIEW_DESC,
	D3D11_RENDER_TARGET_VIEW_DESC, D3D11_DEPTH_STENCIL_VIEW_DESC,
	D3D11_TEXTURE1D_DESC, D3D11_TEXTURE2D_DESC,
	D3D11_PACKED_MIP_DESC, D3D11_TILE_SHAPE,
	D3D11_TILED_RESOURCE_COORDINATE, D3D11_TILE_REGION_SIZE,
	D3D11_MAPPED_SUBRESOURCE, D3D11_TEXTURE3D_DESC,
	D3D11_SUBRESOURCE_TILING };
use resource::interfaces::{ ID3D11View, ID3D11Buffer,
	ID3D11UnorderedAccessView, ID3D11ShaderResourceView,
	ID3D11DepthStencilView, ID3D11RenderTargetView,
	ID3D11Texture1D, ID3D11Texture2D,
	ID3D11Texture3D, ID3D11Resource };
use shader::interfaces::{ ID3D11ClassInstance, ID3D11PixelShader,
	ID3D11VertexShader, ID3D11GeometryShader,
	ID3D11HullShader, ID3D11DomainShader,
	ID3D11ComputeShader, ID3D11ClassLinkage };

#[repr(C)] pub struct ID3D11Asynchronous { pub vtable: *mut ID3D11AsynchronousVtbl }
#[repr(C)] pub struct ID3D11BlendState { pub vtable: *mut ID3D11BlendStateVtbl }
#[repr(C)] pub struct ID3D11BlendState1 { pub vtable: *mut ID3D11BlendState1Vtbl }
#[repr(C)] pub struct ID3D11Counter { pub vtable: *mut ID3D11CounterVtbl }
#[repr(C)] pub struct ID3D11CommandList { pub vtable: *mut ID3D11CommandListVtbl }
#[repr(C)] pub struct ID3D11DepthStencilState { pub vtable: *mut ID3D11DepthStencilStateVtbl }
#[repr(C)] pub struct ID3D11Device { pub vtable: *mut ID3D11DeviceVtbl }
#[repr(C)] pub struct ID3D11Device1 { pub vtable: *mut ID3D11Device1Vtbl }
#[repr(C)] pub struct ID3D11Device2 { pub vtable: *mut ID3D11Device2Vtbl }
#[repr(C)] pub struct ID3D11DeviceChild { pub vtable: *mut ID3D11DeviceChildVtbl }
#[repr(C)] pub struct ID3D11DeviceContext { pub vtable: *mut ID3D11DeviceContextVtbl }
#[repr(C)] pub struct ID3D11DeviceContext1 { pub vtable: *mut ID3D11DeviceContext1Vtbl }
#[repr(C)] pub struct ID3D11DeviceContext2 { pub vtable: *mut ID3D11DeviceContext2Vtbl }
#[repr(C)] pub struct ID3DDeviceContextState { pub vtable: *mut ID3DDeviceContextStateVtbl }
#[repr(C)] pub struct ID3D11InputLayout { pub vtable: *mut ID3D11InputLayoutVtbl }
#[repr(C)] pub struct ID3D11Predicate { pub vtable: *mut ID3D11PredicateVtbl }
#[repr(C)] pub struct ID3D11Query { pub vtable: *mut ID3D11QueryVtbl }
#[repr(C)] pub struct ID3D11RasterizerState { pub vtable: *mut ID3D11RasterizerStateVtbl }
#[repr(C)] pub struct ID3D11RasterizerState1 { pub vtable: *mut ID3D11RasterizerState1Vtbl }
#[repr(C)] pub struct ID3D11SamplerState { pub vtable: *mut ID3D11SamplerStateVtbl }

c_vtable!{
	_ of (), trait IUnknownT {
		fn QueryInterface(riid: REFIID, object: *mut *mut c_void) -> HRESULT,
		fn AddRef() -> ULONG,
		fn Release() -> ULONG,
	} with heirs [
		pub ID3D11DeviceChildVtbl of ID3D11DeviceChild, pub trait ID3D11DeviceChildT {
			fn GetDevice(device: *mut *mut ID3D11Device) -> (),
			fn GetPrivateData(guid: REFGUID, data_size: *mut UINT, data: *mut c_void) -> HRESULT,
			fn SetPrivateData(guid: REFGUID, data_size: UINT, data: *const c_void) -> HRESULT,
			fn SetPrivateDataInterface(guid: REFGUID, data: *const IUnknown) -> HRESULT,
		} with heirs [
			pub ID3D11AsynchronousVtbl of ID3D11Asynchronous, pub trait ID3D11AsynchronousT {
				fn GetDataSize() -> UINT,
			} with heirs [
				pub ID3D11CounterVtbl of ID3D11Counter, pub trait ID3D11CounterT {
					fn GetDesc(desc: *mut D3D11_COUNTER_DESC) -> (),
				}
				pub ID3D11QueryVtbl of ID3D11Query, pub trait ID3D11QueryT {
					fn GetDesc(desc: *mut D3D11_QUERY_DESC) -> (),
				} with heirs [
					pub ID3D11PredicateVtbl of ID3D11Predicate, pub trait ID3D11PredicateT { }
				]
			]
			pub ID3D11BlendStateVtbl of ID3D11BlendState, pub trait ID3D11BlendStateT {
				fn GetDesc(desc: *mut D3D11_BLEND_DESC) -> (),
			} with heirs [
				pub ID3D11BlendState1Vtbl of ID3D11BlendState1, pub trait ID3D11BlendState1T {
					fn GetDesc1(desc: *mut D3D11_BLEND_DESC1) -> (),
				}
			]
			pub ID3D11CommandListVtbl of ID3D11CommandList, pub trait ID3D11CommandListT {
				fn GetContextFlags() -> UINT,
			}
			pub ID3D11DepthStencilStateVtbl of ID3D11DepthStencilState, pub trait ID3D11DepthStencilStateT {
				fn GetDesc(desc: *mut D3D11_DEPTH_STENCIL_DESC) -> (),
			}
		]
		pub ID3D11DeviceVtbl of ID3D11Device, pub trait ID3D11DeviceT {
			fn CreateBuffer(desc: *const D3D11_BUFFER_DESC, initial_data: *const D3D11_SUBRESOURCE_DATA, buffer: *mut *mut ID3D11Buffer) -> HRESULT,
			fn CreateTexture1D(desc: *const D3D11_TEXTURE1D_DESC, initial_data: *const D3D11_SUBRESOURCE_DATA, texture1_d: *mut *mut ID3D11Texture1D) -> HRESULT,
			fn CreateTexture2D(desc: *const D3D11_TEXTURE2D_DESC, initial_data: *const D3D11_SUBRESOURCE_DATA, texture2_d: *mut *mut ID3D11Texture2D) -> HRESULT,
			fn CreateTexture3D(desc: *const D3D11_TEXTURE3D_DESC, initial_data: *const D3D11_SUBRESOURCE_DATA, texture3_d: *mut *mut ID3D11Texture3D) -> HRESULT,
			fn CreateShaderResourceView(resource: *mut ID3D11Resource, desc: *const D3D11_SHADER_RESOURCE_VIEW_DESC, s_r_view: *mut *mut ID3D11ShaderResourceView) -> HRESULT,
			fn CreateUnorderedAccessView(resource: *mut ID3D11Resource, desc: *const D3D11_UNORDERED_ACCESS_VIEW_DESC, u_a_view: *mut *mut ID3D11UnorderedAccessView) -> HRESULT,
			fn CreateRenderTargetView(resource: *mut ID3D11Resource, desc: *const D3D11_RENDER_TARGET_VIEW_DESC, r_t_view: *mut *mut ID3D11RenderTargetView) -> HRESULT,
			fn CreateDepthStencilView(resource: *mut ID3D11Resource, desc: *const D3D11_DEPTH_STENCIL_VIEW_DESC, depth_stencil_view: *mut *mut ID3D11DepthStencilView) -> HRESULT,
			fn CreateInputLayout(input_element_descs: *const D3D11_INPUT_ELEMENT_DESC, num_elements: UINT, shader_bytecode_with_input_signature: *const c_void, bytecode_length: SIZE_T, input_layout: *mut *mut ID3D11InputLayout) -> HRESULT,
			fn CreateVertexShader(shader_bytecode: *const c_void, bytecode_length: SIZE_T, class_linkage: *mut ID3D11ClassLinkage, vertex_shader: *mut *mut ID3D11VertexShader) -> HRESULT,
			fn CreateGeometryShader(shader_bytecode: *const c_void, bytecode_length: SIZE_T, class_linkage: *mut ID3D11ClassLinkage, geometry_shader: *mut *mut ID3D11GeometryShader) -> HRESULT,
			fn CreateGeometryShaderWithStreamOutput(shader_bytecode: *const c_void, bytecode_length: SIZE_T, s_o_declaration: *const D3D11_SO_DECLARATION_ENTRY, num_entries: UINT, buffer_strides: *const UINT, num_strides: UINT, rasterized_stream: UINT, class_linkage: *mut ID3D11ClassLinkage, geometry_shader: *mut *mut ID3D11GeometryShader) -> HRESULT,
			fn CreatePixelShader(shader_bytecode: *const c_void, bytecode_length: SIZE_T, class_linkage: *mut ID3D11ClassLinkage, pixel_shader: *mut *mut ID3D11PixelShader) -> HRESULT,
			fn CreateHullShader(shader_bytecode: *const c_void, bytecode_length: SIZE_T, class_linkage: *mut ID3D11ClassLinkage, hull_shader: *mut *mut ID3D11HullShader) -> HRESULT,
			fn CreateDomainShader(shader_bytecode: *const c_void, bytecode_length: SIZE_T, class_linkage: *mut ID3D11ClassLinkage, domain_shader: *mut *mut ID3D11DomainShader) -> HRESULT,
			fn CreateComputeShader(shader_bytecode: *const c_void, bytecode_length: SIZE_T, class_linkage: *mut ID3D11ClassLinkage, compute_shader: *mut *mut ID3D11ComputeShader) -> HRESULT,
			fn CreateClassLinkage(linkage: *mut *mut ID3D11ClassLinkage) -> HRESULT,
			fn CreateBlendState(blend_state_desc: *const D3D11_BLEND_DESC, blend_state: *mut *mut ID3D11BlendState) -> HRESULT,
			fn CreateDepthStencilState(depth_stencil_desc: *const D3D11_DEPTH_STENCIL_DESC, depth_stencil_state: *mut *mut ID3D11DepthStencilState) -> HRESULT,
			fn CreateRasterizerState(rasterizer_desc: *const D3D11_RASTERIZER_DESC, rasterizer_state: *mut *mut ID3D11RasterizerState) -> HRESULT,
			fn CreateSamplerState(sampler_desc: *const D3D11_SAMPLER_DESC, sampler_state: *mut *mut ID3D11SamplerState) -> HRESULT,
			fn CreateQuery(query_desc: *const D3D11_QUERY_DESC, query: *mut *mut ID3D11Query) -> HRESULT,
			fn CreatePredicate(predicate_desc: *const D3D11_QUERY_DESC, predicate: *mut *mut ID3D11Predicate) -> HRESULT,
			fn CreateCounter(counter_desc: *const D3D11_COUNTER_DESC, counter: *mut *mut ID3D11Counter) -> HRESULT,
			fn CreateDeferredContext(context_flags: UINT, deferred_context: *mut *mut ID3D11DeviceContext) -> HRESULT,
			fn OpenSharedResource(h_resource: HANDLE, returned_interface: REFIID, resource: *mut *mut c_void) -> HRESULT,
			fn CheckFormatSupport(format: DXGI_FORMAT, format_support: *mut UINT) -> HRESULT,
			fn CheckMultisampleQualityLevels(format: DXGI_FORMAT, samle_count: UINT, num_quality_levels: *mut UINT) -> HRESULT,
			fn CheckCounterInfo(counter_info: *mut D3D11_COUNTER_INFO) -> (),
			fn CheckCounter(desc: *const D3D11_COUNTER_DESC, type_: *mut D3D11_COUNTER_TYPE, active_counters: *mut UINT, sz_name: LPSTR, name_length: *mut UINT, sz_units: LPSTR, units_length: *mut UINT, sz_descrition: LPSTR, description_length: *mut UINT) -> HRESULT,
			fn CheckFeatureSupport(feature: D3D11_FEATURE, feature_support_data: *mut c_void, feature_suort_data_size: UINT) -> HRESULT,
			fn GetPrivateData(guid: REFGUID, data_size: *mut UINT, data: *mut c_void) -> HRESULT,
			fn SetPrivateData(guid: REFGUID, data_size: UINT, data: *const c_void) -> HRESULT,
			fn SetPrivateDataInterface(guid: REFGUID, data: *const IUnknown) -> HRESULT,
			fn GetFeatureLevel() -> D3D_FEATURE_LEVEL,
			fn GetCreationFlags() -> UINT,
			fn GetDeviceRemovedReason() -> HRESULT,
			fn GetImmediateContext(immediate_context: *mut *mut ID3D11DeviceContext) -> (),
			fn SetExceptionMode(raise_flags: UINT) -> HRESULT,
			fn GetExceptionMode() -> UINT,
		} with heirs [
			pub ID3D11Device1Vtbl of ID3D11Device1, pub trait ID3D11Device1T {
				fn GetImmediateContext1(ppImmediateContext: *mut *mut ID3D11DeviceContext1) -> (),
				fn CreateDeferredContext1(ContextFlags: UINT, ppDeferredContext: *mut *mut ID3D11DeviceContext1) -> HRESULT,
				fn CreateBlendState1(pBlendStateDesc: *const D3D11_BLEND_DESC1, ppBlendState: *mut *mut ID3D11BlendState1) -> HRESULT,
				fn CreateRasterizerState1(pRasterizerDesc: *const D3D11_RASTERIZER_DESC1, ppRasterizerState: *mut *mut ID3D11RasterizerState1) -> HRESULT,
				fn CreateDeviceContextState(Flags: UINT, pFeatureLevels: *const D3D_FEATURE_LEVEL, FeatureLevels: UINT, SDKVersion: UINT, EmulatedInterface: REFIID, pChosenFeatureLevel: *mut D3D_FEATURE_LEVEL, ppContextState: *mut *mut ID3DDeviceContextState) -> HRESULT,
				fn OpenSharedResource1(hResource: HANDLE, returnedInterface: REFIID, ppResource: *mut *mut c_void) -> HRESULT,
				fn OpenSharedResourceByName(lpName: LPCWSTR, dwDesiredAccess: DWORD, returnedInterface: REFIID, ppResource: *mut *mut c_void) -> HRESULT,
			} with heirs [
				pub ID3D11Device2Vtbl of ID3D11Device2, pub trait ID3D11Device2T {
					fn GetImmediateContext2(ppImmediateContext: *mut *mut ID3D11DeviceContext2) -> (),
					fn CreateDeferredContext2(ContextFlags: UINT, ppDeferredContext: *mut *mut ID3D11DeviceContext2) -> HRESULT,
					fn GetResourceTiling(pTiledResource: *mut ID3D11Resource, pNumTilesForEntireResource: *mut UINT, pPackedMipDesc: *mut D3D11_PACKED_MIP_DESC, pStandardTileShapeForNonPackedMips: *mut D3D11_TILE_SHAPE, pNumSubresourceTilings: *mut UINT, FirstSubresourceTilingToGet: UINT, pSubresourceTilingsForNonPackedMips: *mut D3D11_SUBRESOURCE_TILING) -> (),
					fn CheckMultisampleQualityLevels1(Format: DXGI_FORMAT,SampleCount: UINT, Flags: UINT, pNumQualityLevels: *mut UINT) -> HRESULT,
				}
			]
		]
		pub ID3D11DeviceContextVtbl of ID3D11DeviceContext, pub trait ID3D11DeviceContextT {
			fn VSSetConstantBuffers(StartSlot: UINT, NumBuffers: UINT, ppConstantBuffers: *const *mut ID3D11Buffer) -> (),
			fn PSSetShaderResources(StartSlot: UINT, NumViews: UINT, ppShaderResourceViews: *const *mut ID3D11ShaderResourceView) -> (),
			fn PSSetShader(pPixelShader: *mut ID3D11PixelShader, ppClassInstances: *const *mut ID3D11ClassInstance, NumClassInstances: UINT) -> (),
			fn PSSetSamplers(StartSlot: UINT, NumSamplers: UINT, ppSamplers: *const *mut ID3D11SamplerState) -> (),
			fn VSSetShader(pVertexShader: *mut ID3D11VertexShader, ppClassInstances: *const *mut ID3D11ClassInstance, NumClassInstances: UINT) -> (),
			fn DrawIndexed(IndexCount: UINT, StartIndexLocation: UINT, BaseVertexLocation: INT) -> (),
			fn Draw(VertexCount: UINT, StartVertexLocation: UINT) -> (),
			fn Map(pResource: *mut ID3D11Resource, Subresource: UINT, MapType: D3D11_MAP, MapFlags: UINT, pMappedResource: *mut D3D11_MAPPED_SUBRESOURCE) -> HRESULT,
			fn Unmap(pResource: *mut ID3D11Resource, Subresource: UINT) -> (),
			fn PSSetConstantBuffers(StartSlot: UINT, NumBuffers: UINT, ppConstantBuffers: *const *mut ID3D11Buffer) -> (),
			fn IASetInputLayout(pInputLayout: *mut ID3D11InputLayout) -> (),
			fn IASetVertexBuffers(StartSlot: UINT, NumBuffers: UINT, ppVertexBuffers: *const *mut ID3D11Buffer, pStrides: *const UINT, pOffsets: *const UINT) -> (),
			fn IASetIndexBuffer(pIndexBuffer: *mut ID3D11Buffer, Format: DXGI_FORMAT, Offset: UINT) -> (),
			fn DrawIndexedInstanced(IndexCountPerInstance: UINT, InstanceCount: UINT, StartIndexLocation: UINT, BaseVertexLocation: INT, StartInstanceLocation: UINT) -> (),
			fn DrawInstanced(VertexCountPerInstance: UINT, InstanceCount: UINT, StartVertexLocation: UINT, StartInstanceLocation: UINT) -> (),
			fn GSSetConstantBuffers(StartSlot: UINT, NumBuffers: UINT, ppConstantBuffers: *const *mut ID3D11Buffer) -> (),
			fn GSSetShader(pShader: *mut ID3D11GeometryShader, ppClassInstances: *const *mut ID3D11ClassInstance, NumClassInstances: UINT) -> (),
			fn IASetPrimitiveTopology(Topology: D3D11_PRIMITIVE_TOPOLOGY) -> (),
			fn VSSetShaderResources(StartSlot: UINT, NumViews: UINT, ppShaderResourceViews: *const *mut ID3D11ShaderResourceView) -> (),
			fn VSSetSamplers(StartSlot: UINT, NumSamplers: UINT, ppSamplers: *const *mut ID3D11SamplerState) -> (),
			fn Begin(pAsync: *mut ID3D11Asynchronous) -> (),
			fn End(pAsync: *mut ID3D11Asynchronous) -> (),
			fn GetData(pAsync: *mut ID3D11Asynchronous, pData: *mut c_void, DataSize: UINT, GetDataFlags: UINT) -> HRESULT,
			fn SetPredication(pPredicate: *mut ID3D11Predicate, PredicateValue: BOOL) -> (),
			fn GSSetShaderResources(StartSlot: UINT, NumViews: UINT, ppShaderResourceViews: *const *mut ID3D11ShaderResourceView) -> (),
			fn GSSetSamplers(StartSlot: UINT, NumSamplers: UINT, ppSamplers: *const *mut ID3D11SamplerState) -> (),
			fn OMSetRenderTargets(NumViews: UINT, ppRenderTargetViews: *const *mut ID3D11RenderTargetView, pDepthStencilView: *mut ID3D11DepthStencilView) -> (),
			fn OMSetRenderTargetsAndUnorderedAccessViews(NumRTVs: UINT, ppRenderTargetViews: *const *mut ID3D11RenderTargetView, pDepthStencilView: *mut ID3D11DepthStencilView, UAVStartSlot: UINT, NumUAVs: UINT, ppUnorderedAccessViews: *const *mut ID3D11UnorderedAccessView, pUAVInitialCounts: *const UINT) -> (),
			fn OMSetBlendState(pBlendState: *mut ID3D11BlendState, BlendFactor: [FLOAT; 4], SampleMask: UINT) -> (),
			fn OMSetDepthStencilState(pDepthStencilState: *mut ID3D11DepthStencilState, StencilRef: UINT) -> (),
			fn SOSetTargets(NumBuffers: UINT, ppSOTargets: *const *mut ID3D11Buffer, pOffsets: *const UINT) -> (),
			fn DrawAuto() -> (),
			fn DrawIndexedInstancedIndirect(pBufferForArgs: *mut ID3D11Buffer, AlignedByteOffsetForArgs: UINT) -> (),
			fn DrawInstancedIndirect(pBufferForArgs: *mut ID3D11Buffer, AlignedByteOffsetForArgs: UINT) -> (),
			fn Dispatch(ThreadGroupCountX: UINT, ThreadGroupCountY: UINT, ThreadGroupCountZ: UINT) -> (),
			fn DispatchIndirect(pBufferForArgs: *mut ID3D11Buffer, AlignedByteOffsetForArgs: UINT) -> (),
			fn RSSetState(pRasterizerState: *mut ID3D11RasterizerState) -> (),
			fn RSSetViewports(NumViewports: UINT, pViewports: *const D3D11_VIEWPORT) -> (),
			fn RSSetScissorRects(NumRects: UINT, pRects: *const D3D11_RECT) -> (),
			fn CopySubresourceRegion(pDstResource: *mut ID3D11Resource, DstSubresource: UINT, DstX: UINT, DstY: UINT, DstZ: UINT, pSrcResource: *mut ID3D11Resource, SrcSubresource: UINT, pSrcBox: *const D3D11_BOX) -> (),
			fn CopyResource(pDstResource: *mut ID3D11Resource, pSrcResource: *mut ID3D11Resource) -> (),
			fn UpdateSubresource(pDstResource: *mut ID3D11Resource, DstSubresource: UINT, pDstBox: *const D3D11_BOX, pSrcData: *const c_void, SrcRowPitch: UINT, SrcDepthPitch: UINT) -> (),
			fn CopyStructureCount(pDstBuffer: *mut ID3D11Buffer, DstAlignedByteOffset: UINT, pSrcView: *mut ID3D11UnorderedAccessView) -> (),
			fn ClearRenderTargetView(pRenderTargetView: *mut ID3D11RenderTargetView, ColorRGBA: [FLOAT; 4]) -> (),
			fn ClearUnorderedAccessViewUint(pUnorderedAccessView: *mut ID3D11UnorderedAccessView, Values: [UINT; 4]) -> (),
			fn ClearUnorderedAccessViewFloat(pUnorderedAccessView: *mut ID3D11UnorderedAccessView, Values: [FLOAT; 4]) -> (),
			fn ClearDepthStencilView(pDepthStencilView: *mut ID3D11DepthStencilView, ClearFlags: UINT, Depth: FLOAT, Stencil: UINT8) -> (),
			fn GenerateMips(pShaderResourceView: *mut ID3D11ShaderResourceView) -> (),
			fn SetResourceMinLOD(pResource: *mut ID3D11Resource, MinLOD: FLOAT) -> (),
			fn GetResourceMinLOD(pResource: *mut ID3D11Resource) -> FLOAT,
			fn ResolveSubresource(pDstResource: *mut ID3D11Resource, DstSubresource: UINT, pSrcResource: *mut ID3D11Resource, SrcSubresource: UINT, Format: DXGI_FORMAT) -> (),
			fn ExecuteCommandList(pCommandList: *mut ID3D11CommandList, RestoreContextState: BOOL) -> (),
			fn HSSetShaderResources(StartSlot: UINT, NumViews: UINT, ppShaderResourceViews: *const *mut ID3D11ShaderResourceView) -> (),
			fn HSSetShader(pHullShader: *mut ID3D11HullShader, ppClassInstances: *const *mut ID3D11ClassInstance, NumClassInstances: UINT) -> (),
			fn HSSetSamplers(StartSlot: UINT, NumSamplers: UINT, ppSamplers: *const *mut ID3D11SamplerState) -> (),
			fn HSSetConstantBuffers(StartSlot: UINT, NumBuffers: UINT, ppConstantBuffers: *const *mut ID3D11Buffer) -> (),
			fn DSSetShaderResources(StartSlot: UINT, NumViews: UINT, ppShaderResourceViews: *const *mut ID3D11ShaderResourceView) -> (),
			fn DSSetShader(pDomainShader: *mut ID3D11DomainShader, ppClassInstances: *const *mut ID3D11ClassInstance, NumClassInstances: UINT) -> (),
			fn DSSetSamplers(StartSlot: UINT, NumSamplers: UINT, ppSamplers: *const *mut ID3D11SamplerState) -> (),
			fn DSSetConstantBuffers(StartSlot: UINT, NumBuffers: UINT, ppConstantBuffers: *const *mut ID3D11Buffer) -> (),
			fn CSSetShaderResources(StartSlot: UINT, NumViews: UINT, ppShaderResourceViews: *const *mut ID3D11ShaderResourceView) -> (),
			fn CSSetUnorderedAccessViews(StartSlot: UINT, NumUAVs: UINT, ppUnorderedAccessViews: *const *mut ID3D11UnorderedAccessView, pUAVInitialCounts: *const UINT) -> (),
			fn CSSetShader(pComputeShader: *mut ID3D11ComputeShader, ppClassInstances: *const *mut ID3D11ClassInstance, NumClassInstances: UINT) -> (),
			fn CSSetSamplers(StartSlot: UINT, NumSamplers: UINT, ppSamplers: *const *mut ID3D11SamplerState) -> (),
			fn CSSetConstantBuffers(StartSlot: UINT, NumBuffers: UINT, ppConstantBuffers: *const *mut ID3D11Buffer) -> (),
			fn VSGetConstantBuffers(StartSlot: UINT, NumBuffers: UINT, ppConstantBuffers: *mut *mut ID3D11Buffer) -> (),
			fn PSGetShaderResources(StartSlot: UINT, NumViews: UINT, ppShaderResourceViews: *mut *mut ID3D11ShaderResourceView) -> (),
			fn PSGetShader(ppPixelShader: *mut *mut ID3D11PixelShader, ppClassInstances: *mut *mut ID3D11ClassInstance, pNumClassInstances: *mut UINT) -> (),
			fn PSGetSamplers(StartSlot: UINT, NumSamplers: UINT, ppSamplers: *mut *mut ID3D11SamplerState) -> (),
			fn VSGetShader(ppVertexShader: *mut *mut ID3D11VertexShader, ppClassInstances: *mut *mut ID3D11ClassInstance, pNumClassInstances: *mut UINT) -> (),
			fn PSGetConstantBuffers(StartSlot: UINT, NumBuffers: UINT, ppConstantBuffers: *mut *mut ID3D11Buffer) -> (),
			fn IAGetInputLayout(ppInputLayout: *mut *mut ID3D11InputLayout) -> (),
			fn IAGetVertexBuffers(StartSlot: UINT, NumBuffers: UINT, ppVertexBuffers: *mut *mut ID3D11Buffer, pStrides: *mut UINT, pOffsets: *mut UINT) -> (),
			fn IAGetIndexBuffer(pIndexBuffer: *mut *mut ID3D11Buffer, Format: *mut DXGI_FORMAT, Offset: *mut UINT) -> (),
			fn GSGetConstantBuffers(StartSlot: UINT, NumBuffers: UINT, ppConstantBuffers: *mut *mut ID3D11Buffer) -> (),
			fn GSGetShader(ppGeometryShader: *mut *mut ID3D11GeometryShader, ppClassInstances: *mut *mut ID3D11ClassInstance, pNumClassInstances: *mut UINT) -> (),
			fn IAGetPrimitiveTopology(pTopology: *mut D3D11_PRIMITIVE_TOPOLOGY) -> (),
			fn VSGetShaderResources(StartSlot: UINT, NumViews: UINT, ppShaderResourceViews: *mut *mut ID3D11ShaderResourceView) -> (),
			fn VSGetSamplers(StartSlot: UINT, NumSamplers: UINT, ppSamplers: *mut *mut ID3D11SamplerState) -> (),
			fn GetPredication(ppPredicate: *mut *mut ID3D11Predicate, pPredicateValue: *mut BOOL) -> (),
			fn GSGetShaderResources(StartSlot: UINT, NumViews: UINT, ppShaderResourceViews: *mut *mut ID3D11ShaderResourceView) -> (),
			fn GSGetSamplers(StartSlot: UINT, NumSamplers: UINT, ppSamplers: *mut *mut ID3D11SamplerState) -> (),
			fn OMGetRenderTargets(NumViews: UINT, ppRenderTargetViews: *mut *mut ID3D11RenderTargetView, ppDepthStencilView: *mut *mut ID3D11DepthStencilView) -> (),
			fn OMGetRenderTargetsAndUnorderedAccessViews(NumRTVs: UINT, ppRenderTargetViews: *mut *mut ID3D11RenderTargetView, ppDepthStencilView: *mut *mut ID3D11DepthStencilView, UAVStartSlot: UINT, NumUAVs: UINT, ppUnorderedAccessViews: *mut *mut ID3D11UnorderedAccessView) -> (),
			fn OMGetBlendState(ppBlendState: *mut *mut ID3D11BlendState, BlendFactor: [FLOAT; 4], pSampleMask: *mut UINT) -> (),
			fn OMGetDepthStencilState(ppDepthStencilState: *mut *mut ID3D11DepthStencilState, pStencilRef: *mut UINT) -> (),
			fn SOGetTargets(NumBuffers: UINT, ppSOTargets: *mut *mut ID3D11Buffer) -> (),
			fn RSGetState(ppRasterizerState: *mut *mut ID3D11RasterizerState) -> (),
			fn RSGetViewports(pNumViewports: *mut UINT, pViewports: *mut D3D11_VIEWPORT) -> (),
			fn RSGetScissorRects(pNumRects: *mut UINT, pRects: *mut D3D11_RECT) -> (),
			fn HSGetShaderResources(StartSlot: UINT, NumViews: UINT, ppShaderResourceViews: *mut *mut ID3D11ShaderResourceView) -> (),
			fn HSGetShader(ppHullShader: *mut *mut ID3D11HullShader, ppClassInstances: *mut *mut ID3D11ClassInstance, pNumClassInstances: *mut UINT) -> (),
			fn HSGetSamplers(StartSlot: UINT, NumSamplers: UINT, ppSamplers: *mut *mut ID3D11SamplerState) -> (),
			fn HSGetConstantBuffers(StartSlot: UINT, NumBuffers: UINT, ppConstantBuffers: *mut *mut ID3D11Buffer) -> (),
			fn DSGetShaderResources(StartSlot: UINT, NumViews: UINT, ppShaderResourceViews: *mut *mut ID3D11ShaderResourceView) -> (),
			fn DSGetShader(ppDomainShader: *mut *mut ID3D11DomainShader, ppClassInstances: *mut *mut ID3D11ClassInstance, pNumClassInstances: *mut UINT) -> (),
			fn DSGetSamplers(StartSlot: UINT, NumSamplers: UINT, ppSamplers: *mut *mut ID3D11SamplerState) -> (),
			fn DSGetConstantBuffers(StartSlot: UINT, NumBuffers: UINT, ppConstantBuffers: *mut *mut ID3D11Buffer) -> (),
			fn CSGetShaderResources(StartSlot: UINT, NumViews: UINT, ppShaderResourceViews: *mut *mut ID3D11ShaderResourceView) -> (),
			fn CSGetUnorderedAccessViews(StartSlot: UINT, NumUAVs: UINT, ppUnorderedAccessViews: *mut *mut ID3D11UnorderedAccessView) -> (),
			fn CSGetShader(ppComputeShader: *mut *mut ID3D11ComputeShader, ppClassInstances: *mut *mut ID3D11ClassInstance, pNumClassInstances: *mut UINT) -> (),
			fn CSGetSamplers(StartSlot: UINT, NumSamplers: UINT, ppSamplers: *mut *mut ID3D11SamplerState) -> (),
			fn CSGetConstantBuffers(StartSlot: UINT, NumBuffers: UINT, ppConstantBuffers: *mut *mut ID3D11Buffer) -> (),
			fn ClearState() -> (),
			fn Flush() -> (),
			fn GetType() -> D3D11_DEVICE_CONTEXT_TYPE,
			fn GetContextFlags() -> UINT,
			fn FinishCommandList(RestoreDeferredContextState: BOOL, ppCommandList: *mut *mut ID3D11CommandList) -> HRESULT,
		} with heirs [
			pub ID3D11DeviceContext1Vtbl of ID3D11DeviceContext1, pub trait ID3D11DeviceContext1T {
				fn CopySubresourceRegion1(pDstResource: *mut ID3D11Resource, DstSubresource: UINT, DstX: UINT, DstY: UINT, DstZ: UINT, pSrcResource: *mut ID3D11Resource, SrcSubresource: UINT, pSrcBox: *const D3D11_BOX, CopyFlags: UINT) -> (),
				fn UpdateSubresource1(pDstResource: *mut ID3D11Resource, DstSubresource: UINT, pDstBox: *const D3D11_BOX, pSrcData: *const c_void, SrcRowPitch: UINT, SrcDepthPitch: UINT, CopyFlags: UINT) -> (),
				fn DiscardResource(pResource: *mut ID3D11Resource) -> (),
				fn DiscardView(pResourceView: *mut ID3D11View) -> (),
				fn VSSetConstantBuffers1(StartSlot: UINT, NumBuffers: UINT, ppConstantBuffers: *const *mut ID3D11Buffer, pFirstConstant: *const UINT, pNumConstants: *const UINT) -> (),
				fn HSSetConstantBuffers1(StartSlot: UINT, NumBuffers: UINT, ppConstantBuffers: *const *mut ID3D11Buffer, pFirstConstant: *const UINT, pNumConstants: *const UINT) -> (),
				fn DSSetConstantBuffers1(StartSlot: UINT, NumBuffers: UINT, ppConstantBuffers: *const *mut ID3D11Buffer, pFirstConstant: *const UINT, pNumConstants: *const UINT) -> (),
				fn GSSetConstantBuffers1(StartSlot: UINT, NumBuffers: UINT, ppConstantBuffers: *const *mut ID3D11Buffer, pFirstConstant: *const UINT, pNumConstants: *const UINT) -> (),
				fn PSSetConstantBuffers1(StartSlot: UINT, NumBuffers: UINT, ppConstantBuffers: *const *mut ID3D11Buffer, pFirstConstant: *const UINT, pNumConstants: *const UINT) -> (),
				fn CSSetConstantBuffers1(StartSlot: UINT, NumBuffers: UINT, ppConstantBuffers: *const *mut ID3D11Buffer, pFirstConstant: *const UINT, pNumConstants: *const UINT) -> (),
				fn VSGetConstantBuffers1(StartSlot: UINT, NumBuffers: UINT, ppConstantBuffers: *mut *mut ID3D11Buffer, pFirstConstant: *mut UINT, pNumConstants: *mut UINT) -> (),
				fn HSGetConstantBuffers1(StartSlot: UINT, NumBuffers: UINT, ppConstantBuffers: *mut *mut ID3D11Buffer, pFirstConstant: *mut UINT, pNumConstants: *mut UINT) -> (),
				fn DSGetConstantBuffers1(StartSlot: UINT, NumBuffers: UINT, ppConstantBuffers: *mut *mut ID3D11Buffer, pFirstConstant: *mut UINT, pNumConstants: *mut UINT) -> (),
				fn GSGetConstantBuffers1(StartSlot: UINT, NumBuffers: UINT, ppConstantBuffers: *mut *mut ID3D11Buffer, pFirstConstant: *mut UINT, pNumConstants: *mut UINT) -> (),
				fn PSGetConstantBuffers1(StartSlot: UINT, NumBuffers: UINT, ppConstantBuffers: *mut *mut ID3D11Buffer, pFirstConstant: *mut UINT, pNumConstants: *mut UINT) -> (),
				fn CSGetConstantBuffers1(StartSlot: UINT, NumBuffers: UINT, ppConstantBuffers: *mut *mut ID3D11Buffer, pFirstConstant: *mut UINT, pNumConstants: *mut UINT) -> (),
				fn SwapDeviceContextState(pState: *mut ID3DDeviceContextState, ppPreviousState: *mut *mut ID3DDeviceContextState) -> (),
				fn ClearView(pView: *mut ID3D11View, Color: [FLOAT; 4], pRect: *const D3D11_RECT, NumRects: UINT) -> (),
				fn DiscardView1(pResourceView: *mut ID3D11View, pRects: *const D3D11_RECT, NumRects: UINT) -> (),
			} with heirs [
				pub ID3D11DeviceContext2Vtbl of ID3D11DeviceContext2, pub trait ID3D11DeviceContext2T {
					fn UpdateTileMappings(pTiledResource: *mut ID3D11Resource, NumTiledResourceRegions: UINT, pTiledResourceRegionStartCoordinates: *const D3D11_TILED_RESOURCE_COORDINATE, pTiledResourceRegionSizes: *const D3D11_TILE_REGION_SIZE, pTilePool: *mut ID3D11Buffer, NumRanges: UINT, pRangeFlags: *const UINT, pTilePoolStartOffsets: *const UINT, pRangeTileCounts: *const UINT, Flags: UINT) -> HRESULT,
					fn CopyTileMappings(pDestTiledResource: *mut ID3D11Resource, pDestRegionStartCoordinate: *const D3D11_TILED_RESOURCE_COORDINATE, pSourceTiledResource: *mut ID3D11Resource, pSourceRegionStartCoordinate: *const D3D11_TILED_RESOURCE_COORDINATE, pTileRegionSize: *const D3D11_TILE_REGION_SIZE, Flags: UINT) -> HRESULT,
					fn CopyTiles(pTiledResource: *mut ID3D11Resource, pTileRegionStartCoordinate: *const D3D11_TILED_RESOURCE_COORDINATE, pTileRegionSize: *const D3D11_TILE_REGION_SIZE, pBuffer: *mut ID3D11Buffer, BufferStartOffsetInBytes: UINT64, Flags: UINT) -> (),
					fn UpdateTiles(pDestTiledResource: *mut ID3D11Resource, pDestTileRegionStartCoordinate: *const D3D11_TILED_RESOURCE_COORDINATE, pDestTileRegionSize: *const D3D11_TILE_REGION_SIZE, pSourceTileData: *const c_void, Flags: UINT) -> (),
					fn ResizeTilePool(pTilePool: *mut ID3D11Buffer, NewSizeInBytes: UINT64) -> HRESULT,
					fn TiledResourceBarrier(pTiledResourceOrViewAccessBeforeBarrier: *mut ID3D11DeviceChild, pTiledResourceOrViewAccessAfterBarrier: *mut ID3D11DeviceChild) -> (),
					fn IsAnnotationEnabled() -> BOOL,
					fn SetMarkerInt(pLabel: LPCWSTR, Data: INT) -> (),
					fn BeginEventInt(pLabel: LPCWSTR, Data: INT) -> (),
					fn EndEvent() -> (),
				}
			]
		]
		pub ID3DDeviceContextStateVtbl of ID3DDeviceContextState, pub trait ID3DDeviceContextStateT { }
		pub ID3D11InputLayoutVtbl of ID3D11InputLayout, pub trait ID3D11InputLayoutT { }
		pub ID3D11RasterizerStateVtbl of ID3D11RasterizerState, pub trait ID3D11RasterizerStateT {
			fn GetDesc(desc: *mut D3D11_RASTERIZER_DESC) -> (),
		} with heirs [
			pub ID3D11RasterizerState1Vtbl of ID3D11RasterizerState1, pub trait ID3D11RasterizerState1T {
				fn GetDesc1(desc: *mut D3D11_RASTERIZER_DESC1) -> (),
			}
		]
		pub ID3D11SamplerStateVtbl of ID3D11SamplerState, pub trait ID3D11SamplerStateT {
			fn GetDesc(desc: *mut D3D11_SAMPLER_DESC) -> (),
		}
	]
}