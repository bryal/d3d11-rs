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

use libc::c_void;
use winapi::{ INT, UINT, ULONG, HRESULT, LPCSTR,
	REFIID, REFGUID, LPSTR, SIZE_T, UINT64,
	BOOL };
use dxgi::{ IUnknown, IUnknownT };

use common_version::enumerations::{ D3D_FEATURE_LEVEL, D3D_PRIMITIVE };
use common_version::interfaces::ID3DBlob;
use core::interfaces::{ ID3D11DeviceChildT, ID3D11Device };
use shader::structures::{ D3D11_SHADER_VARIABLE_DESC, D3D11_SHADER_TYPE_DESC,
	D3D11_SHADER_TRACE_DESC, D3D11_SHADER_BUFFER_DESC,
	D3D11_TRACE_REGISTER, D3D11_TRACE_VALUE,
	D3D11_TRACE_STEP, D3D11_TRACE_STATS,
	D3D11_SHADER_DESC, D3D11_SHADER_INPUT_BIND_DESC,
	D3D11_SIGNATURE_PARAMETER_DESC };
use shader::structures::{ D3D11_PARAMETER_DESC, D3D11_LIBRARY_DESC,
	D3D11_CLASS_INSTANCE_DESC };

#[repr(C)] pub struct ID3D11ClassInstance { pub vtable: *mut ID3D11ClassInstanceVtbl }
#[repr(C)] pub struct ID3D11ClassLinkage { pub vtable: *mut ID3D11ClassLinkageVtbl }
#[repr(C)] pub struct ID3D11ComputeShader { pub vtable: *mut ID3D11ComputeShaderVtbl }
#[repr(C)] pub struct ID3D11DomainShader { pub vtable: *mut ID3D11DomainShaderVtbl }
#[repr(C)] pub struct ID3D11FunctionLinkingGraph { pub vtable: *mut ID3D11FunctionLinkingGraphVtbl }
#[repr(C)] pub struct ID3D11FunctionReflection { pub vtable: *mut ID3D11FunctionReflectionVtbl }
#[repr(C)] pub struct ID3D11FunctionParameterReflection { pub vtable: *mut ID3D11FunctionParameterReflectionVtbl }
#[repr(C)] pub struct ID3D11GeometryShader { pub vtable: *mut ID3D11GeometryShaderVtbl }
#[repr(C)] pub struct ID3D11HullShader { pub vtable: *mut ID3D11HullShaderVtbl }
#[repr(C)] pub struct ID3D11LibraryReflection { pub vtable: *mut ID3D11LibraryReflectionVtbl }
#[repr(C)] pub struct ID3D11Linker { pub vtable: *mut ID3D11LinkerVtbl }
#[repr(C)] pub struct ID3D11LinkingNode { pub vtable: *mut ID3D11LinkingNodeVtbl }
#[repr(C)] pub struct ID3D11Module { pub vtable: *mut ID3D11ModuleVtbl }
#[repr(C)] pub struct ID3D11ModuleInstance { pub vtable: *mut ID3D11ModuleInstanceVtbl }
#[repr(C)] pub struct ID3D11PixelShader { pub vtable: *mut ID3D11PixelShaderVtbl }
#[repr(C)] pub struct ID3D11ShaderReflection { pub vtable: *mut ID3D11ShaderReflectionVtbl }
#[repr(C)] pub struct ID3D11ShaderReflectionConstantBuffer { pub vtable: *mut ID3D11ShaderReflectionConstantBufferVtbl }
#[repr(C)] pub struct ID3D11ShaderReflectionType { pub vtable: *mut ID3D11ShaderReflectionTypeVtbl }
#[repr(C)] pub struct ID3D11ShaderReflectionVariable { pub vtable: *mut ID3D11ShaderReflectionVariableVtbl }
#[repr(C)] pub struct ID3D11ShaderTrace { pub vtable: *mut ID3D11ShaderTraceVtbl }
#[repr(C)] pub struct ID3D11ShaderTraceFactory { pub vtable: *mut ID3D11ShaderTraceFactoryVtbl }
#[repr(C)] pub struct ID3D11VertexShader { pub vtable: *mut ID3D11VertexShaderVtbl }

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
			pub ID3D11ClassInstanceVtbl of ID3D11ClassInstance, pub trait ID3D11ClassInstanceT {
				fn GetClassLinkage(ppLinkage: *mut *mut ID3D11ClassLinkage) -> (),
				fn GetDesc(pDesc: *mut D3D11_CLASS_INSTANCE_DESC) -> (),
				fn GetInstanceName(pInstanceName: LPSTR, pBufferLength: *mut SIZE_T) -> (),
				fn GetTypeName(pTypeName: LPSTR, pBufferLength: *mut SIZE_T) -> (),
			}
			pub ID3D11ClassLinkageVtbl of ID3D11ClassLinkage, pub trait ID3D11ClassLinkageT {
				fn GetClassInstance(pClassInstanceName: LPCSTR, InstanceIndex: UINT, ppInstance: *mut *mut ID3D11ClassInstance) -> HRESULT,
				fn CreateClassInstance(pClassTypeName: LPCSTR, ConstantBufferOffset: UINT, ConstantVectorOffset: UINT, TextureOffset: UINT, SamplerOffset: UINT, ppInstance: *mut *mut ID3D11ClassInstance) -> HRESULT,
			}
			pub ID3D11ComputeShaderVtbl of ID3D11ComputeShader, pub trait ID3D11ComputeShaderT { }
			pub ID3D11DomainShaderVtbl of ID3D11DomainShader, pub trait ID3D11DomainShaderT { }
			pub ID3D11GeometryShaderVtbl of ID3D11GeometryShader,
				pub trait ID3D11GeometryShaderT { }
			pub ID3D11HullShaderVtbl of ID3D11HullShader, pub trait ID3D11HullShaderT { }
			pub ID3D11PixelShaderVtbl of ID3D11PixelShader, pub trait ID3D11PixelShaderT { }
			pub ID3D11VertexShaderVtbl of ID3D11VertexShader, pub trait ID3D11VertexShaderT { }
		]
		pub ID3D11FunctionLinkingGraphVtbl of ID3D11FunctionLinkingGraph,
			pub trait ID3D11FunctionLinkingGraphT
		{
			fn CreateModuleInstance(module_instance: *mut *mut ID3D11ModuleInstance, error_buffer: *mut *mut ID3DBlob) -> HRESULT,
			fn SetInputSignature(input_parameters: *const D3D11_PARAMETER_DESC, input_parameters_count: UINT, output_node: *mut *mut ID3D11LinkingNode) -> HRESULT,
			fn CallFunction(modure_instance_namespace: LPCSTR, module_with_fn_prototype: *mut ID3D11Module, funtion_name: LPCSTR, call_node: *mut *mut ID3D11LinkingNode) -> HRESULT,
			fn PassValue(src_node: *mut ID3D11LinkingNode, src_parameter_index: INT, dst_node: *mut ID3D11LinkingNode, dst_parameter_index: INT) -> HRESULT,
			fn PassValueWithSwizzle(src_node: *mut ID3D11LinkingNode, src_parameter_index: INT, src_swizzle: LPCSTR, dst_node: *mut ID3D11LinkingNode, dst_parameter_index: INT, dst_swizzle: LPCSTR) -> HRESULT,
			fn GetLastError(error_buf: *mut *mut ID3DBlob) -> HRESULT,
			fn GenerateHlsl(flags: UINT, buffer: *mut *mut ID3DBlob) -> HRESULT,
		}
		pub ID3D11LibraryReflectionVtbl of ID3D11LibraryReflection,
			pub trait ID3D11LibraryReflectionT
		{
			fn GetDesc(desc: *mut D3D11_LIBRARY_DESC) -> HRESULT,
			fn GetFunctionByIndex(function_index: INT) -> *mut ID3D11FunctionReflection,
		}
		pub ID3D11LinkerVtbl of ID3D11Linker, pub trait ID3D11LinkerT {
			fn Link(pEntry: *mut ID3D11ModuleInstance, pEntryName: LPCSTR, pTargetName: LPCSTR, uFlags: UINT, ShaderBlob: *mut *mut ID3DBlob, ErrorBuffer: *mut *mut ID3DBlob) -> HRESULT,
			fn UseLibrary(LibraryMI: *mut ID3D11ModuleInstance) -> HRESULT,
			fn AddClipPlaneFromCBuffer(uCBufferSlot: UINT, uCBufferEntry: UINT) -> HRESULT,
		}
		pub ID3D11LinkingNodeVtbl of ID3D11LinkingNode, pub trait ID3D11LinkingNodeT { }
		pub ID3D11ModuleVtbl of ID3D11Module, pub trait ID3D11ModuleT {
			fn CreateInstance(pNamespace: LPCSTR, ppModuleInstance: *mut *mut ID3D11ModuleInstance) -> HRESULT,
		}
		pub ID3D11ModuleInstanceVtbl of ID3D11ModuleInstance, pub trait ID3D11ModuleInstanceT {
			fn BindConstantBuffer(uSrcSlot: UINT, uDstSlot: UINT, cbDstOffset: UINT) -> HRESULT,
			fn BindConstantBufferByName(pName: LPCSTR, uDstSlot: UINT, cbDstOffset: UINT) -> HRESULT,
			fn BindResource(uSrcSlot: UINT, uDstSlot: UINT, uCount: UINT) -> HRESULT,
			fn BindResourceByName(pName: LPCSTR, uDstSlot: UINT, uCount: UINT) -> HRESULT,
			fn BindSampler(uSrcSlot: UINT, uDstSlot: UINT, uCount: UINT) -> HRESULT,
			fn BindSamplerByName(pName: LPCSTR, uDstSlot: UINT, uCount: UINT) -> HRESULT,
			fn BindUnorderedAccessView(uSrcSlot: UINT, uDstSlot: UINT, uCount: UINT) -> HRESULT,
			fn BindUnorderedAccessViewByName(pName: LPCSTR, uDstSlot: UINT, uCount: UINT) -> HRESULT,
			fn BindResourceAsUnorderedAccessView(uSrcSrvSlot: UINT, uDstUavSlot: UINT, uCount: UINT) -> HRESULT,
			fn BindResourceAsUnorderedAccessViewByName(pSrvName: LPCSTR, uDstUavSlot: UINT, uCount: UINT) -> HRESULT,
		}
		pub ID3D11ShaderReflectionVtbl of ID3D11ShaderReflection,
			pub trait ID3D11ShaderReflectionT
		{
			fn GetDesc(pDesc: *mut D3D11_SHADER_DESC) -> HRESULT,
			fn GetConstantBufferByIndex(Index: UINT) -> *mut ID3D11ShaderReflectionConstantBuffer,
			fn GetConstantBufferByName(Name: LPCSTR) -> *mut ID3D11ShaderReflectionConstantBuffer,
			fn GetResourceBindingDesc(ResourceIndex: UINT, pDesc: *mut D3D11_SHADER_INPUT_BIND_DESC) -> HRESULT,
			fn GetInputParameterDesc(ParameterIndex: UINT, pDesc: *mut D3D11_SIGNATURE_PARAMETER_DESC) -> HRESULT,
			fn GetOutputParameterDesc(ParameterIndex: UINT, pDesc: *mut D3D11_SIGNATURE_PARAMETER_DESC) -> HRESULT,
			fn GetPatchConstantParameterDesc(ParameterIndex: UINT, pDesc: *mut D3D11_SIGNATURE_PARAMETER_DESC) -> HRESULT,
			fn GetVariableByName(Name: LPCSTR) -> *mut ID3D11ShaderReflectionVariable,
			fn GetResourceBindingDescByName(Name: LPCSTR, pDesc: *mut D3D11_SHADER_INPUT_BIND_DESC) -> HRESULT,
			fn GetMovInstructionCount() -> UINT,
			fn GetMovcInstructionCount() -> UINT,
			fn GetConversionInstructionCount() -> UINT,
			fn GetBitwiseInstructionCount() -> UINT,
			fn GetGSInputPrimitive() -> D3D_PRIMITIVE,
			fn IsSampleFrequencyShader() -> BOOL,
			fn GetNumInterfaceSlots() -> UINT,
			fn GetMinFeatureLevel(pLevel: *mut D3D_FEATURE_LEVEL) -> HRESULT,
			fn GetThreadGroupSize(pSizeX: *mut UINT, pSizeY: *mut UINT, pSizeZ: *mut UINT) -> UINT,
			fn GetRequiresFlags() -> UINT64,
		}
		pub ID3D11ShaderTraceVtbl of ID3D11ShaderTrace, pub trait ID3D11ShaderTraceT {
			fn TraceReady(pTestCount: *mut UINT64) -> HRESULT,
			fn ResetTrace() -> (),
			fn GetTraceStats(pTraceStats: *mut D3D11_TRACE_STATS) -> HRESULT,
			fn PSSelectStamp(stampIndex: UINT) -> HRESULT,
			fn GetInitialRegisterContents(pRegister: *mut D3D11_TRACE_REGISTER, pValue: *mut D3D11_TRACE_VALUE) -> HRESULT,
			fn GetStep(stepIndex: UINT, pTraceStep: *mut D3D11_TRACE_STEP) -> HRESULT,
			fn GetWrittenRegister(stepIndex: UINT, writtenRegisterIndex: UINT, pRegister: *mut D3D11_TRACE_REGISTER, pValue: *mut D3D11_TRACE_VALUE) -> HRESULT,
			fn GetReadRegister(stepIndex: UINT, readRegisterIndex: UINT, pRegister: *mut D3D11_TRACE_REGISTER, pValue: *mut D3D11_TRACE_VALUE) -> HRESULT,
		}
		pub ID3D11ShaderTraceFactoryVtbl of ID3D11ShaderTraceFactory,
			pub trait ID3D11ShaderTraceFactoryT
		{
			fn CreateShaderTrace(pShader: *mut IUnknown, pTraceDesc: *mut D3D11_SHADER_TRACE_DESC, ppShaderTrace: *mut *mut ID3D11ShaderTrace) -> HRESULT,
		}
	]
	pub ID3D11FunctionReflectionVtbl of ID3D11FunctionReflection,
		pub trait ID3D11FunctionReflectionT
	{
		fn GetDesc(desc: *mut D3D11_PARAMETER_DESC) -> HRESULT,
	}
	pub ID3D11FunctionParameterReflectionVtbl of ID3D11FunctionParameterReflection,
		pub trait ID3D11FunctionParameterReflectionT
	{
		fn GetDesc(desc: *mut D3D11_PARAMETER_DESC) -> HRESULT,
	}
	pub ID3D11ShaderReflectionConstantBufferVtbl of ID3D11ShaderReflectionConstantBuffer,
		pub trait ID3D11ShaderReflectionConstantBufferT
	{
		fn GetDesc(pDesc: *mut D3D11_SHADER_BUFFER_DESC) -> HRESULT,
		fn GetVariableByIndex(Index: UINT) -> *mut ID3D11ShaderReflectionVariable,
		fn GetVariableByName(Name: LPCSTR) -> *mut ID3D11ShaderReflectionVariable,
	}
	pub ID3D11ShaderReflectionTypeVtbl of ID3D11ShaderReflectionType,
		pub trait ID3D11ShaderReflectionTypeT
	{
		fn GetDesc(pDesc: *mut D3D11_SHADER_TYPE_DESC) -> HRESULT,
		fn GetMemberTypeByIndex(Index: UINT) -> *mut ID3D11ShaderReflectionType,
		fn GetMemberTypeByName(Name: LPCSTR) -> *mut ID3D11ShaderReflectionType,
		fn GetMemberTypeName(Index: UINT) -> LPCSTR,
		fn IsEqual(pType: *mut ID3D11ShaderReflectionType) -> HRESULT,
		fn GetSubType() -> *mut ID3D11ShaderReflectionType,
		fn GetBaseClass() -> *mut ID3D11ShaderReflectionType,
		fn GetNumInterfaces() -> UINT,
		fn GetInterfaceByIndex(uIndex: UINT) -> *mut ID3D11ShaderReflectionType,
		fn IsOfType(pType: *mut ID3D11ShaderReflectionType) -> HRESULT,
		fn ImplementsInterface(pBase: *mut ID3D11ShaderReflectionType) -> HRESULT,
	}
	pub ID3D11ShaderReflectionVariableVtbl of ID3D11ShaderReflectionVariable,
		pub trait ID3D11ShaderReflectionVariableT
	{
		fn GetDesc(pDesc: *mut D3D11_SHADER_VARIABLE_DESC) -> HRESULT,
		fn GetType() -> *mut ID3D11ShaderReflectionType,
		fn GetBuffer() -> *mut ID3D11ShaderReflectionConstantBuffer,
		fn GetInterfaceSlot(uArrayIndex: UINT) -> UINT,
	}
}