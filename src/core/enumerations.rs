//! Core Enumerations provided by D3D11
//!
//! # References
//! [D3D11 Core Enumerations, MSDN]
//! (https://msdn.microsoft.com/en-us/library/windows/desktop/ff476152(v =vs.85).aspx)

#![allow(non_snake_case, non_camel_case_types)]

// CORE ENUMERATIONS

pub enum D3D11_ASYNC_GETDATA_FLAG {
    DONOTFLUSH = 0x1,
}

#[repr(C)]
pub enum D3D11_BLEND {
    ZERO = 1,
    ONE = 2,
    SRC_COLOR = 3,
    INV_SRC_COLOR = 4,
    SRC_ALPHA = 5,
    INV_SRC_ALPHA = 6,
    DEST_ALPHA = 7,
    INV_DEST_ALPHA = 8,
    DEST_COLOR = 9,
    INV_DEST_COLOR = 10,
    SRC_ALPHA_SAT = 11,
    BLEND_FACTOR = 14,
    INV_BLEND_FACTOR = 15,
    SRC1_COLOR = 16,
    INV_SRC1_COLOR = 17,
    SRC1_ALPHA = 18,
    INV_SRC1_ALPHA = 19,
}

#[repr(C)]
pub enum D3D11_BLEND_OP {
    ADD = 1,
    SUBTRACT = 2,
    REV_SUBTRACT = 3,
    MIN = 4,
    MAX = 5,
}

#[repr(C)]
pub enum D3D11_CLEAR_FLAG {
    DEPTH = 0x1,
    STENCIL = 0x2,
}

#[repr(C)]
pub enum D3D11_COLOR_WRITE_ENABLE {
    RED = 1,
    GREEN = 2,
    BLUE = 4,
    ALPHA = 8,
    ALL = (((D3D11_COLOR_WRITE_ENABLE::RED as isize |
		D3D11_COLOR_WRITE_ENABLE::GREEN as isize) |
			D3D11_COLOR_WRITE_ENABLE::BLUE as isize) |
				D3D11_COLOR_WRITE_ENABLE::ALPHA as isize),
}

#[repr(C)]
pub enum D3D11_COMPARISON_FUNC {
    NEVER = 1,
    LESS = 2,
    EQUAL = 3,
    LESS_EQUAL = 4,
    GREATER = 5,
    NOT_EQUAL = 6,
    GREATER_EQUAL = 7,
    ALWAYS = 8,
}

#[repr(C)]
pub enum D3D11_COPY_FLAGS {
    NO_OVERWRITE = 0x1,
    DISCARD = 0x2,
}

pub enum D3D11_COUNTER {
    DEVICE_DEPENDENT_0 = 0x40000000,
}

#[repr(C)]
pub enum D3D11_COUNTER_TYPE {
    FLOAT32 = 0,
    UINT16,
    UINT32,
    UINT64,
}

pub enum D3D11_1_CREATE_DEVICE_CONTEXT_STATE_FLAG {
    SINGLETHREADED = 0x1,
}

#[repr(C)]
pub enum D3D11_CULL_MODE {
    NONE = 1,
    FRONT = 2,
    BACK = 3,
}

#[repr(C)]
pub enum D3D11_DEPTH_WRITE_MASK {
    ZERO = 0,
    ALL = 1,
}

#[repr(C)]
pub enum D3D11_DEVICE_CONTEXT_TYPE {
    IMMEDIATE = 0,
    DEFERRED,
}

#[repr(C)]
pub enum D3D11_FEATURE {
    THREADING = 0,
    DOUBLES,
    FORMAT_SUPPORT,
    FORMAT_SUPPORT2,
    D3D10_X_HARDWARE_OPTIONS,
    D3D11_OPTIONS,
    ARCHITECTURE_INFO,
    D3D9_OPTIONS,
    SHADER_MIN_PRECISION_SUPPORT,
    D3D9_SHADOW_SUPPORT,
    D3D11_OPTIONS1,
    D3D9_SIMPLE_INSTANCING_SUPPORT,
    MARKER_SUPPORT,
    D3D9_OPTIONS1,
}

#[repr(C)]
pub enum D3D11_FILL_MODE {
    WIREFRAME = 2,
    SOLID = 3,
}

#[repr(C)]
pub enum D3D11_FILTER {
    MIN_MAG_MIP_POINT = 0,
    MIN_MAG_POINT_MIP_LINEAR = 0x1,
    MIN_POINT_MAG_LINEAR_MIP_POINT = 0x4,
    MIN_POINT_MAG_MIP_LINEAR = 0x5,
    MIN_LINEAR_MAG_MIP_POINT = 0x10,
    MIN_LINEAR_MAG_POINT_MIP_LINEAR = 0x11,
    MIN_MAG_LINEAR_MIP_POINT = 0x14,
    MIN_MAG_MIP_LINEAR = 0x15,
    ANISOTROPIC = 0x55,
    COMPARISON_MIN_MAG_MIP_POINT = 0x80,
    COMPARISON_MIN_MAG_POINT_MIP_LINEAR = 0x81,
    COMPARISON_MIN_POINT_MAG_LINEAR_MIP_POINT = 0x84,
    COMPARISON_MIN_POINT_MAG_MIP_LINEAR = 0x85,
    COMPARISON_MIN_LINEAR_MAG_MIP_POINT = 0x90,
    COMPARISON_MIN_LINEAR_MAG_POINT_MIP_LINEAR = 0x91,
    COMPARISON_MIN_MAG_LINEAR_MIP_POINT = 0x94,
    COMPARISON_MIN_MAG_MIP_LINEAR = 0x95,
    COMPARISON_ANISOTROPIC = 0xd5,
    MINIMUM_MIN_MAG_MIP_POINT = 0x100,
    MINIMUM_MIN_MAG_POINT_MIP_LINEAR = 0x101,
    MINIMUM_MIN_POINT_MAG_LINEAR_MIP_POINT = 0x104,
    MINIMUM_MIN_POINT_MAG_MIP_LINEAR = 0x105,
    MINIMUM_MIN_LINEAR_MAG_MIP_POINT = 0x110,
    MINIMUM_MIN_LINEAR_MAG_POINT_MIP_LINEAR = 0x111,
    MINIMUM_MIN_MAG_LINEAR_MIP_POINT = 0x114,
    MINIMUM_MIN_MAG_MIP_LINEAR = 0x115,
    MINIMUM_ANISOTROPIC = 0x155,
    MAXIMUM_MIN_MAG_MIP_POINT = 0x180,
    MAXIMUM_MIN_MAG_POINT_MIP_LINEAR = 0x181,
    MAXIMUM_MIN_POINT_MAG_LINEAR_MIP_POINT = 0x184,
    MAXIMUM_MIN_POINT_MAG_MIP_LINEAR = 0x185,
    MAXIMUM_MIN_LINEAR_MAG_MIP_POINT = 0x190,
    MAXIMUM_MIN_LINEAR_MAG_POINT_MIP_LINEAR = 0x191,
    MAXIMUM_MIN_MAG_LINEAR_MIP_POINT = 0x194,
    MAXIMUM_MIN_MAG_MIP_LINEAR = 0x195,
    MAXIMUM_ANISOTROPIC = 0x1d5,
}

#[repr(C)]
pub enum D3D11_FILTER_TYPE {
    POINT = 0,
    LINEAR = 1,
}

#[repr(C)]
pub enum D3D11_FORMAT_SUPPORT {
    BUFFER = 0x1,
    IA_VERTEX_BUFFER = 0x2,
    IA_INDEX_BUFFER = 0x4,
    SO_BUFFER = 0x8,
    TEXTURE1D = 0x10,
    TEXTURE2D = 0x20,
    TEXTURE3D = 0x40,
    TEXTURECUBE = 0x80,
    SHADER_LOAD = 0x100,
    SHADER_SAMPLE = 0x200,
    SHADER_SAMPLE_COMPARISON = 0x400,
    SHADER_SAMPLE_MONO_TEXT = 0x800,
    MIP = 0x1000,
    MIP_AUTOGEN = 0x2000,
    RENDER_TARGET = 0x4000,
    BLENDABLE = 0x8000,
    DEPTH_STENCIL = 0x10000,
    CPU_LOCKABLE = 0x20000,
    MULTISAMPLE_RESOLVE = 0x40000,
    DISPLAY = 0x80000,
    CAST_WITHIN_BIT_LAYOUT = 0x100000,
    MULTISAMPLE_RENDERTARGET = 0x200000,
    MULTISAMPLE_LOAD = 0x400000,
    SHADER_GATHER = 0x800000,
    BACK_BUFFER_CAST = 0x1000000,
    TYPED_UNORDERED_ACCESS_VIEW = 0x2000000,
    SHADER_GATHER_COMPARISON = 0x4000000,
    DECODER_OUTPUT = 0x8000000,
    VIDEO_PROCESSOR_OUTPUT = 0x10000000,
    VIDEO_PROCESSOR_INPUT = 0x20000000,
    VIDEO_ENCODER = 0x40000000,
}

#[repr(C)]
pub enum D3D11_FORMAT_SUPPORT2 {
    UAV_ATOMIC_ADD = 0x1,
    UAV_ATOMIC_BITWISE_OPS = 0x2,
    UAV_ATOMIC_COMPARE_STORE_OR_COMPARE_EXCHANGE = 0x4,
    UAV_ATOMIC_EXCHANGE = 0x8,
    UAV_ATOMIC_SIGNED_MIN_OR_MAX = 0x10,
    UAV_ATOMIC_UNSIGNED_MIN_OR_MAX = 0x20,
    UAV_TYPED_LOAD = 0x40,
    UAV_TYPED_STORE = 0x80,
    OUTPUT_MERGER_LOGIC_OP = 0x100,
    TILED = 0x200,
    SHAREABLE = 0x400,
}

#[repr(C)]
pub enum D3D11_INPUT_CLASSIFICATION {
    INPUT_PER_VERTEX_DATA = 0,
    INPUT_PER_INSTANCE_DATA = 1,
}

#[repr(C)]
pub enum D3D11_LOGIC_OP {
    CLEAR = 0,
    SET,
    COPY,
    COPY_INVERTED,
    NOOP,
    INVERT,
    AND,
    NAND,
    OR,
    NOR,
    XOR,
    EQUIV,
    AND_REVERSE,
    AND_INVERTED,
    OR_REVERSE,
    OR_INVERTED,
}

#[repr(C)]
pub enum D3D11_PRIMITIVE_TOPOLOGY {
    UNDEFINED = 0,
    POINTLIST = 1,
    LINELIST = 2,
    LINESTRIP = 3,
    TRIANGLELIST = 4,
    TRIANGLESTRIP = 5,
    LINELIST_ADJ = 10,
    LINESTRIP_ADJ = 11,
    TRIANGLELIST_ADJ = 12,
    TRIANGLESTRIP_ADJ = 13,
    CONTROL_POINT_PATCHLIST_1 = 33,
    CONTROL_POINT_PATCHLIST_2 = 34,
    CONTROL_POINT_PATCHLIST_3 = 35,
    CONTROL_POINT_PATCHLIST_4 = 36,
    CONTROL_POINT_PATCHLIST_5 = 37,
    CONTROL_POINT_PATCHLIST_6 = 38,
    CONTROL_POINT_PATCHLIST_7 = 39,
    CONTROL_POINT_PATCHLIST_8 = 40,
    CONTROL_POINT_PATCHLIST_9 = 41,
    CONTROL_POINT_PATCHLIST_10 = 42,
    CONTROL_POINT_PATCHLIST_11 = 43,
    CONTROL_POINT_PATCHLIST_12 = 44,
    CONTROL_POINT_PATCHLIST_13 = 45,
    CONTROL_POINT_PATCHLIST_14 = 46,
    CONTROL_POINT_PATCHLIST_15 = 47,
    CONTROL_POINT_PATCHLIST_16 = 48,
    CONTROL_POINT_PATCHLIST_17 = 49,
    CONTROL_POINT_PATCHLIST_18 = 50,
    CONTROL_POINT_PATCHLIST_19 = 51,
    CONTROL_POINT_PATCHLIST_20 = 52,
    CONTROL_POINT_PATCHLIST_21 = 53,
    CONTROL_POINT_PATCHLIST_22 = 54,
    CONTROL_POINT_PATCHLIST_23 = 55,
    CONTROL_POINT_PATCHLIST_24 = 56,
    CONTROL_POINT_PATCHLIST_25 = 57,
    CONTROL_POINT_PATCHLIST_26 = 58,
    CONTROL_POINT_PATCHLIST_27 = 59,
    CONTROL_POINT_PATCHLIST_28 = 60,
    CONTROL_POINT_PATCHLIST_29 = 61,
    CONTROL_POINT_PATCHLIST_30 = 62,
    CONTROL_POINT_PATCHLIST_31 = 63,
    CONTROL_POINT_PATCHLIST_32 = 64,
}

#[repr(C)]
pub enum D3D11_QUERY {
    EVENT = 0,
    OCCLUSION,
    TIMESTAMP,
    TIMESTAMP_DISJOINT,
    PIPELINE_STATISTICS,
    OCCLUSION_PREDICATE,
    SO_STATISTICS,
    SO_OVERFLOW_PREDICATE,
    SO_STATISTICS_STREAM0,
    SO_OVERFLOW_PREDICATE_STREAM0,
    SO_STATISTICS_STREAM1,
    SO_OVERFLOW_PREDICATE_STREAM1,
    SO_STATISTICS_STREAM2,
    SO_OVERFLOW_PREDICATE_STREAM2,
    SO_STATISTICS_STREAM3,
    SO_OVERFLOW_PREDICATE_STREAM3,
}

pub enum D3D11_QUERY_MISC_FLAG {
    PREDICATEHINT = 0x1,
}

pub enum D3D11_RAISE_FLAG {
    DRIVER_INTERNAL_ERROR = 0x1,
}

#[repr(C)]
pub enum D3D11_SHADER_MIN_PRECISION_SUPPORT {
    MP_10_BIT = 0x1,
    MP_16_BIT = 0x2,
}

#[repr(C)]
pub enum D3D11_STENCIL_OP {
    KEEP = 1,
    ZERO = 2,
    REPLACE = 3,
    INCR_SAT = 4,
    DECR_SAT = 5,
    INVERT = 6,
    INCR = 7,
    DECR = 8,
}

#[repr(C)]
pub enum D3D11_TEXTURE_ADDRESS_MODE {
    WRAP = 1,
    MIRROR = 2,
    CLAMP = 3,
    BORDER = 4,
    MIRROR_ONCE = 5,
}

#[repr(C)]
pub enum D3D11_TEXTURECUBE_FACE {
    POSITIVE_X = 0,
    NEGATIVE_X = 1,
    POSITIVE_Y = 2,
    NEGATIVE_Y = 3,
    POSITIVE_Z = 4,
    NEGATIVE_Z = 5,
}

#[repr(C)]
pub enum D3D11_TILED_RESOURCES_TIER {
    NOT_SUPPORTED = 0,
    TIER_1 = 1,
    TIER_2 = 2,
}
