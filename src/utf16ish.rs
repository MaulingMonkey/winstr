use std::ffi::{OsStr, OsString};
use std::path::{Path, PathBuf};



pub(crate) trait UTF16ish<'s> {
    type Iter : Iterator<Item = u16> + 's;
    fn utf16ish(&'s self) -> Self::Iter;
}



impl<'s, T: UTF16ish<'s>> UTF16ish<'s> for &'s T {
    type Iter = T::Iter;
    fn utf16ish(&self) -> Self::Iter { UTF16ish::utf16ish(&**self) }

}

impl<'s, T: UTF16ish<'s> + ToOwned> UTF16ish<'s> for std::borrow::Cow<'s, T> {
    type Iter = T::Iter;
    fn utf16ish(&'s self) -> Self::Iter { UTF16ish::utf16ish(&**self) }

}

impl<'s> UTF16ish<'s> for [u16] {
    type Iter = std::iter::Copied<std::slice::Iter<'s, u16>>;
    fn utf16ish(&'s self) -> Self::Iter { self.iter().copied() }
}

impl<'s> UTF16ish<'s> for str {
    type Iter = std::str::EncodeUtf16<'s>;
    fn utf16ish(&'s self) -> Self::Iter { self.encode_utf16() }
}

impl<'s> UTF16ish<'s> for String {
    type Iter = std::str::EncodeUtf16<'s>;
    fn utf16ish(&'s self) -> Self::Iter { self.encode_utf16() }
}

impl<'s> UTF16ish<'s> for OsStr {
    type Iter = std::os::windows::ffi::EncodeWide<'s>;
    fn utf16ish(&'s self) -> Self::Iter { std::os::windows::ffi::OsStrExt::encode_wide(self) }
}

impl<'s> UTF16ish<'s> for OsString {
    type Iter = std::os::windows::ffi::EncodeWide<'s>;
    fn utf16ish(&'s self) -> Self::Iter { std::os::windows::ffi::OsStrExt::encode_wide(&**self) }
}

impl<'s> UTF16ish<'s> for Path {
    type Iter = std::os::windows::ffi::EncodeWide<'s>;
    fn utf16ish(&'s self) -> Self::Iter { std::os::windows::ffi::OsStrExt::encode_wide(self.as_os_str()) }
}

impl<'s> UTF16ish<'s> for PathBuf {
    type Iter = std::os::windows::ffi::EncodeWide<'s>;
    fn utf16ish(&'s self) -> Self::Iter { std::os::windows::ffi::OsStrExt::encode_wide(self.as_os_str()) }
}
