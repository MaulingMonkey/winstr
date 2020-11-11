extern crate proc_macro;

#[cfg(windows)] mod bstring;

#[cfg(feature = "bstr")] #[cfg(windows)] #[proc_macro]
pub fn bstr_impl(input: proc_macro::TokenStream) -> proc_macro::TokenStream { bstring::bstr_impl(input) }
