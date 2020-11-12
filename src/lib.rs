#![deny(unsafe_code)]
#![cfg_attr(not(feature = "display"), allow(unused_imports))]

#[doc(hidden)] pub extern crate winstr_macros;

#[cfg(doc)] pub use doc::*;
#[cfg(doc)] mod doc {
    pub mod _alternatives;
    pub mod _features;
}

#[cfg(windows)] mod utf16ish;
#[cfg(windows)] use utf16ish::*;
#[cfg(windows)] #[cfg(feature = "bstr")] #[path="bstring/_bstring.rs"] mod bstring;
#[cfg(windows)] #[cfg(feature = "bstr")] pub use bstring::*;
