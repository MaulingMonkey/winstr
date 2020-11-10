#![forbid(unsafe_code)]

use crate::*;

use winapi::shared::ntdef::LPCWSTR;

use std::borrow::Borrow;
use std::cmp::Ordering;
use std::ffi::{OsStr, OsString};
use std::fmt::{self, Debug, Display, Formatter};
use std::hash::{Hash, Hasher};
use std::os::windows::ffi::{OsStrExt, OsStringExt};
use std::path::PathBuf;



#[cfg(feature = "display    ")]
impl Display                for BString { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { Display::fmt(&**self, fmt) } }
impl Debug                  for BString { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { Debug::fmt(&**self, fmt) } }
impl AsRef<BStr>            for BString { fn as_ref(&self) -> &BStr { &**self } }
impl AsRef<[u16]>           for BString { fn as_ref(&self) -> &[u16] { self.units() } }
impl Borrow<BStr>           for BString { fn borrow(&self) -> &BStr { &**self } }
impl Borrow<[u16]>          for BString { fn borrow(&self) -> &[u16] { self.units() } }
impl Clone                  for BString { fn clone(&self) -> Self { Self::from_code_units(self.units().iter().cloned()).unwrap() } }
impl From<&BStr>            for BString { fn from(value: &BStr      ) -> Self { Self::from_bstr(value).unwrap() } }
impl From<&str>             for BString { fn from(value: &str       ) -> Self { Self::from_str(value).unwrap() } }
impl From<&String>          for BString { fn from(value: &String    ) -> Self { Self::from_str(value).unwrap() } }
impl From< String>          for BString { fn from(value:  String    ) -> Self { Self::from_str(&value).unwrap() } }
impl From<&OsStr>           for BString { fn from(value: &OsStr     ) -> Self { Self::from_osstr(value).unwrap() } }
impl From<&OsString>        for BString { fn from(value: &OsString  ) -> Self { Self::from_osstr(value).unwrap() } }
impl From< OsString>        for BString { fn from(value:  OsString  ) -> Self { Self::from_osstr(&value).unwrap() } }
impl PartialEq<BStr>        for BString { fn eq(&self, other: &BStr    ) -> bool { self.units() == other.units() } }
impl PartialEq<BString>     for BString { fn eq(&self, other: &BString ) -> bool { self.units() == other.units() } }
impl PartialEq<[u16]>       for BString { fn eq(&self, other: &[u16]   ) -> bool { self.units() == other } }
impl PartialEq<BString>     for [u16]   { fn eq(&self, other: &BString ) -> bool { other == self } }
impl Eq                     for BString {}
impl PartialOrd<BStr>       for BString { fn partial_cmp(&self, other: &BStr    ) -> Option<Ordering> { self.units().partial_cmp(other.units()) } }
impl PartialOrd<BString>    for BString { fn partial_cmp(&self, other: &BString ) -> Option<Ordering> { self.units().partial_cmp(other.units()) } }
impl PartialOrd<[u16]>      for BString { fn partial_cmp(&self, other: &[u16]   ) -> Option<Ordering> { self.units().partial_cmp(other) } }
impl PartialOrd<BString>    for [u16]   { fn partial_cmp(&self, other: &BString ) -> Option<Ordering> { other.partial_cmp(self) } }
impl Ord                    for BString { fn cmp(&self, other: &BString) -> Ordering { self.units().cmp(other.units()) } }
impl Hash                   for BString { fn hash<H: Hasher>(&self, state: &mut H) { self.units().hash(state) } }

#[cfg(feature = "display")]
impl Display                for BStr    { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { Display::fmt(&PathBuf::from(OsString::from_wide(self.units())).display(), fmt) } }
impl Debug                  for BStr    { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { Debug::fmt(&OsString::from_wide(self.units()), fmt) } }
impl AsRef<BStr>            for BStr    { fn as_ref(&self) -> &BStr { self } }
impl AsRef<[u16]>           for BStr    { fn as_ref(&self) -> &[u16] { self.units() } }
impl Borrow<[u16]>          for BStr    { fn borrow(&self) -> &[u16] { self.units() } }
impl PartialEq<BStr>        for BStr    { fn eq(&self, other: &BStr    ) -> bool { self.units() == other.units() } }
impl PartialEq<BString>     for BStr    { fn eq(&self, other: &BString ) -> bool { self.units() == other.units() } }
impl PartialEq<[u16]>       for BStr    { fn eq(&self, other: &[u16]   ) -> bool { self.units() == other } }
impl PartialEq<BStr>        for [u16]   { fn eq(&self, other: &BStr   ) -> bool { other == self } }
impl Eq                     for BStr    {}
impl PartialOrd<BStr>       for BStr    { fn partial_cmp(&self, other: &BStr    ) -> Option<Ordering> { self.units().partial_cmp(other.units()) } }
impl PartialOrd<BString>    for BStr    { fn partial_cmp(&self, other: &BString ) -> Option<Ordering> { self.units().partial_cmp(other.units()) } }
impl PartialOrd<[u16]>      for BStr    { fn partial_cmp(&self, other: &[u16]   ) -> Option<Ordering> { self.units().partial_cmp(other) } }
impl PartialOrd<BStr>       for [u16]   { fn partial_cmp(&self, other: &BStr    ) -> Option<Ordering> { other.partial_cmp(self) } }
impl Ord                    for BStr    { fn cmp(&self, other: &BStr) -> Ordering { self.units().cmp(other.units()) } }
impl Hash                   for BStr    { fn hash<H: Hasher>(&self, state: &mut H) { self.units().hash(state) } }



impl BString {
    /// Create a [BString] from a [str]
    pub fn from_str(s: impl AsRef<str>) -> Option<Self> { Self::from_code_units(ESI::new(s.as_ref().encode_utf16())) }

    /// Create a [BString] from a [OsStr]
    pub fn from_osstr(s: impl AsRef<OsStr>) -> Option<Self> { Self::from_code_units(ESI::new(s.as_ref().encode_wide())) }

    /// Create a [BString] from a [BStr]
    pub fn from_bstr(s: impl AsRef<BStr>) -> Option<Self> { Self::from_code_units(s.as_ref().units().iter().copied()) }
}



impl BStr {
    /// LPCWSTR / `* const wchar_t`
    pub fn as_lpcwstr(&self) -> LPCWSTR { self.as_bstr() }

    /// 32-bit length in [u16] unicode [code unit]s, including the implicit terminal `0u16`
    ///
    /// [code unit]:    https://unicode.org/glossary/#code_unit
    pub fn len320(&self) -> u32 { self.len32() + 1 }

    /// Length in [u16] unicode [code unit]s, excluding the implicit terminal `0u16`
    ///
    /// [code unit]:    https://unicode.org/glossary/#code_unit
    #[cfg(not(target_pointer_width = "16"))]
    pub fn len(&self) -> usize { self.len32() as usize }

    /// Length in [u16] unicode [code unit]s, including the implicit terminal `0u16`
    ///
    /// [code unit]:    https://unicode.org/glossary/#code_unit
    #[cfg(not(target_pointer_width = "16"))]
    pub fn len0(&self) -> usize { self.len320() as usize }

    /// The [u16] unicode [code unit]s of the string, excluding the terminal `0u16`
    ///
    /// [code unit]:    https://unicode.org/glossary/#code_unit
    #[cfg(not(target_pointer_width = "16"))]
    pub fn units(&self) -> &[u16] { let u = self.units0(); &u[..u.len()-1] }
}



/// "Exact Size Iterator" adapter
struct ESI<I: Iterator> {
    len:    usize,
    iter:   I,
}

impl<I: Iterator + Clone> ESI<I> {
    pub fn new(iter: I) -> Self {
        Self {
            len: iter.clone().count(),
            iter,
        }
    }
}

impl<I: Iterator> Iterator for ESI<I> {
    type Item = I::Item;
    fn next(&mut self) -> Option<Self::Item> { self.iter.next() }
}

impl<I: Iterator> ExactSizeIterator for ESI<I> {
    fn len(&self) -> usize { self.len }
}
