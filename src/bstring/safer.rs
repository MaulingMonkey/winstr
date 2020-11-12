#![forbid(unsafe_code)]

use crate::*;

use winapi::shared::ntdef::LPCWSTR;

use std::borrow::{Borrow, Cow};
use std::cmp::Ordering;
use std::ffi::{OsStr, OsString};
use std::fmt::{self, Debug, Display, Formatter};
use std::hash::{Hash, Hasher};
use std::os::windows::ffi::{OsStrExt, OsStringExt};
use std::path::{Path, PathBuf};



#[cfg(feature = "display")]
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
impl Eq                     for BString {}
impl Ord                    for BString { fn cmp(&self, other: &BString) -> Ordering { self.units().cmp(other.units()) } }
impl Hash                   for BString { fn hash<H: Hasher>(&self, state: &mut H) { self.units().hash(state) } }

#[cfg(feature = "display")]
impl Display                for BStr    { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { Display::fmt(&PathBuf::from(OsString::from_wide(self.units())).display(), fmt) } }
impl Debug                  for BStr    { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { Debug::fmt(&OsString::from_wide(self.units()), fmt) } }
impl AsRef<BStr>            for BStr    { fn as_ref(&self) -> &BStr { self } }
impl AsRef<[u16]>           for BStr    { fn as_ref(&self) -> &[u16] { self.units() } }
impl Borrow<[u16]>          for BStr    { fn borrow(&self) -> &[u16] { self.units() } }
impl Eq                     for &BStr   {}
impl Ord                    for &BStr   { fn cmp(&self, other: &&BStr) -> Ordering { self.units().cmp(other.units()) } }
impl Hash                   for &BStr   { fn hash<H: Hasher>(&self, state: &mut H) { self.units().hash(state) } }

// Okay, this is a *lot* of traits.  I'm just mimicing the stdlib here though.
//
// Sliceable DST rules, using `str` as an example
// 1.   Implement `&str == &str` (used for `"foo" == "bar"`)
// 2.   Implement `str  ==  str` (used for `"foo"[..] == "bar"[..]`)
// 3.   Skip `&str == str` / `str == &str`
// 4.   Implement `&str == ...`
// 5.   Implement `str  == ...`
// 6.   Implement `... == &str`
// 7.   Implement `... ==  str`
// 8.   All the above for ordering comparisons too
//
// `&BStr` is slightly simpler than `&str` - it is not sliceable and cannot be directly used as a value

macro_rules! peo {
    ( &? $left:ty, $($tt:tt)* ) => {
        peo!(& $left, $($tt)*);
        peo!(  $left, $($tt)*);
    };
    ( $left:ty, &? $($tt:tt)* ) => {
        peo!($left, & $($tt)*);
        peo!($left,   $($tt)*);
    };
    ( $left:ty, $right:ty ) => {
        impl PartialEq<$left> for $right {
            fn eq(&self, other: &$left) -> bool {
                self.utf16ish().eq(other.utf16ish())
            }
        }
        impl PartialOrd<$left> for $right {
            fn partial_cmp(&self, other: &$left) -> Option<Ordering> {
                self.utf16ish().partial_cmp(other.utf16ish())
            }
        }
    };
}

peo!(&BStr,   &BStr  );
peo!(BString, BString);

peo!(BString,           &BStr  ); peo!(&BStr,   BString         );
peo!(&?[u16],           &BStr  ); peo!(&BStr,   &?[u16]         ); // useful for wchar::wch! comparisons
peo!(&?str,             &BStr  ); peo!(&BStr,   &?str           );
peo!(String,            &BStr  ); peo!(&BStr,   String          );
peo!(&?OsStr,           &BStr  ); peo!(&BStr,   &?OsStr         );
peo!(OsString,          &BStr  ); peo!(&BStr,   OsString        );
peo!(&?Path,            &BStr  ); peo!(&BStr,   &?Path          );
peo!(PathBuf,           &BStr  ); peo!(&BStr,   PathBuf         );
peo!(Cow<'_, [u16]>,    &BStr  ); peo!(&BStr,   Cow<'_, [u16]>  );
peo!(Cow<'_, str>,      &BStr  ); peo!(&BStr,   Cow<'_, str>    );
peo!(Cow<'_, OsStr>,    &BStr  ); peo!(&BStr,   Cow<'_, OsStr>  );
peo!(Cow<'_, Path>,     &BStr  ); peo!(&BStr,   Cow<'_, Path>   );

//peo!(&BStr,           BString); peo!(BString, &BStr           ); // already covered
peo!(&?str,             BString); peo!(BString, &?str           );
peo!(&?[u16],           BString); peo!(BString, &?[u16]         );
peo!(String,            BString); peo!(BString, String          );
peo!(&?OsStr,           BString); peo!(BString, &?OsStr         );
peo!(OsString,          BString); peo!(BString, OsString        );
peo!(&?Path,            BString); peo!(BString, &?Path          );
peo!(PathBuf,           BString); peo!(BString, PathBuf         );
peo!(Cow<'_, [u16]>,    BString); peo!(BString, Cow<'_, [u16]>  );
peo!(Cow<'_, str>,      BString); peo!(BString, Cow<'_, str>    );
peo!(Cow<'_, OsStr>,    BString); peo!(BString, Cow<'_, OsStr>  );
peo!(Cow<'_, Path>,     BString); peo!(BString, Cow<'_, Path>   );



impl<'s> UTF16ish<'s> for BStr {
    type Iter = std::iter::Copied<std::slice::Iter<'s, u16>>;
    fn utf16ish(&'s self) -> Self::Iter { self.units().iter().copied() }
}

impl<'s> UTF16ish<'s> for BString {
    type Iter = std::iter::Copied<std::slice::Iter<'s, u16>>;
    fn utf16ish(&'s self) -> Self::Iter { self.units().iter().copied() }
}



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
