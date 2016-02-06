//! Structures used to create and use shaders
//!
//! # References
//! [Shader Structures, MSDN]
//! (https://msdn.microsoft.com/en-us/library/windows/desktop/ff476176(v=vs.85).aspx)

#![allow(non_snake_case, non_camel_case_types)]

use winapi::minwindef::*;
use winapi::basetsd::*;
use winapi::LPCSTR;

use common_version::enumerations::*;
use shader::enumerations::*;

pub type D3D11_TRACE_COMPONENT_MASK = UINT8;
pub type D3D11_TRACE_MISC_OPERATIONS_MASK = UINT16;

#[repr(C)]
pub struct D3D11_CLASS_INSTANCE_DESC {
    pub InstanceId: UINT,
    pub InstanceIndex: UINT,
    pub TypeId: UINT,
    pub ConstantBuffer: UINT,
    pub BaseConstantBufferOffset: UINT,
    pub BaseTexture: UINT,
    pub BaseSampler: UINT,
    pub Created: BOOL,
}
#[repr(C)]
pub struct D3D11_COMPUTE_SHADER_TRACE_DESC {
    pub Invocation: UINT64,
    pub ThreadIDInGroup: [UINT; 3],
    pub ThreadGroupID: [UINT; 3],
}
#[repr(C)]
pub struct D3D11_DOMAIN_SHADER_TRACE_DESC {
    pub Invocation: UINT64,
}
#[repr(C)]
pub struct D3D11_FUNCTION_DESC {
    pub Version: UINT,
    pub Creator: LPCSTR,
    pub Flags: UINT,
    pub ConstantBuffers: UINT,
    pub BoundResources: UINT,
    pub InstructionCount: UINT,
    pub TempRegisterCount: UINT,
    pub TempArrayCount: UINT,
    pub DefCount: UINT,
    pub DclCount: UINT,
    pub TextureNormalInstructions: UINT,
    pub TextureLoadInstructions: UINT,
    pub TextureCompInstructions: UINT,
    pub TextureBiasInstructions: UINT,
    pub TextureGradientInstructions: UINT,
    pub FloatInstructionCount: UINT,
    pub IntInstructionCount: UINT,
    pub UintInstructionCount: UINT,
    pub StaticFlowControlCount: UINT,
    pub DynamicFlowControlCount: UINT,
    pub MacroInstructionCount: UINT,
    pub ArrayInstructionCount: UINT,
    pub MovInstructionCount: UINT,
    pub MovcInstructionCount: UINT,
    pub ConversionInstructionCount: UINT,
    pub BitwiseInstructionCount: UINT,
    pub MinFeatureLevel: D3D_FEATURE_LEVEL,
    pub RequiredFeatureFlags: UINT64,
    pub Name: LPCSTR,
    pub FunctionParameterCount: INT,
    pub HasReturn: BOOL,
    pub Has10Level9VertexShader: BOOL,
    pub Has10Level9PixelShader: BOOL,
}
#[repr(C)]
pub struct D3D11_GEOMETRY_SHADER_TRACE_DESC {
    pub Invocation: UINT64,
}
#[repr(C)]
pub struct D3D11_HULL_SHADER_TRACE_DESC {
    pub Invocation: UINT64,
}
#[repr(C)]
pub struct D3D11_LIBRARY_DESC {
    pub Creator: LPCSTR,
    pub Flags: UINT,
    pub FunctionCount: UINT,
}
#[repr(C)]
pub struct D3D11_PARAMETER_DESC {
    pub Name: LPCSTR,
    pub SemanticName: LPCSTR,
    pub Type: D3D_SHADER_VARIABLE_TYPE,
    pub Class: D3D_SHADER_VARIABLE_CLASS,
    pub Rows: UINT,
    pub Columns: UINT,
    pub InterpolationMode: D3D_INTERPOLATION_MODE,
    pub Flags: D3D_PARAMETER_FLAGS,
    pub FirstInRegister: UINT,
    pub FirstInComponent: UINT,
    pub FirstOutRegister: UINT,
    pub FirstOutComponent: UINT,
}
#[repr(C)]
pub struct D3D11_PIXEL_SHADER_TRACE_DESC {
    pub Invocation: UINT64,
    pub X: INT,
    pub Y: INT,
    pub SampleMask: UINT64,
}
#[repr(C)]
pub struct D3D11_SHADER_BUFFER_DESC {
    pub Name: LPCSTR,
    pub Type: D3D_CBUFFER_TYPE,
    pub Variables: UINT,
    pub Size: UINT,
    pub uFlags: UINT,
}
#[repr(C)]
pub struct D3D11_SHADER_DESC {
    pub Version: UINT,
    pub Creator: LPCSTR,
    pub Flags: UINT,
    pub ConstantBuffers: UINT,
    pub BoundResources: UINT,
    pub InputParameters: UINT,
    pub OutputParameters: UINT,
    pub InstructionCount: UINT,
    pub TempRegisterCount: UINT,
    pub TempArrayCount: UINT,
    pub DefCount: UINT,
    pub DclCount: UINT,
    pub TextureNormalInstructions: UINT,
    pub TextureLoadInstructions: UINT,
    pub TextureCompInstructions: UINT,
    pub TextureBiasInstructions: UINT,
    pub TextureGradientInstructions: UINT,
    pub FloatInstructionCount: UINT,
    pub IntInstructionCount: UINT,
    pub UintInstructionCount: UINT,
    pub StaticFlowControlCount: UINT,
    pub DynamicFlowControlCount: UINT,
    pub MacroInstructionCount: UINT,
    pub ArrayInstructionCount: UINT,
    pub CutInstructionCount: UINT,
    pub EmitInstructionCount: UINT,
    pub GSOutputTopology: D3D_PRIMITIVE_TOPOLOGY,
    pub GSMaxOutputVertexCount: UINT,
    pub InputPrimitive: D3D_PRIMITIVE,
    pub PatchConstantParameters: UINT,
    pub cGSInstanceCount: UINT,
    pub cControlPoints: UINT,
    pub HSOutputPrimitive: D3D_TESSELLATOR_OUTPUT_PRIMITIVE,
    pub HSPartitioning: D3D_TESSELLATOR_PARTITIONING,
    pub TessellatorDomain: D3D_TESSELLATOR_DOMAIN,
    pub cBarrierInstructions: UINT,
    pub cInterlockedInstructions: UINT,
    pub cTextureStoreInstructions: UINT,
}
#[repr(C)]
pub struct D3D11_SHADER_INPUT_BIND_DESC {
    pub Name: LPCSTR,
    pub Type: D3D_SHADER_INPUT_TYPE,
    pub BindPoint: UINT,
    pub BindCount: UINT,
    pub uFlags: UINT,
    pub ReturnType: D3D_RESOURCE_RETURN_TYPE,
    pub Dimension: D3D_SRV_DIMENSION,
    pub NumSamples: UINT,
}
#[repr(C)]
pub struct D3D11_SHADER_TRACE_DESC {
    pub Type: D3D11_SHADER_TYPE,
    pub Flags: UINT,
    pub union_0: D3D11_COMPUTE_SHADER_TRACE_DESC,
}
#[repr(C)]
pub struct D3D11_SHADER_TYPE_DESC {
    pub Class: D3D_SHADER_VARIABLE_CLASS,
    pub Type: D3D_SHADER_VARIABLE_TYPE,
    pub Rows: UINT,
    pub Columns: UINT,
    pub Elements: UINT,
    pub Members: UINT,
    pub Offset: UINT,
    pub Name: LPCSTR,
}
#[repr(C)]
pub struct D3D11_SHADER_VARIABLE_DESC {
    pub Name: LPCSTR,
    pub StartOffset: UINT,
    pub Size: UINT,
    pub uFlags: UINT,
    pub DefaultValue: LPVOID,
    pub StartTexture: UINT,
    pub TextureSize: UINT,
    pub StartSampler: UINT,
    pub SamplerSize: UINT,
}
#[repr(C)]
pub struct D3D11_SIGNATURE_PARAMETER_DESC {
    pub SemanticName: LPCSTR,
    pub SemanticIndex: UINT,
    pub Register: UINT,
    pub SystemValueType: D3D_NAME,
    pub ComponentType: D3D_REGISTER_COMPONENT_TYPE,
    pub Mask: BYTE,
    pub ReadWriteMask: BYTE,
    pub Stream: UINT,
    pub MinPrecision: D3D_MIN_PRECISION,
}
#[repr(C)]
pub struct D3D11_TRACE_REGISTER {
    pub RegType: D3D11_TRACE_REGISTER_TYPE,
    pub union_0: [UINT16; 2],
    pub OperandIndex: UINT8,
    pub Flags: UINT8,
}
#[repr(C)]
pub struct D3D11_TRACE_STATS {
    pub TraceDesc: D3D11_SHADER_TRACE_DESC,
    pub NumInvocationsInStamp: UINT8,
    pub TargetStampIndex: UINT8,
    pub NumTraceSteps: UINT,
    pub InputMask: [D3D11_TRACE_COMPONENT_MASK; 32],
    pub OutputMask: [D3D11_TRACE_COMPONENT_MASK; 32],
    pub NumTemps: UINT16,
    pub MaxIndexableTempIndex: UINT16,
    pub IndexableTempSize: [UINT16; 4096],
    pub ImmediateConstantBufferSize: UINT16,
    pub PixelPosition: [[UINT; 2]; 4],
    pub PixelCoverageMask: [UINT64; 4],
    pub PixelDiscardedMask: [UINT64; 4],
    pub PixelCoverageMaskAfterShader: [UINT64; 4],
    pub PixelCoverageMaskAfterA2CSampleMask: [UINT64; 4],
    pub PixelCoverageMaskAfterA2CSampleMaskDepth: [UINT64; 4],
    pub PixelCoverageMaskAfterA2CSampleMaskDepthStencil: [UINT64; 4],
    pub PSOutputsDepth: BOOL,
    pub PSOutputsMask: BOOL,
    pub GSInputPrimitive: D3D11_TRACE_GS_INPUT_PRIMITIVE,
    pub GSInputsPrimitiveID: BOOL,
    pub HSOutputPatchConstantMask: [D3D11_TRACE_COMPONENT_MASK; 32],
    pub DSInputPatchConstantMask: [D3D11_TRACE_COMPONENT_MASK; 32],
}
#[repr(C)]
pub struct D3D11_TRACE_STEP {
    pub ID: UINT,
    pub InstructionActive: BOOL,
    pub NumRegistersWritten: UINT8,
    pub NumRegistersRead: UINT8,
    pub MiscOperations: D3D11_TRACE_MISC_OPERATIONS_MASK,
    pub OpcodeType: UINT,
    pub CurrentGlobalCycle: UINT64,
}
#[repr(C)]
pub struct D3D11_TRACE_VALUE {
    pub Bits: [UINT; 4],
    pub ValidMask: D3D11_TRACE_COMPONENT_MASK,
}
#[repr(C)]
pub struct D3D11_VERTEX_SHADER_TRACE_DESC {
    pub Invocation: UINT64,
}
