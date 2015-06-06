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

#![allow(non_snake_case)]

use winapi::minwindef::*;
use winapi::basetsd::*;
use winapi::{ LPCSTR, HRESULT, LPCWSTR, GUID };
use dxgi::{ QueryIID, IUnknown, IUnknownVtbl, COMInterface };

use constants::*;
use common_version::enumerations::D3D_INCLUDE_TYPE;

pub type ID3DBlob = ID3D10Blob;

com_interface!{ ID3D10Blob(ID3D10BlobVtbl): IUnknown(IUnknownVtbl) {
	fn GetBufferPointer(&mut self) -> LPVOID,
	fn GetBufferSize(&mut self) -> SIZE_T
}}
com_interface!{ ID3DUserDefinedAnnotation(ID3DUserDefinedAnnotationVtbl): IUnknown(IUnknownVtbl) {
	fn BeginEvent(&mut self, Name: LPCWSTR) -> INT,
	fn EndEvent(&mut self) -> INT,
	fn SetMarker(&mut self, Name: LPCWSTR) -> (),
	fn GetStatus(&mut self) -> BOOL
}}
interface!{ ID3DInclude(ID3DIncludeVtbl) {
	fn Open(&mut self, IncludeType: D3D_INCLUDE_TYPE, pFileName: LPCSTR, pParentData: LPCVOID, ppData: *mut LPCVOID, pBytes: *mut UINT) -> HRESULT,
	fn Close(&mut self, pData: LPCVOID) -> HRESULT
}}

impl QueryIID for ID3D10Blob { fn iid() -> GUID { IID_ID3D10Blob } }
impl QueryIID for ID3DUserDefinedAnnotation { fn iid() -> GUID { IID_ID3DUserDefinedAnnotation } }
