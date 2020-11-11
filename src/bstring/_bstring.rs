#[cfg(doc)]
#[macro_export]
/// Create a &[BStr] literal at compile time
macro_rules! bstr {
    ( $string:literal ) => {
        winstr_macros::bstr_impl!(($crate) $string)
    };
}

#[cfg(not(doc))] // use wildcards for better error messages from proc macro
#[macro_export]
macro_rules! bstr {
    ( $($tt:tt)+ ) => {
        winstr_macros::bstr_impl!(($crate) $($tt)+)
    };
}

mod danger; pub use danger::*;
mod safer;
mod traits; pub use traits::*;
