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

//! Structures used to create and use shaders
//!
//! # References
//! [Shader Structures, MSDN]
//! (https://msdn.microsoft.com/en-us/library/windows/desktop/ff476176(v=vs.85).aspx)

#![allow(non_snake_case, non_camel_case_types)]

use winapi::{ UINT, UINT8, UINT16, UINT64, INT, BOOL,
	BYTE, LPCSTR, LPVOID };

use common_version::enumerations::{ D3D_REGISTER_COMPONENT_TYPE, D3D_MIN_PRECISION,
	D3D_NAME, D3D_SHADER_VARIABLE_TYPE,
	D3D_SHADER_VARIABLE_CLASS, D3D_FEATURE_LEVEL,
	D3D_CBUFFER_TYPE, D3D_PRIMITIVE_TOPOLOGY,
	D3D_TESSELLATOR_PARTITIONING, D3D_TESSELLATOR_DOMAIN,
	D3D_SHADER_INPUT_TYPE, D3D_RESOURCE_RETURN_TYPE,
	D3D_SRV_DIMENSION, D3D_INTERPOLATION_MODE,
	D3D_PRIMITIVE, D3D_TESSELLATOR_OUTPUT_PRIMITIVE };
use shader::enumerations::{ D3D11_TRACE_GS_INPUT_PRIMITIVE, D3D11_TRACE_REGISTER_TYPE,
	D3D11_SHADER_TYPE, D3D_PARAMETER_FLAGS };

pub type D3D11_TRACE_COMPONENT_MASK = UINT8;
pub type D3D11_TRACE_MISC_OPERATIONS_MASK = UINT16;

#[repr(C)] pub struct D3D11_CLASS_INSTANCE_DESC {
	InstanceId: UINT,
	InstanceIndex: UINT,
	TypeId: UINT,
	ConstantBuffer: UINT,
	BaseConstantBufferOffset: UINT,
	BaseTexture: UINT,
	BaseSampler: UINT,
	Created: BOOL,
}
#[repr(C)] pub struct D3D11_COMPUTE_SHADER_TRACE_DESC {
	Invocation: UINT64,
	ThreadIDInGroup: [UINT; 3],
	ThreadGroupID: [UINT; 3],
}
#[repr(C)] pub struct D3D11_DOMAIN_SHADER_TRACE_DESC {
	Invocation: UINT64,
}
#[repr(C)] pub struct D3D11_FUNCTION_DESC {
	Version: UINT,                     // Shader version
	Creator: LPCSTR,                     // Creator string
	Flags: UINT,                       // Shader compilation/parse flags
	
	ConstantBuffers: UINT,             // Number of constant buffers
	BoundResources: UINT,              // Number of bound resources

	InstructionCount: UINT,            // Number of emitted instructions
	TempRegisterCount: UINT,           // Number of temporary registers used 
	TempArrayCount: UINT,              // Number of temporary arrays used
	DefCount: UINT,                    // Number of constant defines 
	DclCount: UINT,                    // Number of declarations (input + output)
	TextureNormalInstructions: UINT,   // Number of non-categorized texture instructions
	TextureLoadInstructions: UINT,     // Number of texture load instructions
	TextureCompInstructions: UINT,     // Number of texture comparison instructions
	TextureBiasInstructions: UINT,     // Number of texture bias instructions
	TextureGradientInstructions: UINT, // Number of texture gradient instructions
	FloatInstructionCount: UINT,       // Number of floating point arithmetic instructions used
	IntInstructionCount: UINT,         // Number of signed integer arithmetic instructions used
	UintInstructionCount: UINT,        // Number of unsigned integer arithmetic instructions used
	StaticFlowControlCount: UINT,      // Number of static flow control instructions used
	DynamicFlowControlCount: UINT,     // Number of dynamic flow control instructions used
	MacroInstructionCount: UINT,       // Number of macro instructions used
	ArrayInstructionCount: UINT,       // Number of array instructions used
	MovInstructionCount: UINT,         // Number of mov instructions used
	MovcInstructionCount: UINT,        // Number of movc instructions used
	ConversionInstructionCount: UINT,  // Number of type conversion instructions used
	BitwiseInstructionCount: UINT,     // Number of bitwise arithmetic instructions used
	MinFeatureLevel: D3D_FEATURE_LEVEL,             // Min target of the function byte code
	RequiredFeatureFlags: UINT64,        // Required feature flags

	Name: LPCSTR,                        // Function name
	FunctionParameterCount: INT,      // Number of logical parameters in the function signature (not including return)
	HasReturn: BOOL,                   // TRUE, if function returns a value, false - it is a subroutine
	Has10Level9VertexShader: BOOL,     // TRUE, if there is a 10L9 VS blob
	Has10Level9PixelShader: BOOL,      // TRUE, if there is a 10L9 PS blob
}
#[repr(C)] pub struct D3D11_GEOMETRY_SHADER_TRACE_DESC {
	Invocation: UINT64,
}
#[repr(C)] pub struct D3D11_HULL_SHADER_TRACE_DESC {
	Invocation: UINT64,
}
#[repr(C)] pub struct D3D11_LIBRARY_DESC {
	Creator: LPCSTR,           // The name of the originator of the library.
	Flags: UINT,             // Compilation flags.
	FunctionCount: UINT,     // Number of functions exported from the library.
}
#[repr(C)] pub struct D3D11_PARAMETER_DESC {
	Name: LPCSTR,               // Parameter name.
	SemanticName: LPCSTR,       // Parameter semantic name (+index).
	Type: D3D_SHADER_VARIABLE_TYPE,               // Element type.
	Class: D3D_SHADER_VARIABLE_CLASS,              // Scalar/Vector/Matrix.
	Rows: UINT,               // Rows are for matrix parameters.
	Columns: UINT,            // Components or Columns in matrix.
	InterpolationMode: D3D_INTERPOLATION_MODE,  // Interpolation mode.
	Flags: D3D_PARAMETER_FLAGS,              // Parameter modifiers.

	FirstInRegister: UINT,    // The first input register for this parameter.
	FirstInComponent: UINT,   // The first input register component for this parameter.
	FirstOutRegister: UINT,   // The first output register for this parameter.
	FirstOutComponent: UINT,  // The first output register component for this parameter.
}
#[repr(C)] pub struct D3D11_PIXEL_SHADER_TRACE_DESC {
	Invocation: UINT64,
	X: INT,
	Y: INT,
	SampleMask: UINT64,
}
#[repr(C)] pub struct D3D11_SHADER_BUFFER_DESC {
	Name: LPCSTR,           // Name of the constant buffer
	Type: D3D_CBUFFER_TYPE,           // Indicates type of buffer content
	Variables: UINT,      // Number of member variables
	Size: UINT,           // Size of CB (in bytes)
	uFlags: UINT,         // Buffer description flags
}
#[repr(C)] pub struct D3D11_SHADER_DESC {
	Version: UINT,                     // Shader version
	Creator: LPCSTR,                     // Creator string
	Flags: UINT,                       // Shader compilation/parse flags
	
	ConstantBuffers: UINT,             // Number of constant buffers
	BoundResources: UINT,              // Number of bound resources
	InputParameters: UINT,             // Number of parameters in the input signature
	OutputParameters: UINT,            // Number of parameters in the output signature

	InstructionCount: UINT,            // Number of emitted instructions
	TempRegisterCount: UINT,           // Number of temporary registers used 
	TempArrayCount: UINT,              // Number of temporary arrays used
	DefCount: UINT,                    // Number of constant defines 
	DclCount: UINT,                    // Number of declarations (input + output)
	TextureNormalInstructions: UINT,   // Number of non-categorized texture instructions
	TextureLoadInstructions: UINT,     // Number of texture load instructions
	TextureCompInstructions: UINT,     // Number of texture comparison instructions
	TextureBiasInstructions: UINT,     // Number of texture bias instructions
	TextureGradientInstructions: UINT, // Number of texture gradient instructions
	FloatInstructionCount: UINT,       // Number of floating point arithmetic instructions used
	IntInstructionCount: UINT,         // Number of signed integer arithmetic instructions used
	UintInstructionCount: UINT,        // Number of unsigned integer arithmetic instructions used
	StaticFlowControlCount: UINT,      // Number of static flow control instructions used
	DynamicFlowControlCount: UINT,     // Number of dynamic flow control instructions used
	MacroInstructionCount: UINT,       // Number of macro instructions used
	ArrayInstructionCount: UINT,       // Number of array instructions used
	CutInstructionCount: UINT,         // Number of cut instructions used
	EmitInstructionCount: UINT,        // Number of emit instructions used
	GSOutputTopology: D3D_PRIMITIVE_TOPOLOGY,            // Geometry shader output topology
	GSMaxOutputVertexCount: UINT,      // Geometry shader maximum output vertex count
	InputPrimitive: D3D_PRIMITIVE,              // GS/HS input primitive
	PatchConstantParameters: UINT,     // Number of parameters in the patch constant signature
	cGSInstanceCount: UINT,            // Number of Geometry shader instances
	cControlPoints: UINT,              // Number of control points in the HS->DS stage
	HSOutputPrimitive: D3D_TESSELLATOR_OUTPUT_PRIMITIVE,  // Primitive output by the tessellator
	HSPartitioning: D3D_TESSELLATOR_PARTITIONING,         // Partitioning mode of the tessellator
	TessellatorDomain: D3D_TESSELLATOR_DOMAIN,           // Domain of the tessellator (quad, tri, isoline)
	// instruction counts
	cBarrierInstructions: UINT,                           // Number of barrier instructions in a compute shader
	cInterlockedInstructions: UINT,                       // Number of interlocked instructions
	cTextureStoreInstructions: UINT,                      // Number of texture writes
}
#[repr(C)] pub struct D3D11_SHADER_INPUT_BIND_DESC
{
	Name: LPCSTR,           // Name of the resource
	Type: D3D_SHADER_INPUT_TYPE,           // Type of resource (e.g. texture, cbuffer, etc.)
	BindPoint: UINT,      // Starting bind point
	BindCount: UINT,      // Number of contiguous bind points (for arrays)
	
	uFlags: UINT,         // Input binding flags
	ReturnType: D3D_RESOURCE_RETURN_TYPE,     // Return type (if texture)
	Dimension: D3D_SRV_DIMENSION,      // Dimension (if texture)
	NumSamples: UINT,     // Number of samples (0 if not MS texture)
}
#[repr(C)] pub struct D3D11_SHADER_TRACE_DESC {
	Type: D3D11_SHADER_TYPE,
	Flags: UINT,
	union_0: D3D11_COMPUTE_SHADER_TRACE_DESC,
}
#[repr(C)] pub struct D3D11_SHADER_TYPE_DESC
{
	Class: D3D_SHADER_VARIABLE_CLASS,          // Variable class (e.g. object, matrix, etc.)
	Type: D3D_SHADER_VARIABLE_TYPE,           // Variable type (e.g. float, sampler, etc.)
	Rows: UINT,           // Number of rows (for matrices, 1 for other numeric, 0 if not applicable)
	Columns: UINT,        // Number of columns (for vectors & matrices, 1 for other numeric, 0 if not applicable)
	Elements: UINT,       // Number of elements (0 if not an array)
	Members: UINT,        // Number of members (0 if not a structure)
	Offset: UINT,         // Offset from the start of structure (0 if not a structure member)
	Name: LPCSTR,           // Name of type, can be NULL
}
#[repr(C)] pub struct D3D11_SHADER_VARIABLE_DESC
{
	Name: LPCSTR,           // Name of the variable
	StartOffset: UINT,    // Offset in constant buffer's backing store
	Size: UINT,           // Size of variable (in bytes)
	uFlags: UINT,         // Variable flags
	DefaultValue: LPVOID,   // Raw pointer to default value
	StartTexture: UINT,   // First texture index (or -1 if no textures used)
	TextureSize: UINT,    // Number of texture slots possibly used.
	StartSampler: UINT,   // First sampler index (or -1 if no textures used)
	SamplerSize: UINT,    // Number of sampler slots possibly used.
}
#[repr(C)] pub struct D3D11_SIGNATURE_PARAMETER_DESC
{
	SemanticName: LPCSTR,   // Name of the semantic
	SemanticIndex: UINT,  // Index of the semantic
	Register: UINT,       // Number of member variables
	SystemValueType: D3D_NAME,// A predefined system value, or D3D_NAME_UNDEFINED if not applicable
	ComponentType: D3D_REGISTER_COMPONENT_TYPE,  // Scalar type (e.g. uint, float, etc.)
	Mask: BYTE,           // Mask to indicate which components of the register
												// are used (combination of D3D10_COMPONENT_MASK values)
	ReadWriteMask: BYTE,  // Mask to indicate whether a given component is 
												// never written (if this is an output signature) or
												// always read (if this is an input signature).
												// (combination of D3D_MASK_* values)
	Stream: UINT,         // Stream index
	MinPrecision: D3D_MIN_PRECISION,   // Minimum desired interpolation precision
}
#[repr(C)] pub struct D3D11_TRACE_REGISTER {
	RegType: D3D11_TRACE_REGISTER_TYPE,
	union_0: [UINT16; 2],
	OperandIndex: UINT8,
	Flags: UINT8,
}
#[repr(C)] pub struct D3D11_TRACE_STATS {
	TraceDesc: D3D11_SHADER_TRACE_DESC,
	NumInvocationsInStamp: UINT8,
	TargetStampIndex: UINT8,
	NumTraceSteps: UINT,
	InputMask: [D3D11_TRACE_COMPONENT_MASK; 32],
	OutputMask: [D3D11_TRACE_COMPONENT_MASK; 32],
	NumTemps: UINT16,
	MaxIndexableTempIndex: UINT16,
	IndexableTempSize: [UINT16; 4096],
	ImmediateConstantBufferSize: UINT16,
	PixelPosition: [[UINT; 2]; 4],
	PixelCoverageMask: [UINT64; 4],
	PixelDiscardedMask: [UINT64; 4],
	PixelCoverageMaskAfterShader: [UINT64; 4],
	PixelCoverageMaskAfterA2CSampleMask: [UINT64; 4],
	PixelCoverageMaskAfterA2CSampleMaskDepth: [UINT64; 4],
	PixelCoverageMaskAfterA2CSampleMaskDepthStencil: [UINT64; 4],
	PSOutputsDepth: BOOL,
	PSOutputsMask: BOOL,
	GSInputPrimitive: D3D11_TRACE_GS_INPUT_PRIMITIVE,
	GSInputsPrimitiveID: BOOL,
	HSOutputPatchConstantMask: [D3D11_TRACE_COMPONENT_MASK; 32],
	DSInputPatchConstantMask: [D3D11_TRACE_COMPONENT_MASK; 32],
}
#[repr(C)] pub struct D3D11_TRACE_STEP {
	ID: UINT,
	InstructionActive: BOOL,
	NumRegistersWritten: UINT8,
	NumRegistersRead: UINT8,
	MiscOperations: D3D11_TRACE_MISC_OPERATIONS_MASK,
	OpcodeType: UINT,
	CurrentGlobalCycle: UINT64,
}
#[repr(C)] pub struct D3D11_TRACE_VALUE {
	Bits: [UINT; 4],
	ValidMask: D3D11_TRACE_COMPONENT_MASK,
}
#[repr(C)] pub struct D3D11_VERTEX_SHADER_TRACE_DESC {
	Invocation: UINT64,
}