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

#![allow(non_snake_case)]

use winapi::minwindef::*;
use winapi::basetsd::*;
use winapi::{ REFGUID, HRESULT, GUID, REFIID, HANDLE, LPSTR, LPCWSTR, c_void };
use dxgi::{ DXGI_FORMAT, QueryIID, IUnknown, IUnknownVtbl, COMInterface };

use constants::*;
use core::enumerations::*;
use core::structures::*;
use common_version::enumerations::D3D_FEATURE_LEVEL;
use resource::enumerations::D3D11_MAP;
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

com_interface!{ ID3D11DeviceChild(ID3D11DeviceChildVtbl): IUnknown(IUnknownVtbl) {
	fn GetDevice(&mut self, device: *mut *mut ID3D11Device) -> (),
	fn GetPrivateData(&mut self, guid: REFGUID, data_size: *mut UINT, data: *mut c_void) -> HRESULT,
	fn SetPrivateData(&mut self, guid: REFGUID, data_size: UINT, data: *const c_void) -> HRESULT,
	fn SetPrivateDataInterface(&mut self, guid: REFGUID, data: *const IUnknown) -> HRESULT
}}
com_interface!{ ID3D11Asynchronous(ID3D11AsynchronousVtbl): ID3D11DeviceChild(ID3D11DeviceChildVtbl) {
	fn GetDataSize(&mut self) -> UINT
}}
com_interface!{ ID3D11Counter(ID3D11CounterVtbl): ID3D11Asynchronous(ID3D11AsynchronousVtbl) {
	fn GetDesc(&mut self, desc: *mut D3D11_COUNTER_DESC) -> ()
}}
com_interface!{ ID3D11Query(ID3D11QueryVtbl): ID3D11Asynchronous(ID3D11AsynchronousVtbl) {
	fn GetDesc(&mut self, desc: *mut D3D11_QUERY_DESC) -> ()
}}
com_interface!{ ID3D11Predicate(ID3D11PredicateVtbl): ID3D11Query(ID3D11QueryVtbl) { } }
com_interface!{ ID3D11BlendState(ID3D11BlendStateVtbl): ID3D11DeviceChild(ID3D11DeviceChildVtbl) {
	fn GetDesc(&mut self, desc: *mut D3D11_BLEND_DESC) -> ()
}}
com_interface!{ ID3D11BlendState1(ID3D11BlendState1Vtbl): ID3D11BlendState(ID3D11BlendStateVtbl) {
	fn GetDesc1(&mut self, desc: *mut D3D11_BLEND_DESC1) -> ()
}}
com_interface!{ ID3D11CommandList(ID3D11CommandListVtbl): ID3D11DeviceChild(ID3D11DeviceChildVtbl) {
	fn GetContextFlags(&mut self) -> UINT
}}
com_interface!{ ID3D11DepthStencilState(ID3D11DepthStencilStateVtbl): ID3D11DeviceChild(ID3D11DeviceChildVtbl) {
	fn GetDesc(&mut self, desc: *mut D3D11_DEPTH_STENCIL_DESC) -> ()
}}
com_interface!{ ID3D11DeviceContext(ID3D11DeviceContextVtbl): ID3D11DeviceChild(ID3D11DeviceChildVtbl) {
	fn VSSetConstantBuffers(&mut self, StartSlot: UINT, NumBuffers: UINT, ppConstantBuffers: *const *mut ID3D11Buffer) -> (),
	fn PSSetShaderResources(&mut self, StartSlot: UINT, NumViews: UINT, ppShaderResourceViews: *const *mut ID3D11ShaderResourceView) -> (),
	fn PSSetShader(&mut self, pPixelShader: *mut ID3D11PixelShader, ppClassInstances: *const *mut ID3D11ClassInstance, NumClassInstances: UINT) -> (),
	fn PSSetSamplers(&mut self, StartSlot: UINT, NumSamplers: UINT, ppSamplers: *const *mut ID3D11SamplerState) -> (),
	fn VSSetShader(&mut self, pVertexShader: *mut ID3D11VertexShader, ppClassInstances: *const *mut ID3D11ClassInstance, NumClassInstances: UINT) -> (),
	fn DrawIndexed(&mut self, IndexCount: UINT, StartIndexLocation: UINT, BaseVertexLocation: INT) -> (),
	fn Draw(&mut self, VertexCount: UINT, StartVertexLocation: UINT) -> (),
	fn Map(&mut self, pResource: *mut ID3D11Resource, Subresource: UINT, MapType: D3D11_MAP, MapFlags: UINT, pMappedResource: *mut D3D11_MAPPED_SUBRESOURCE) -> HRESULT,
	fn Unmap(&mut self, pResource: *mut ID3D11Resource, Subresource: UINT) -> (),
	fn PSSetConstantBuffers(&mut self, StartSlot: UINT, NumBuffers: UINT, ppConstantBuffers: *const *mut ID3D11Buffer) -> (),
	fn IASetInputLayout(&mut self, pInputLayout: *mut ID3D11InputLayout) -> (),
	fn IASetVertexBuffers(&mut self, StartSlot: UINT, NumBuffers: UINT, ppVertexBuffers: *const *mut ID3D11Buffer, pStrides: *const UINT, pOffsets: *const UINT) -> (),
	fn IASetIndexBuffer(&mut self, pIndexBuffer: *mut ID3D11Buffer, Format: DXGI_FORMAT, Offset: UINT) -> (),
	fn DrawIndexedInstanced(&mut self, IndexCountPerInstance: UINT, InstanceCount: UINT, StartIndexLocation: UINT, BaseVertexLocation: INT, StartInstanceLocation: UINT) -> (),
	fn DrawInstanced(&mut self, VertexCountPerInstance: UINT, InstanceCount: UINT, StartVertexLocation: UINT, StartInstanceLocation: UINT) -> (),
	fn GSSetConstantBuffers(&mut self, StartSlot: UINT, NumBuffers: UINT, ppConstantBuffers: *const *mut ID3D11Buffer) -> (),
	fn GSSetShader(&mut self, pShader: *mut ID3D11GeometryShader, ppClassInstances: *const *mut ID3D11ClassInstance, NumClassInstances: UINT) -> (),
	fn IASetPrimitiveTopology(&mut self, Topology: D3D11_PRIMITIVE_TOPOLOGY) -> (),
	fn VSSetShaderResources(&mut self, StartSlot: UINT, NumViews: UINT, ppShaderResourceViews: *const *mut ID3D11ShaderResourceView) -> (),
	fn VSSetSamplers(&mut self, StartSlot: UINT, NumSamplers: UINT, ppSamplers: *const *mut ID3D11SamplerState) -> (),
	fn Begin(&mut self, pAsync: *mut ID3D11Asynchronous) -> (),
	fn End(&mut self, pAsync: *mut ID3D11Asynchronous) -> (),
	fn GetData(&mut self, pAsync: *mut ID3D11Asynchronous, pData: *mut c_void, DataSize: UINT, GetDataFlags: UINT) -> HRESULT,
	fn SetPredication(&mut self, pPredicate: *mut ID3D11Predicate, PredicateValue: BOOL) -> (),
	fn GSSetShaderResources(&mut self, StartSlot: UINT, NumViews: UINT, ppShaderResourceViews: *const *mut ID3D11ShaderResourceView) -> (),
	fn GSSetSamplers(&mut self, StartSlot: UINT, NumSamplers: UINT, ppSamplers: *const *mut ID3D11SamplerState) -> (),
	fn OMSetRenderTargets(&mut self, NumViews: UINT, ppRenderTargetViews: *const *mut ID3D11RenderTargetView, pDepthStencilView: *mut ID3D11DepthStencilView) -> (),
	fn OMSetRenderTargetsAndUnorderedAccessViews(&mut self, NumRTVs: UINT, ppRenderTargetViews: *const *mut ID3D11RenderTargetView, pDepthStencilView: *mut ID3D11DepthStencilView, UAVStartSlot: UINT, NumUAVs: UINT, ppUnorderedAccessViews: *const *mut ID3D11UnorderedAccessView, pUAVInitialCounts: *const UINT) -> (),
	fn OMSetBlendState(&mut self, pBlendState: *mut ID3D11BlendState, BlendFactor: [FLOAT; 4], SampleMask: UINT) -> (),
	fn OMSetDepthStencilState(&mut self, pDepthStencilState: *mut ID3D11DepthStencilState, StencilRef: UINT) -> (),
	fn SOSetTargets(&mut self, NumBuffers: UINT, ppSOTargets: *const *mut ID3D11Buffer, pOffsets: *const UINT) -> (),
	fn DrawAuto(&mut self) -> (),
	fn DrawIndexedInstancedIndirect(&mut self, pBufferForArgs: *mut ID3D11Buffer, AlignedByteOffsetForArgs: UINT) -> (),
	fn DrawInstancedIndirect(&mut self, pBufferForArgs: *mut ID3D11Buffer, AlignedByteOffsetForArgs: UINT) -> (),
	fn Dispatch(&mut self, ThreadGroupCountX: UINT, ThreadGroupCountY: UINT, ThreadGroupCountZ: UINT) -> (),
	fn DispatchIndirect(&mut self, pBufferForArgs: *mut ID3D11Buffer, AlignedByteOffsetForArgs: UINT) -> (),
	fn RSSetState(&mut self, pRasterizerState: *mut ID3D11RasterizerState) -> (),
	fn RSSetViewports(&mut self, NumViewports: UINT, pViewports: *const D3D11_VIEWPORT) -> (),
	fn RSSetScissorRects(&mut self, NumRects: UINT, pRects: *const D3D11_RECT) -> (),
	fn CopySubresourceRegion(&mut self, pDstResource: *mut ID3D11Resource, DstSubresource: UINT, DstX: UINT, DstY: UINT, DstZ: UINT, pSrcResource: *mut ID3D11Resource, SrcSubresource: UINT, pSrcBox: *const D3D11_BOX) -> (),
	fn CopyResource(&mut self, pDstResource: *mut ID3D11Resource, pSrcResource: *mut ID3D11Resource) -> (),
	fn UpdateSubresource(&mut self, pDstResource: *mut ID3D11Resource, DstSubresource: UINT, pDstBox: *const D3D11_BOX, pSrcData: *const c_void, SrcRowPitch: UINT, SrcDepthPitch: UINT) -> (),
	fn CopyStructureCount(&mut self, pDstBuffer: *mut ID3D11Buffer, DstAlignedByteOffset: UINT, pSrcView: *mut ID3D11UnorderedAccessView) -> (),
	fn ClearRenderTargetView(&mut self, pRenderTargetView: *mut ID3D11RenderTargetView, ColorRGBA: [FLOAT; 4]) -> (),
	fn ClearUnorderedAccessViewUint(&mut self, pUnorderedAccessView: *mut ID3D11UnorderedAccessView, Values: [UINT; 4]) -> (),
	fn ClearUnorderedAccessViewFloat(&mut self, pUnorderedAccessView: *mut ID3D11UnorderedAccessView, Values: [FLOAT; 4]) -> (),
	fn ClearDepthStencilView(&mut self, pDepthStencilView: *mut ID3D11DepthStencilView, ClearFlags: UINT, Depth: FLOAT, Stencil: UINT8) -> (),
	fn GenerateMips(&mut self, pShaderResourceView: *mut ID3D11ShaderResourceView) -> (),
	fn SetResourceMinLOD(&mut self, pResource: *mut ID3D11Resource, MinLOD: FLOAT) -> (),
	fn GetResourceMinLOD(&mut self, pResource: *mut ID3D11Resource) -> FLOAT,
	fn ResolveSubresource(&mut self, pDstResource: *mut ID3D11Resource, DstSubresource: UINT, pSrcResource: *mut ID3D11Resource, SrcSubresource: UINT, Format: DXGI_FORMAT) -> (),
	fn ExecuteCommandList(&mut self, pCommandList: *mut ID3D11CommandList, RestoreContextState: BOOL) -> (),
	fn HSSetShaderResources(&mut self, StartSlot: UINT, NumViews: UINT, ppShaderResourceViews: *const *mut ID3D11ShaderResourceView) -> (),
	fn HSSetShader(&mut self, pHullShader: *mut ID3D11HullShader, ppClassInstances: *const *mut ID3D11ClassInstance, NumClassInstances: UINT) -> (),
	fn HSSetSamplers(&mut self, StartSlot: UINT, NumSamplers: UINT, ppSamplers: *const *mut ID3D11SamplerState) -> (),
	fn HSSetConstantBuffers(&mut self, StartSlot: UINT, NumBuffers: UINT, ppConstantBuffers: *const *mut ID3D11Buffer) -> (),
	fn DSSetShaderResources(&mut self, StartSlot: UINT, NumViews: UINT, ppShaderResourceViews: *const *mut ID3D11ShaderResourceView) -> (),
	fn DSSetShader(&mut self, pDomainShader: *mut ID3D11DomainShader, ppClassInstances: *const *mut ID3D11ClassInstance, NumClassInstances: UINT) -> (),
	fn DSSetSamplers(&mut self, StartSlot: UINT, NumSamplers: UINT, ppSamplers: *const *mut ID3D11SamplerState) -> (),
	fn DSSetConstantBuffers(&mut self, StartSlot: UINT, NumBuffers: UINT, ppConstantBuffers: *const *mut ID3D11Buffer) -> (),
	fn CSSetShaderResources(&mut self, StartSlot: UINT, NumViews: UINT, ppShaderResourceViews: *const *mut ID3D11ShaderResourceView) -> (),
	fn CSSetUnorderedAccessViews(&mut self, StartSlot: UINT, NumUAVs: UINT, ppUnorderedAccessViews: *const *mut ID3D11UnorderedAccessView, pUAVInitialCounts: *const UINT) -> (),
	fn CSSetShader(&mut self, pComputeShader: *mut ID3D11ComputeShader, ppClassInstances: *const *mut ID3D11ClassInstance, NumClassInstances: UINT) -> (),
	fn CSSetSamplers(&mut self, StartSlot: UINT, NumSamplers: UINT, ppSamplers: *const *mut ID3D11SamplerState) -> (),
	fn CSSetConstantBuffers(&mut self, StartSlot: UINT, NumBuffers: UINT, ppConstantBuffers: *const *mut ID3D11Buffer) -> (),
	fn VSGetConstantBuffers(&mut self, StartSlot: UINT, NumBuffers: UINT, ppConstantBuffers: *mut *mut ID3D11Buffer) -> (),
	fn PSGetShaderResources(&mut self, StartSlot: UINT, NumViews: UINT, ppShaderResourceViews: *mut *mut ID3D11ShaderResourceView) -> (),
	fn PSGetShader(&mut self, ppPixelShader: *mut *mut ID3D11PixelShader, ppClassInstances: *mut *mut ID3D11ClassInstance, pNumClassInstances: *mut UINT) -> (),
	fn PSGetSamplers(&mut self, StartSlot: UINT, NumSamplers: UINT, ppSamplers: *mut *mut ID3D11SamplerState) -> (),
	fn VSGetShader(&mut self, ppVertexShader: *mut *mut ID3D11VertexShader, ppClassInstances: *mut *mut ID3D11ClassInstance, pNumClassInstances: *mut UINT) -> (),
	fn PSGetConstantBuffers(&mut self, StartSlot: UINT, NumBuffers: UINT, ppConstantBuffers: *mut *mut ID3D11Buffer) -> (),
	fn IAGetInputLayout(&mut self, ppInputLayout: *mut *mut ID3D11InputLayout) -> (),
	fn IAGetVertexBuffers(&mut self, StartSlot: UINT, NumBuffers: UINT, ppVertexBuffers: *mut *mut ID3D11Buffer, pStrides: *mut UINT, pOffsets: *mut UINT) -> (),
	fn IAGetIndexBuffer(&mut self, pIndexBuffer: *mut *mut ID3D11Buffer, Format: *mut DXGI_FORMAT, Offset: *mut UINT) -> (),
	fn GSGetConstantBuffers(&mut self, StartSlot: UINT, NumBuffers: UINT, ppConstantBuffers: *mut *mut ID3D11Buffer) -> (),
	fn GSGetShader(&mut self, ppGeometryShader: *mut *mut ID3D11GeometryShader, ppClassInstances: *mut *mut ID3D11ClassInstance, pNumClassInstances: *mut UINT) -> (),
	fn IAGetPrimitiveTopology(&mut self, pTopology: *mut D3D11_PRIMITIVE_TOPOLOGY) -> (),
	fn VSGetShaderResources(&mut self, StartSlot: UINT, NumViews: UINT, ppShaderResourceViews: *mut *mut ID3D11ShaderResourceView) -> (),
	fn VSGetSamplers(&mut self, StartSlot: UINT, NumSamplers: UINT, ppSamplers: *mut *mut ID3D11SamplerState) -> (),
	fn GetPredication(&mut self, ppPredicate: *mut *mut ID3D11Predicate, pPredicateValue: *mut BOOL) -> (),
	fn GSGetShaderResources(&mut self, StartSlot: UINT, NumViews: UINT, ppShaderResourceViews: *mut *mut ID3D11ShaderResourceView) -> (),
	fn GSGetSamplers(&mut self, StartSlot: UINT, NumSamplers: UINT, ppSamplers: *mut *mut ID3D11SamplerState) -> (),
	fn OMGetRenderTargets(&mut self, NumViews: UINT, ppRenderTargetViews: *mut *mut ID3D11RenderTargetView, ppDepthStencilView: *mut *mut ID3D11DepthStencilView) -> (),
	fn OMGetRenderTargetsAndUnorderedAccessViews(&mut self, NumRTVs: UINT, ppRenderTargetViews: *mut *mut ID3D11RenderTargetView, ppDepthStencilView: *mut *mut ID3D11DepthStencilView, UAVStartSlot: UINT, NumUAVs: UINT, ppUnorderedAccessViews: *mut *mut ID3D11UnorderedAccessView) -> (),
	fn OMGetBlendState(&mut self, ppBlendState: *mut *mut ID3D11BlendState, BlendFactor: [FLOAT; 4], pSampleMask: *mut UINT) -> (),
	fn OMGetDepthStencilState(&mut self, ppDepthStencilState: *mut *mut ID3D11DepthStencilState, pStencilRef: *mut UINT) -> (),
	fn SOGetTargets(&mut self, NumBuffers: UINT, ppSOTargets: *mut *mut ID3D11Buffer) -> (),
	fn RSGetState(&mut self, ppRasterizerState: *mut *mut ID3D11RasterizerState) -> (),
	fn RSGetViewports(&mut self, pNumViewports: *mut UINT, pViewports: *mut D3D11_VIEWPORT) -> (),
	fn RSGetScissorRects(&mut self, pNumRects: *mut UINT, pRects: *mut D3D11_RECT) -> (),
	fn HSGetShaderResources(&mut self, StartSlot: UINT, NumViews: UINT, ppShaderResourceViews: *mut *mut ID3D11ShaderResourceView) -> (),
	fn HSGetShader(&mut self, ppHullShader: *mut *mut ID3D11HullShader, ppClassInstances: *mut *mut ID3D11ClassInstance, pNumClassInstances: *mut UINT) -> (),
	fn HSGetSamplers(&mut self, StartSlot: UINT, NumSamplers: UINT, ppSamplers: *mut *mut ID3D11SamplerState) -> (),
	fn HSGetConstantBuffers(&mut self, StartSlot: UINT, NumBuffers: UINT, ppConstantBuffers: *mut *mut ID3D11Buffer) -> (),
	fn DSGetShaderResources(&mut self, StartSlot: UINT, NumViews: UINT, ppShaderResourceViews: *mut *mut ID3D11ShaderResourceView) -> (),
	fn DSGetShader(&mut self, ppDomainShader: *mut *mut ID3D11DomainShader, ppClassInstances: *mut *mut ID3D11ClassInstance, pNumClassInstances: *mut UINT) -> (),
	fn DSGetSamplers(&mut self, StartSlot: UINT, NumSamplers: UINT, ppSamplers: *mut *mut ID3D11SamplerState) -> (),
	fn DSGetConstantBuffers(&mut self, StartSlot: UINT, NumBuffers: UINT, ppConstantBuffers: *mut *mut ID3D11Buffer) -> (),
	fn CSGetShaderResources(&mut self, StartSlot: UINT, NumViews: UINT, ppShaderResourceViews: *mut *mut ID3D11ShaderResourceView) -> (),
	fn CSGetUnorderedAccessViews(&mut self, StartSlot: UINT, NumUAVs: UINT, ppUnorderedAccessViews: *mut *mut ID3D11UnorderedAccessView) -> (),
	fn CSGetShader(&mut self, ppComputeShader: *mut *mut ID3D11ComputeShader, ppClassInstances: *mut *mut ID3D11ClassInstance, pNumClassInstances: *mut UINT) -> (),
	fn CSGetSamplers(&mut self, StartSlot: UINT, NumSamplers: UINT, ppSamplers: *mut *mut ID3D11SamplerState) -> (),
	fn CSGetConstantBuffers(&mut self, StartSlot: UINT, NumBuffers: UINT, ppConstantBuffers: *mut *mut ID3D11Buffer) -> (),
	fn ClearState(&mut self) -> (),
	fn Flush(&mut self) -> (),
	fn GetType(&mut self) -> D3D11_DEVICE_CONTEXT_TYPE,
	fn GetContextFlags(&mut self) -> UINT,
	fn FinishCommandList(&mut self, RestoreDeferredContextState: BOOL, ppCommandList: *mut *mut ID3D11CommandList) -> HRESULT
}}
com_interface!{ ID3D11DeviceContext1(ID3D11DeviceContext1Vtbl): ID3D11DeviceContext(ID3D11DeviceContextVtbl) {
	fn CopySubresourceRegion1(&mut self, pDstResource: *mut ID3D11Resource, DstSubresource: UINT, DstX: UINT, DstY: UINT, DstZ: UINT, pSrcResource: *mut ID3D11Resource, SrcSubresource: UINT, pSrcBox: *const D3D11_BOX, CopyFlags: UINT) -> (),
	fn UpdateSubresource1(&mut self, pDstResource: *mut ID3D11Resource, DstSubresource: UINT, pDstBox: *const D3D11_BOX, pSrcData: *const c_void, SrcRowPitch: UINT, SrcDepthPitch: UINT, CopyFlags: UINT) -> (),
	fn DiscardResource(&mut self, pResource: *mut ID3D11Resource) -> (),
	fn DiscardView(&mut self, pResourceView: *mut ID3D11View) -> (),
	fn VSSetConstantBuffers1(&mut self, StartSlot: UINT, NumBuffers: UINT, ppConstantBuffers: *const *mut ID3D11Buffer, pFirstConstant: *const UINT, pNumConstants: *const UINT) -> (),
	fn HSSetConstantBuffers1(&mut self, StartSlot: UINT, NumBuffers: UINT, ppConstantBuffers: *const *mut ID3D11Buffer, pFirstConstant: *const UINT, pNumConstants: *const UINT) -> (),
	fn DSSetConstantBuffers1(&mut self, StartSlot: UINT, NumBuffers: UINT, ppConstantBuffers: *const *mut ID3D11Buffer, pFirstConstant: *const UINT, pNumConstants: *const UINT) -> (),
	fn GSSetConstantBuffers1(&mut self, StartSlot: UINT, NumBuffers: UINT, ppConstantBuffers: *const *mut ID3D11Buffer, pFirstConstant: *const UINT, pNumConstants: *const UINT) -> (),
	fn PSSetConstantBuffers1(&mut self, StartSlot: UINT, NumBuffers: UINT, ppConstantBuffers: *const *mut ID3D11Buffer, pFirstConstant: *const UINT, pNumConstants: *const UINT) -> (),
	fn CSSetConstantBuffers1(&mut self, StartSlot: UINT, NumBuffers: UINT, ppConstantBuffers: *const *mut ID3D11Buffer, pFirstConstant: *const UINT, pNumConstants: *const UINT) -> (),
	fn VSGetConstantBuffers1(&mut self, StartSlot: UINT, NumBuffers: UINT, ppConstantBuffers: *mut *mut ID3D11Buffer, pFirstConstant: *mut UINT, pNumConstants: *mut UINT) -> (),
	fn HSGetConstantBuffers1(&mut self, StartSlot: UINT, NumBuffers: UINT, ppConstantBuffers: *mut *mut ID3D11Buffer, pFirstConstant: *mut UINT, pNumConstants: *mut UINT) -> (),
	fn DSGetConstantBuffers1(&mut self, StartSlot: UINT, NumBuffers: UINT, ppConstantBuffers: *mut *mut ID3D11Buffer, pFirstConstant: *mut UINT, pNumConstants: *mut UINT) -> (),
	fn GSGetConstantBuffers1(&mut self, StartSlot: UINT, NumBuffers: UINT, ppConstantBuffers: *mut *mut ID3D11Buffer, pFirstConstant: *mut UINT, pNumConstants: *mut UINT) -> (),
	fn PSGetConstantBuffers1(&mut self, StartSlot: UINT, NumBuffers: UINT, ppConstantBuffers: *mut *mut ID3D11Buffer, pFirstConstant: *mut UINT, pNumConstants: *mut UINT) -> (),
	fn CSGetConstantBuffers1(&mut self, StartSlot: UINT, NumBuffers: UINT, ppConstantBuffers: *mut *mut ID3D11Buffer, pFirstConstant: *mut UINT, pNumConstants: *mut UINT) -> (),
	fn SwapDeviceContextState(&mut self, pState: *mut ID3DDeviceContextState, ppPreviousState: *mut *mut ID3DDeviceContextState) -> (),
	fn ClearView(&mut self, pView: *mut ID3D11View, Color: [FLOAT; 4], pRect: *const D3D11_RECT, NumRects: UINT) -> (),
	fn DiscardView1(&mut self, pResourceView: *mut ID3D11View, pRects: *const D3D11_RECT, NumRects: UINT) -> ()
}}
com_interface!{ ID3D11DeviceContext2(ID3D11DeviceContext2Vtbl): ID3D11DeviceContext1(ID3D11DeviceContext1Vtbl) {
	fn UpdateTileMappings(&mut self, pTiledResource: *mut ID3D11Resource, NumTiledResourceRegions: UINT, pTiledResourceRegionStartCoordinates: *const D3D11_TILED_RESOURCE_COORDINATE, pTiledResourceRegionSizes: *const D3D11_TILE_REGION_SIZE, pTilePool: *mut ID3D11Buffer, NumRanges: UINT, pRangeFlags: *const UINT, pTilePoolStartOffsets: *const UINT, pRangeTileCounts: *const UINT, Flags: UINT) -> HRESULT,
	fn CopyTileMappings(&mut self, pDestTiledResource: *mut ID3D11Resource, pDestRegionStartCoordinate: *const D3D11_TILED_RESOURCE_COORDINATE, pSourceTiledResource: *mut ID3D11Resource, pSourceRegionStartCoordinate: *const D3D11_TILED_RESOURCE_COORDINATE, pTileRegionSize: *const D3D11_TILE_REGION_SIZE, Flags: UINT) -> HRESULT,
	fn CopyTiles(&mut self, pTiledResource: *mut ID3D11Resource, pTileRegionStartCoordinate: *const D3D11_TILED_RESOURCE_COORDINATE, pTileRegionSize: *const D3D11_TILE_REGION_SIZE, pBuffer: *mut ID3D11Buffer, BufferStartOffsetInBytes: UINT64, Flags: UINT) -> (),
	fn UpdateTiles(&mut self, pDestTiledResource: *mut ID3D11Resource, pDestTileRegionStartCoordinate: *const D3D11_TILED_RESOURCE_COORDINATE, pDestTileRegionSize: *const D3D11_TILE_REGION_SIZE, pSourceTileData: *const c_void, Flags: UINT) -> (),
	fn ResizeTilePool(&mut self, pTilePool: *mut ID3D11Buffer, NewSizeInBytes: UINT64) -> HRESULT,
	fn TiledResourceBarrier(&mut self, pTiledResourceOrViewAccessBeforeBarrier: *mut ID3D11DeviceChild, pTiledResourceOrViewAccessAfterBarrier: *mut ID3D11DeviceChild) -> (),
	fn IsAnnotationEnabled(&mut self) -> BOOL,
	fn SetMarkerInt(&mut self, pLabel: LPCWSTR, Data: INT) -> (),
	fn BeginEventInt(&mut self, pLabel: LPCWSTR, Data: INT) -> (),
	fn EndEvent(&mut self) -> ()
}}
com_interface!{ ID3D11Device(ID3D11DeviceVtbl): IUnknown(IUnknownVtbl) {
	fn CreateBuffer(&mut self, desc: *const D3D11_BUFFER_DESC, initial_data: *const D3D11_SUBRESOURCE_DATA, buffer: *mut *mut ID3D11Buffer) -> HRESULT,
	fn CreateTexture1D(&mut self, desc: *const D3D11_TEXTURE1D_DESC, initial_data: *const D3D11_SUBRESOURCE_DATA, texture1_d: *mut *mut ID3D11Texture1D) -> HRESULT,
	fn CreateTexture2D(&mut self, desc: *const D3D11_TEXTURE2D_DESC, initial_data: *const D3D11_SUBRESOURCE_DATA, texture2_d: *mut *mut ID3D11Texture2D) -> HRESULT,
	fn CreateTexture3D(&mut self, desc: *const D3D11_TEXTURE3D_DESC, initial_data: *const D3D11_SUBRESOURCE_DATA, texture3_d: *mut *mut ID3D11Texture3D) -> HRESULT,
	fn CreateShaderResourceView(&mut self, resource: *mut ID3D11Resource, desc: *const D3D11_SHADER_RESOURCE_VIEW_DESC, s_r_view: *mut *mut ID3D11ShaderResourceView) -> HRESULT,
	fn CreateUnorderedAccessView(&mut self, resource: *mut ID3D11Resource, desc: *const D3D11_UNORDERED_ACCESS_VIEW_DESC, u_a_view: *mut *mut ID3D11UnorderedAccessView) -> HRESULT,
	fn CreateRenderTargetView(&mut self, resource: *mut ID3D11Resource, desc: *const D3D11_RENDER_TARGET_VIEW_DESC, r_t_view: *mut *mut ID3D11RenderTargetView) -> HRESULT,
	fn CreateDepthStencilView(&mut self, resource: *mut ID3D11Resource, desc: *const D3D11_DEPTH_STENCIL_VIEW_DESC, depth_stencil_view: *mut *mut ID3D11DepthStencilView) -> HRESULT,
	fn CreateInputLayout(&mut self, input_element_descs: *const D3D11_INPUT_ELEMENT_DESC, num_elements: UINT, shader_bytecode_with_input_signature: *const c_void, bytecode_length: SIZE_T, input_layout: *mut *mut ID3D11InputLayout) -> HRESULT,
	fn CreateVertexShader(&mut self, shader_bytecode: *const c_void, bytecode_length: SIZE_T, class_linkage: *mut ID3D11ClassLinkage, vertex_shader: *mut *mut ID3D11VertexShader) -> HRESULT,
	fn CreateGeometryShader(&mut self, shader_bytecode: *const c_void, bytecode_length: SIZE_T, class_linkage: *mut ID3D11ClassLinkage, geometry_shader: *mut *mut ID3D11GeometryShader) -> HRESULT,
	fn CreateGeometryShaderWithStreamOutput(&mut self, shader_bytecode: *const c_void, bytecode_length: SIZE_T, s_o_declaration: *const D3D11_SO_DECLARATION_ENTRY, num_entries: UINT, buffer_strides: *const UINT, num_strides: UINT, rasterized_stream: UINT, class_linkage: *mut ID3D11ClassLinkage, geometry_shader: *mut *mut ID3D11GeometryShader) -> HRESULT,
	fn CreatePixelShader(&mut self, shader_bytecode: *const c_void, bytecode_length: SIZE_T, class_linkage: *mut ID3D11ClassLinkage, pixel_shader: *mut *mut ID3D11PixelShader) -> HRESULT,
	fn CreateHullShader(&mut self, shader_bytecode: *const c_void, bytecode_length: SIZE_T, class_linkage: *mut ID3D11ClassLinkage, hull_shader: *mut *mut ID3D11HullShader) -> HRESULT,
	fn CreateDomainShader(&mut self, shader_bytecode: *const c_void, bytecode_length: SIZE_T, class_linkage: *mut ID3D11ClassLinkage, domain_shader: *mut *mut ID3D11DomainShader) -> HRESULT,
	fn CreateComputeShader(&mut self, shader_bytecode: *const c_void, bytecode_length: SIZE_T, class_linkage: *mut ID3D11ClassLinkage, compute_shader: *mut *mut ID3D11ComputeShader) -> HRESULT,
	fn CreateClassLinkage(&mut self, linkage: *mut *mut ID3D11ClassLinkage) -> HRESULT,
	fn CreateBlendState(&mut self, blend_state_desc: *const D3D11_BLEND_DESC, blend_state: *mut *mut ID3D11BlendState) -> HRESULT,
	fn CreateDepthStencilState(&mut self, depth_stencil_desc: *const D3D11_DEPTH_STENCIL_DESC, depth_stencil_state: *mut *mut ID3D11DepthStencilState) -> HRESULT,
	fn CreateRasterizerState(&mut self, rasterizer_desc: *const D3D11_RASTERIZER_DESC, rasterizer_state: *mut *mut ID3D11RasterizerState) -> HRESULT,
	fn CreateSamplerState(&mut self, sampler_desc: *const D3D11_SAMPLER_DESC, sampler_state: *mut *mut ID3D11SamplerState) -> HRESULT,
	fn CreateQuery(&mut self, query_desc: *const D3D11_QUERY_DESC, query: *mut *mut ID3D11Query) -> HRESULT,
	fn CreatePredicate(&mut self, predicate_desc: *const D3D11_QUERY_DESC, predicate: *mut *mut ID3D11Predicate) -> HRESULT,
	fn CreateCounter(&mut self, counter_desc: *const D3D11_COUNTER_DESC, counter: *mut *mut ID3D11Counter) -> HRESULT,
	fn CreateDeferredContext(&mut self, context_flags: UINT, deferred_context: *mut *mut ID3D11DeviceContext) -> HRESULT,
	fn OpenSharedResource(&mut self, h_resource: HANDLE, returned_interface: REFIID, resource: *mut *mut c_void) -> HRESULT,
	fn CheckFormatSupport(&mut self, format: DXGI_FORMAT, format_support: *mut UINT) -> HRESULT,
	fn CheckMultisampleQualityLevels(&mut self, format: DXGI_FORMAT, samle_count: UINT, num_quality_levels: *mut UINT) -> HRESULT,
	fn CheckCounterInfo(&mut self, counter_info: *mut D3D11_COUNTER_INFO) -> (),
	fn CheckCounter(&mut self, desc: *const D3D11_COUNTER_DESC, type_: *mut D3D11_COUNTER_TYPE, active_counters: *mut UINT, sz_name: LPSTR, name_length: *mut UINT, sz_units: LPSTR, units_length: *mut UINT, sz_descrition: LPSTR, description_length: *mut UINT) -> HRESULT,
	fn CheckFeatureSupport(&mut self, feature: D3D11_FEATURE, feature_support_data: *mut c_void, feature_suort_data_size: UINT) -> HRESULT,
	fn GetPrivateData(&mut self, guid: REFGUID, data_size: *mut UINT, data: *mut c_void) -> HRESULT,
	fn SetPrivateData(&mut self, guid: REFGUID, data_size: UINT, data: *const c_void) -> HRESULT,
	fn SetPrivateDataInterface(&mut self, guid: REFGUID, data: *const IUnknown) -> HRESULT,
	fn GetFeatureLevel(&mut self) -> D3D_FEATURE_LEVEL,
	fn GetCreationFlags(&mut self) -> UINT,
	fn GetDeviceRemovedReason(&mut self) -> HRESULT,
	fn GetImmediateContext(&mut self, immediate_context: *mut *mut ID3D11DeviceContext) -> (),
	fn SetExceptionMode(&mut self, raise_flags: UINT) -> HRESULT,
	fn GetExceptionMode(&mut self) -> UINT
}}
com_interface!{ ID3D11Device1(ID3D11Device1Vtbl): ID3D11Device(ID3D11DeviceVtbl) {
	fn GetImmediateContext1(&mut self, ppImmediateContext: *mut *mut ID3D11DeviceContext1) -> (),
	fn CreateDeferredContext1(&mut self, ContextFlags: UINT, ppDeferredContext: *mut *mut ID3D11DeviceContext1) -> HRESULT,
	fn CreateBlendState1(&mut self, pBlendStateDesc: *const D3D11_BLEND_DESC1, ppBlendState: *mut *mut ID3D11BlendState1) -> HRESULT,
	fn CreateRasterizerState1(&mut self, pRasterizerDesc: *const D3D11_RASTERIZER_DESC1, ppRasterizerState: *mut *mut ID3D11RasterizerState1) -> HRESULT,
	fn CreateDeviceContextState(&mut self, Flags: UINT, pFeatureLevels: *const D3D_FEATURE_LEVEL, FeatureLevels: UINT, SDKVersion: UINT, EmulatedInterface: REFIID, pChosenFeatureLevel: *mut D3D_FEATURE_LEVEL, ppContextState: *mut *mut ID3DDeviceContextState) -> HRESULT,
	fn OpenSharedResource1(&mut self, hResource: HANDLE, returnedInterface: REFIID, ppResource: *mut *mut c_void) -> HRESULT,
	fn OpenSharedResourceByName(&mut self, lpName: LPCWSTR, dwDesiredAccess: DWORD, returnedInterface: REFIID, ppResource: *mut *mut c_void) -> HRESULT
}}
com_interface!{ ID3D11Device2(ID3D11Device2Vtbl): ID3D11Device1(ID3D11Device1Vtbl) {
	fn GetImmediateContext2(&mut self, ppImmediateContext: *mut *mut ID3D11DeviceContext2) -> (),
	fn CreateDeferredContext2(&mut self, ContextFlags: UINT, ppDeferredContext: *mut *mut ID3D11DeviceContext2) -> HRESULT,
	fn GetResourceTiling(&mut self, pTiledResource: *mut ID3D11Resource, pNumTilesForEntireResource: *mut UINT, pPackedMipDesc: *mut D3D11_PACKED_MIP_DESC, pStandardTileShapeForNonPackedMips: *mut D3D11_TILE_SHAPE, pNumSubresourceTilings: *mut UINT, FirstSubresourceTilingToGet: UINT, pSubresourceTilingsForNonPackedMips: *mut D3D11_SUBRESOURCE_TILING) -> (),
	fn CheckMultisampleQualityLevels1(&mut self, Format: DXGI_FORMAT,SampleCount: UINT, Flags: UINT, pNumQualityLevels: *mut UINT) -> HRESULT
}}
com_interface!{ ID3DDeviceContextState(ID3DDeviceContextStateVtbl): IUnknown(IUnknownVtbl) { } }
com_interface!{ ID3D11InputLayout(ID3D11InputLayoutVtbl): IUnknown(IUnknownVtbl) { } }
com_interface!{ ID3D11RasterizerState(ID3D11RasterizerStateVtbl): IUnknown(IUnknownVtbl) {
	fn GetDesc(&mut self, desc: *mut D3D11_RASTERIZER_DESC) -> ()
}}
com_interface!{ ID3D11RasterizerState1(ID3D11RasterizerState1Vtbl): ID3D11RasterizerState(ID3D11RasterizerStateVtbl) {
	fn GetDesc1(&mut self, desc: *mut D3D11_RASTERIZER_DESC1) -> ()
}}
com_interface!{ ID3D11SamplerState(ID3D11SamplerStateVtbl): IUnknown(IUnknownVtbl) {
	fn GetDesc(&mut self, desc: *mut D3D11_SAMPLER_DESC) -> ()
}}

impl QueryIID for ID3D11Asynchronous { fn iid() -> GUID { IID_ID3D11Asynchronous } }
impl QueryIID for ID3D11BlendState { fn iid() -> GUID { IID_ID3D11BlendState } }
impl QueryIID for ID3D11BlendState1 { fn iid() -> GUID { IID_ID3D11BlendState1 } }
impl QueryIID for ID3D11Counter { fn iid() -> GUID { IID_ID3D11Counter } }
impl QueryIID for ID3D11CommandList { fn iid() -> GUID { IID_ID3D11CommandList } }
impl QueryIID for ID3D11DepthStencilState { fn iid() -> GUID { IID_ID3D11DepthStencilState } }
impl QueryIID for ID3D11Device { fn iid() -> GUID { IID_ID3D11Device } }
impl QueryIID for ID3D11Device1 { fn iid() -> GUID { IID_ID3D11Device1 } }
impl QueryIID for ID3D11Device2 { fn iid() -> GUID { IID_ID3D11Device2 } }
impl QueryIID for ID3D11DeviceChild { fn iid() -> GUID { IID_ID3D11DeviceChild } }
impl QueryIID for ID3D11DeviceContext { fn iid() -> GUID { IID_ID3D11DeviceContext } }
impl QueryIID for ID3D11DeviceContext1 { fn iid() -> GUID { IID_ID3D11DeviceContext1 } }
impl QueryIID for ID3D11DeviceContext2 { fn iid() -> GUID { IID_ID3D11DeviceContext2 } }
impl QueryIID for ID3DDeviceContextState { fn iid() -> GUID { IID_ID3DDeviceContextState } }
impl QueryIID for ID3D11InputLayout { fn iid() -> GUID { IID_ID3D11InputLayout } }
impl QueryIID for ID3D11Predicate { fn iid() -> GUID { IID_ID3D11Predicate } }
impl QueryIID for ID3D11Query { fn iid() -> GUID { IID_ID3D11Query } }
impl QueryIID for ID3D11RasterizerState { fn iid() -> GUID { IID_ID3D11RasterizerState } }
impl QueryIID for ID3D11RasterizerState1 { fn iid() -> GUID { IID_ID3D11RasterizerState1 } }
impl QueryIID for ID3D11SamplerState { fn iid() -> GUID { IID_ID3D11SamplerState } }
