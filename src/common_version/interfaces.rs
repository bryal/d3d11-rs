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

//! Common Version interfaces provided by D3D11
//!
//! # References
//! [Common Version Interfaces, MSDN]
//! (https://msdn.microsoft.com/en-us/library/windows/desktop/ff728662(v=vs.85).aspx)

use libc::c_void;
use winapi::{ LPVOID, SIZE_T, LPCVOID, LPCSTR, HRESULT,
	REFIID, ULONG, INT, BOOL, LPCWSTR,
	UINT };
use dxgi::IUnknownT;

use common_version::enumerations::{ D3D_INCLUDE_TYPE };

type ID3DBlob = ID3D10Blob;

#[repr(C)] pub struct ID3D10Blob { pub vtable: *mut ID3DBlobVtbl }
#[repr(C)] pub struct ID3DInclude { pub vtable: *mut ID3DIncludeVtbl }
#[repr(C)] pub struct ID3DUserDefinedAnnotation { pub vtable: *mut ID3DUserDefinedAnnotationVtbl }

c_vtable!{
	_ of (), trait IUnknownT {
		fn QueryInterface(riid: REFIID, object: *mut *mut c_void) -> HRESULT,
		fn AddRef() -> ULONG,
		fn Release() -> ULONG,
	} with heirs [
		pub ID3D10BlobVtbl of ID3D10Blob, pub trait ID3D10BlobT {
			fn GetBufferPointer() -> LPVOID,
			fn GetBufferSize() -> SIZE_T,
		}
		pub ID3DUserDefinedAnnotationVtbl of ID3DUserDefinedAnnotation, pub trait ID3DUserDefinedAnnotationT {
			fn BeginEvent(Name: LPCWSTR) -> INT,
			fn EndEvent() -> INT,
			fn SetMarker(Name: LPCWSTR) -> (),
			fn GetStatus() -> BOOL,
		}
	]
	pub ID3DIncludeVtbl of ID3DInclude, pub trait ID3DIncludeT {
		fn Open(IncludeType: D3D_INCLUDE_TYPE, pFileName: LPCSTR, pParentData: LPCVOID, ppData: *mut LPCVOID, pBytes: *mut UINT) -> HRESULT,
		fn Close(pData: LPCVOID) -> HRESULT,
	}
}