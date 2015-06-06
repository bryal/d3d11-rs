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
use winapi::basetsd::*;
use winapi::{ HRESULT, LPCSTR, GUID, LPSTR };
use dxgi::{ IUnknown, IUnknownVtbl, QueryIID, COMInterface };

use constants::*;
use common_version::enumerations::{ D3D_FEATURE_LEVEL, D3D_PRIMITIVE };
use common_version::interfaces::ID3DBlob;
use core::interfaces::{ ID3D11DeviceChild, ID3D11DeviceChildVtbl };
use shader::structures::{ D3D11_SHADER_VARIABLE_DESC, D3D11_SHADER_TYPE_DESC,
	D3D11_SHADER_TRACE_DESC, D3D11_SHADER_BUFFER_DESC,
	D3D11_TRACE_REGISTER, D3D11_TRACE_VALUE,
	D3D11_TRACE_STEP, D3D11_TRACE_STATS,
	D3D11_SHADER_DESC, D3D11_SHADER_INPUT_BIND_DESC,
	D3D11_SIGNATURE_PARAMETER_DESC, D3D11_PARAMETER_DESC,
	D3D11_LIBRARY_DESC, D3D11_CLASS_INSTANCE_DESC };

com_interface!{ ID3D11ClassInstance(ID3D11ClassInstanceVtbl): ID3D11DeviceChild(ID3D11DeviceChildVtbl) {
	fn GetClassLinkage(&mut self, ppLinkage: *mut *mut ID3D11ClassLinkage) -> (),
	fn GetDesc(&mut self, pDesc: *mut D3D11_CLASS_INSTANCE_DESC) -> (),
	fn GetInstanceName(&mut self, pInstanceName: LPSTR, pBufferLength: *mut SIZE_T) -> (),
	fn GetTypeName(&mut self, pTypeName: LPSTR, pBufferLength: *mut SIZE_T) -> ()
}}
com_interface!{ ID3D11ClassLinkage(ID3D11ClassLinkageVtbl): ID3D11DeviceChild(ID3D11DeviceChildVtbl) {
	fn GetClassInstance(&mut self, pClassInstanceName: LPCSTR, InstanceIndex: UINT, ppInstance: *mut *mut ID3D11ClassInstance) -> HRESULT,
	fn CreateClassInstance(&mut self, pClassTypeName: LPCSTR, ConstantBufferOffset: UINT, ConstantVectorOffset: UINT, TextureOffset: UINT, SamplerOffset: UINT, ppInstance: *mut *mut ID3D11ClassInstance) -> HRESULT
}}
com_interface!{ ID3D11ComputeShader(ID3D11ComputeShaderVtbl): ID3D11DeviceChild(ID3D11DeviceChildVtbl) { } }
com_interface!{ ID3D11DomainShader(ID3D11DomainShaderVtbl): ID3D11DeviceChild(ID3D11DeviceChildVtbl) { } }
com_interface!{ ID3D11GeometryShader(ID3D11GeometryShaderVtbl): ID3D11DeviceChild(ID3D11DeviceChildVtbl) { } }
com_interface!{ ID3D11HullShader(ID3D11HullShaderVtbl): ID3D11DeviceChild(ID3D11DeviceChildVtbl) { } }
com_interface!{ ID3D11PixelShader(ID3D11PixelShaderVtbl): ID3D11DeviceChild(ID3D11DeviceChildVtbl) { } }
com_interface!{ ID3D11VertexShader(ID3D11VertexShaderVtbl): ID3D11DeviceChild(ID3D11DeviceChildVtbl) { } }
com_interface!{ ID3D11FunctionLinkingGraph(ID3D11FunctionLinkingGraphVtbl): IUnknown(IUnknownVtbl) {
	fn CreateModuleInstance(&mut self, module_instance: *mut *mut ID3D11ModuleInstance, error_buffer: *mut *mut ID3DBlob) -> HRESULT,
	fn SetInputSignature(&mut self, input_parameters: *const D3D11_PARAMETER_DESC, input_parameters_count: UINT, output_node: *mut *mut ID3D11LinkingNode) -> HRESULT,
	fn CallFunction(&mut self, modure_instance_namespace: LPCSTR, module_with_fn_prototype: *mut ID3D11Module, funtion_name: LPCSTR, call_node: *mut *mut ID3D11LinkingNode) -> HRESULT,
	fn PassValue(&mut self, src_node: *mut ID3D11LinkingNode, src_parameter_index: INT, dst_node: *mut ID3D11LinkingNode, dst_parameter_index: INT) -> HRESULT,
	fn PassValueWithSwizzle(&mut self, src_node: *mut ID3D11LinkingNode, src_parameter_index: INT, src_swizzle: LPCSTR, dst_node: *mut ID3D11LinkingNode, dst_parameter_index: INT, dst_swizzle: LPCSTR) -> HRESULT,
	fn GetLastError(&mut self, error_buf: *mut *mut ID3DBlob) -> HRESULT,
	fn GenerateHlsl(&mut self, flags: UINT, buffer: *mut *mut ID3DBlob) -> HRESULT
}}
com_interface!{ ID3D11LibraryReflection(ID3D11LibraryReflectionVtbl): IUnknown(IUnknownVtbl) {
	fn GetDesc(&mut self, desc: *mut D3D11_LIBRARY_DESC) -> HRESULT,
	fn GetFunctionByIndex(&mut self, function_index: INT) -> *mut ID3D11FunctionReflection
}}
com_interface!{ ID3D11Linker(ID3D11LinkerVtbl): IUnknown(IUnknownVtbl) {
	fn Link(&mut self, pEntry: *mut ID3D11ModuleInstance, pEntryName: LPCSTR, pTargetName: LPCSTR, uFlags: UINT, ShaderBlob: *mut *mut ID3DBlob, ErrorBuffer: *mut *mut ID3DBlob) -> HRESULT,
	fn UseLibrary(&mut self, LibraryMI: *mut ID3D11ModuleInstance) -> HRESULT,
	fn AddClipPlaneFromCBuffer(&mut self, uCBufferSlot: UINT, uCBufferEntry: UINT) -> HRESULT
}}
com_interface!{ ID3D11LinkingNode(ID3D11LinkingNodeVtbl): IUnknown(IUnknownVtbl) { } }
com_interface!{ ID3D11Module(ID3D11ModuleVtbl): IUnknown(IUnknownVtbl) {
	fn CreateInstance(&mut self, pNamespace: LPCSTR, ppModuleInstance: *mut *mut ID3D11ModuleInstance) -> HRESULT
}}
com_interface!{ ID3D11ModuleInstance(ID3D11ModuleInstanceVtbl): IUnknown(IUnknownVtbl) {
	fn BindConstantBuffer(&mut self, uSrcSlot: UINT, uDstSlot: UINT, cbDstOffset: UINT) -> HRESULT,
	fn BindConstantBufferByName(&mut self, pName: LPCSTR, uDstSlot: UINT, cbDstOffset: UINT) -> HRESULT,
	fn BindResource(&mut self, uSrcSlot: UINT, uDstSlot: UINT, uCount: UINT) -> HRESULT,
	fn BindResourceByName(&mut self, pName: LPCSTR, uDstSlot: UINT, uCount: UINT) -> HRESULT,
	fn BindSampler(&mut self, uSrcSlot: UINT, uDstSlot: UINT, uCount: UINT) -> HRESULT,
	fn BindSamplerByName(&mut self, pName: LPCSTR, uDstSlot: UINT, uCount: UINT) -> HRESULT,
	fn BindUnorderedAccessView(&mut self, uSrcSlot: UINT, uDstSlot: UINT, uCount: UINT) -> HRESULT,
	fn BindUnorderedAccessViewByName(&mut self, pName: LPCSTR, uDstSlot: UINT, uCount: UINT) -> HRESULT,
	fn BindResourceAsUnorderedAccessView(&mut self, uSrcSrvSlot: UINT, uDstUavSlot: UINT, uCount: UINT) -> HRESULT,
	fn BindResourceAsUnorderedAccessViewByName(&mut self, pSrvName: LPCSTR, uDstUavSlot: UINT, uCount: UINT) -> HRESULT
}}
com_interface!{ ID3D11ShaderReflection(ID3D11ShaderReflectionVtbl): IUnknown(IUnknownVtbl) {
	fn GetDesc(&mut self, pDesc: *mut D3D11_SHADER_DESC) -> HRESULT,
	fn GetConstantBufferByIndex(&mut self, Index: UINT) -> *mut ID3D11ShaderReflectionConstantBuffer,
	fn GetConstantBufferByName(&mut self, Name: LPCSTR) -> *mut ID3D11ShaderReflectionConstantBuffer,
	fn GetResourceBindingDesc(&mut self, ResourceIndex: UINT, pDesc: *mut D3D11_SHADER_INPUT_BIND_DESC) -> HRESULT,
	fn GetInputParameterDesc(&mut self, ParameterIndex: UINT, pDesc: *mut D3D11_SIGNATURE_PARAMETER_DESC) -> HRESULT,
	fn GetOutputParameterDesc(&mut self, ParameterIndex: UINT, pDesc: *mut D3D11_SIGNATURE_PARAMETER_DESC) -> HRESULT,
	fn GetPatchConstantParameterDesc(&mut self, ParameterIndex: UINT, pDesc: *mut D3D11_SIGNATURE_PARAMETER_DESC) -> HRESULT,
	fn GetVariableByName(&mut self, Name: LPCSTR) -> *mut ID3D11ShaderReflectionVariable,
	fn GetResourceBindingDescByName(&mut self, Name: LPCSTR, pDesc: *mut D3D11_SHADER_INPUT_BIND_DESC) -> HRESULT,
	fn GetMovInstructionCount(&mut self) -> UINT,
	fn GetMovcInstructionCount(&mut self) -> UINT,
	fn GetConversionInstructionCount(&mut self) -> UINT,
	fn GetBitwiseInstructionCount(&mut self) -> UINT,
	fn GetGSInputPrimitive(&mut self) -> D3D_PRIMITIVE,
	fn IsSampleFrequencyShader(&mut self) -> BOOL,
	fn GetNumInterfaceSlots(&mut self) -> UINT,
	fn GetMinFeatureLevel(&mut self, pLevel: *mut D3D_FEATURE_LEVEL) -> HRESULT,
	fn GetThreadGroupSize(&mut self, pSizeX: *mut UINT, pSizeY: *mut UINT, pSizeZ: *mut UINT) -> UINT,
	fn GetRequiresFlags(&mut self) -> UINT64
}}
com_interface!{ ID3D11ShaderTrace(ID3D11ShaderTraceVtbl): IUnknown(IUnknownVtbl) {
	fn TraceReady(&mut self, pTestCount: *mut UINT64) -> HRESULT,
	fn ResetTrace(&mut self) -> (),
	fn GetTraceStats(&mut self, pTraceStats: *mut D3D11_TRACE_STATS) -> HRESULT,
	fn PSSelectStamp(&mut self, stampIndex: UINT) -> HRESULT,
	fn GetInitialRegisterContents(&mut self, pRegister: *mut D3D11_TRACE_REGISTER, pValue: *mut D3D11_TRACE_VALUE) -> HRESULT,
	fn GetStep(&mut self, stepIndex: UINT, pTraceStep: *mut D3D11_TRACE_STEP) -> HRESULT,
	fn GetWrittenRegister(&mut self, stepIndex: UINT, writtenRegisterIndex: UINT, pRegister: *mut D3D11_TRACE_REGISTER, pValue: *mut D3D11_TRACE_VALUE) -> HRESULT,
	fn GetReadRegister(&mut self, stepIndex: UINT, readRegisterIndex: UINT, pRegister: *mut D3D11_TRACE_REGISTER, pValue: *mut D3D11_TRACE_VALUE) -> HRESULT
}}
com_interface!{ ID3D11ShaderTraceFactory(ID3D11ShaderTraceFactoryVtbl): IUnknown(IUnknownVtbl) {
	fn CreateShaderTrace(&mut self, pShader: *mut IUnknown, pTraceDesc: *mut D3D11_SHADER_TRACE_DESC, ppShaderTrace: *mut *mut ID3D11ShaderTrace) -> HRESULT
}}

interface!{ ID3D11FunctionReflection(ID3D11FunctionReflectionVtbl) {
	fn GetDesc(&mut self, desc: *mut D3D11_PARAMETER_DESC) -> HRESULT
}}
interface!{ ID3D11FunctionParameterReflection(ID3D11FunctionParameterReflectionVtbl) {
	fn GetDesc(&mut self, desc: *mut D3D11_PARAMETER_DESC) -> HRESULT
}}
interface!{ ID3D11ShaderReflectionConstantBuffer(ID3D11ShaderReflectionConstantBufferVtbl) {
	fn GetDesc(&mut self, pDesc: *mut D3D11_SHADER_BUFFER_DESC) -> HRESULT,
	fn GetVariableByIndex(&mut self, Index: UINT) -> *mut ID3D11ShaderReflectionVariable,
	fn GetVariableByName(&mut self, Name: LPCSTR) -> *mut ID3D11ShaderReflectionVariable
}}
interface!{ ID3D11ShaderReflectionType(ID3D11ShaderReflectionTypeVtbl) {
	fn GetDesc(&mut self, pDesc: *mut D3D11_SHADER_TYPE_DESC) -> HRESULT,
	fn GetMemberTypeByIndex(&mut self, Index: UINT) -> *mut ID3D11ShaderReflectionType,
	fn GetMemberTypeByName(&mut self, Name: LPCSTR) -> *mut ID3D11ShaderReflectionType,
	fn GetMemberTypeName(&mut self, Index: UINT) -> LPCSTR,
	fn IsEqual(&mut self, pType: *mut ID3D11ShaderReflectionType) -> HRESULT,
	fn GetSubType(&mut self) -> *mut ID3D11ShaderReflectionType,
	fn GetBaseClass(&mut self) -> *mut ID3D11ShaderReflectionType,
	fn GetNumInterfaces(&mut self) -> UINT,
	fn GetInterfaceByIndex(&mut self, uIndex: UINT) -> *mut ID3D11ShaderReflectionType,
	fn IsOfType(&mut self, pType: *mut ID3D11ShaderReflectionType) -> HRESULT,
	fn ImplementsInterface(&mut self, pBase: *mut ID3D11ShaderReflectionType) -> HRESULT
}}
interface!{ ID3D11ShaderReflectionVariable(ID3D11ShaderReflectionVariableVtbl) {
	fn GetDesc(&mut self, pDesc: *mut D3D11_SHADER_VARIABLE_DESC) -> HRESULT,
	fn GetType(&mut self) -> *mut ID3D11ShaderReflectionType,
	fn GetBuffer(&mut self) -> *mut ID3D11ShaderReflectionConstantBuffer,
	fn GetInterfaceSlot(&mut self, uArrayIndex: UINT) -> UINT
}}

impl QueryIID for ID3D11ClassInstance { fn iid() -> GUID { IID_ID3D11ClassInstance } }
impl QueryIID for ID3D11ClassLinkage { fn iid() -> GUID { IID_ID3D11ClassLinkage } }
impl QueryIID for ID3D11ComputeShader { fn iid() -> GUID { IID_ID3D11ComputeShader } }
impl QueryIID for ID3D11DomainShader { fn iid() -> GUID { IID_ID3D11DomainShader } }
impl QueryIID for ID3D11GeometryShader { fn iid() -> GUID { IID_ID3D11GeometryShader } }
impl QueryIID for ID3D11HullShader { fn iid() -> GUID { IID_ID3D11HullShader } }
impl QueryIID for ID3D11PixelShader { fn iid() -> GUID { IID_ID3D11PixelShader } }
impl QueryIID for ID3D11VertexShader { fn iid() -> GUID { IID_ID3D11VertexShader } }
impl QueryIID for ID3D11FunctionLinkingGraph { fn iid() -> GUID { IID_ID3D11FunctionLinkingGraph } }
impl QueryIID for ID3D11LibraryReflection { fn iid() -> GUID { IID_ID3D11LibraryReflection } }
impl QueryIID for ID3D11Linker { fn iid() -> GUID { IID_ID3D11Linker } }
impl QueryIID for ID3D11LinkingNode { fn iid() -> GUID { IID_ID3D11LinkingNode } }
impl QueryIID for ID3D11Module { fn iid() -> GUID { IID_ID3D11Module } }
impl QueryIID for ID3D11ModuleInstance { fn iid() -> GUID { IID_ID3D11ModuleInstance } }
impl QueryIID for ID3D11ShaderReflection { fn iid() -> GUID { IID_ID3D11ShaderReflection } }
