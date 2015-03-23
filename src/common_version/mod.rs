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

//! "The Direct3D API defines several API elements that are common to the Direct3D 11, Direct3D 10,
//! and Direct3D 10.1 versions, and versions later than Direct3D 11.
//! You can use these API elements in your code for any of these Direct3D versions.
//! These API elements are known as version neutral."
//!
//! # References
//! [Common Version Reference, MSDN]
//! (https://msdn.microsoft.com/en-us/library/windows/desktop/ff728660(v=vs.85).aspx)

pub use self::enumerations::*;
pub use self::structures::*;
pub use self::interfaces::*;

pub mod enumerations;
pub mod structures;
pub mod interfaces;