//! Common Version structures provided
//!
//! # References
//! [Common Version Structures, MSDN]
//! (https://msdn.microsoft.com/en-us/library/windows/desktop/ff728663(v=vs.85).aspx)

#![allow(non_snake_case, non_camel_case_types)]

use winapi::LPCSTR;

#[repr(C)]
pub struct D3D_SHADER_MACRO {
    pub Name: LPCSTR,
    pub Definition: LPCSTR,
}
