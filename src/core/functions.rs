//! Functions provided by D3D11
//!
//! # References
//! [D3D11 Functions, MSDN]
//! (https://msdn.microsoft.com/en-us/library/windows/desktop/ff476153(v=vs.85).aspx)

use winapi::{UINT, HRESULT, HMODULE};
use dxgi::{IDXGIAdapter, IDXGISwapChain, DXGI_SWAP_CHAIN_DESC};

use core::interfaces::{ID3D11Device, ID3D11DeviceContext};
use common_version::enumerations::{D3D_DRIVER_TYPE, D3D_FEATURE_LEVEL};

#[link(name="d3d11")]
extern "C" {
    pub fn D3D11CreateDevice(adapter: *mut IDXGIAdapter,
                             driver_type: D3D_DRIVER_TYPE,
                             software: HMODULE,
                             flags: UINT,
                             feature_levels: *const D3D_FEATURE_LEVEL,
                             num_feature_levels: UINT,
                             sdk_version: UINT,
                             device: *mut *mut ID3D11Device,
                             feature_level: *mut D3D_FEATURE_LEVEL,
                             immidiate_context: *mut *mut ID3D11DeviceContext)
                             -> HRESULT;

    pub fn D3D11CreateDeviceAndSwapChain(adapter: *mut IDXGIAdapter,
                                         driver_type: D3D_DRIVER_TYPE,
                                         software: HMODULE,
                                         flags: UINT,
                                         feature_levels: *const D3D_FEATURE_LEVEL,
                                         num_feature_levels: UINT,
                                         sdk_version: UINT,
                                         swapchain_desc: *const DXGI_SWAP_CHAIN_DESC,
                                         swapchain: *mut *mut IDXGISwapChain,
                                         device: *mut *mut ID3D11Device,
                                         feature_level: *mut D3D_FEATURE_LEVEL,
                                         immidiate_context: *mut *mut ID3D11DeviceContext)
                                         -> HRESULT;
}
