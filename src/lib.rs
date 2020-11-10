#![deny(unsafe_code)]
#![cfg_attr(not(feature = "display"), allow(unused_imports))]

#[cfg(doc)] pub use doc::*;
#[cfg(doc)] mod doc {
    pub mod _alternatives;
    pub mod _features;
}

#[cfg(windows)] #[cfg(feature = "bstr")] #[path="bstring/_bstring.rs"] mod bstring;
#[cfg(windows)] #[cfg(feature = "bstr")] pub use bstring::*;
