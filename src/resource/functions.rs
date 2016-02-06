//! Resource functions provided by D3D11
//!
//! # References
//! [Resource Functions, MSDN]
//! (https://msdn.microsoft.com/en-us/library/windows/desktop/ff476171(v=vs.85).aspx)

use winapi::UINT;

extern "C" {
    pub fn D3D11CalcSubresource(mip_slice: UINT, array_slice: UINT, mip_levels: UINT) -> UINT;
}
