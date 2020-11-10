use crate::*;

use std::ffi::{OsStr, OsString};
use std::path::{Path, PathBuf};



/// Similar to `Into<BString>`, except that this won't require copying `&BStr` or `&BString` arguments.
pub trait IntoBStr {
    type BStr : AsBStrPtr;
    fn into(self) -> Self::BStr;
}

impl<B: AsRef<BStr>> IntoBStr for B {
    type BStr = Self;
    fn into(self) -> Self { self }
}

impl IntoBStr for  String   { type BStr = BString; fn into(self) -> BString { BString::from_str(&self).unwrap() } }
impl IntoBStr for &String   { type BStr = BString; fn into(self) -> BString { BString::from_str( self).unwrap() } }
impl IntoBStr for &str      { type BStr = BString; fn into(self) -> BString { BString::from_str( self).unwrap() } }
impl IntoBStr for  OsString { type BStr = BString; fn into(self) -> BString { BString::from_osstr(&self).unwrap() } }
impl IntoBStr for &OsString { type BStr = BString; fn into(self) -> BString { BString::from_osstr( self).unwrap() } }
impl IntoBStr for &OsStr    { type BStr = BString; fn into(self) -> BString { BString::from_osstr( self).unwrap() } }
impl IntoBStr for  PathBuf  { type BStr = BString; fn into(self) -> BString { BString::from_osstr(&self).unwrap() } }
impl IntoBStr for &PathBuf  { type BStr = BString; fn into(self) -> BString { BString::from_osstr( self).unwrap() } }
impl IntoBStr for &Path     { type BStr = BString; fn into(self) -> BString { BString::from_osstr( self).unwrap() } }



/// Similar to `Into<Option<BString>>`, except that this won't require copying
/// `&BStr` or `&BString` arguments.  Additionally, you can use `()` in lieu of
/// `None` - whereas trying to pass `None` to a function accepting
/// `Into<Option<BString>>` would cause ambiguous type errors.
pub trait IntoOptBStr {
    type OptBStr : AsOptBStrPtr;
    fn into(self) -> Self::OptBStr;
}

impl<B: AsOptBStrPtr + Sized> IntoOptBStr for B {
    type OptBStr = Self;
    fn into(self) -> Self { self }
}

impl IntoOptBStr for ()        { type OptBStr = Option<BString>; fn into(self) -> Option<BString> { None } }

impl IntoOptBStr for  String   { type OptBStr = Option<BString>; fn into(self) -> Option<BString> { BString::from_str(&self) } }
impl IntoOptBStr for &String   { type OptBStr = Option<BString>; fn into(self) -> Option<BString> { BString::from_str( self) } }
impl IntoOptBStr for &str      { type OptBStr = Option<BString>; fn into(self) -> Option<BString> { BString::from_str( self) } }
impl IntoOptBStr for  OsString { type OptBStr = Option<BString>; fn into(self) -> Option<BString> { BString::from_osstr(&self) } }
impl IntoOptBStr for &OsString { type OptBStr = Option<BString>; fn into(self) -> Option<BString> { BString::from_osstr( self) } }
impl IntoOptBStr for &OsStr    { type OptBStr = Option<BString>; fn into(self) -> Option<BString> { BString::from_osstr( self) } }
impl IntoOptBStr for  PathBuf  { type OptBStr = Option<BString>; fn into(self) -> Option<BString> { BString::from_osstr(&self) } }
impl IntoOptBStr for &PathBuf  { type OptBStr = Option<BString>; fn into(self) -> Option<BString> { BString::from_osstr( self) } }
impl IntoOptBStr for &Path     { type OptBStr = Option<BString>; fn into(self) -> Option<BString> { BString::from_osstr( self) } }

impl IntoOptBStr for Option< String  > { type OptBStr = Option<BString>; fn into(self) -> Option<BString> { self.and_then(|s| BString::from_str(&s)) } }
impl IntoOptBStr for Option<&String  > { type OptBStr = Option<BString>; fn into(self) -> Option<BString> { self.and_then(|s| BString::from_str( s)) } }
impl IntoOptBStr for Option<&str     > { type OptBStr = Option<BString>; fn into(self) -> Option<BString> { self.and_then(|s| BString::from_str( s)) } }
impl IntoOptBStr for Option< OsString> { type OptBStr = Option<BString>; fn into(self) -> Option<BString> { self.and_then(|s| BString::from_osstr(&s)) } }
impl IntoOptBStr for Option<&OsString> { type OptBStr = Option<BString>; fn into(self) -> Option<BString> { self.and_then(|s| BString::from_osstr( s)) } }
impl IntoOptBStr for Option<&OsStr   > { type OptBStr = Option<BString>; fn into(self) -> Option<BString> { self.and_then(|s| BString::from_osstr( s)) } }
impl IntoOptBStr for Option< PathBuf > { type OptBStr = Option<BString>; fn into(self) -> Option<BString> { self.and_then(|s| BString::from_osstr(&s)) } }
impl IntoOptBStr for Option<&PathBuf > { type OptBStr = Option<BString>; fn into(self) -> Option<BString> { self.and_then(|s| BString::from_osstr( s)) } }
impl IntoOptBStr for Option<&Path    > { type OptBStr = Option<BString>; fn into(self) -> Option<BString> { self.and_then(|s| BString::from_osstr( s)) } }

impl IntoOptBStr for &Option< String  > { type OptBStr = Option<BString>; fn into(self) -> Option<BString> { self.as_ref().and_then(|s| BString::from_str(s)) } }
impl IntoOptBStr for &Option<&String  > { type OptBStr = Option<BString>; fn into(self) -> Option<BString> { self.and_then(|s| BString::from_str(s)) } }
impl IntoOptBStr for &Option<&str     > { type OptBStr = Option<BString>; fn into(self) -> Option<BString> { self.and_then(|s| BString::from_str(s)) } }
impl IntoOptBStr for &Option< OsString> { type OptBStr = Option<BString>; fn into(self) -> Option<BString> { self.as_ref().and_then(|s| BString::from_osstr(s)) } }
impl IntoOptBStr for &Option<&OsString> { type OptBStr = Option<BString>; fn into(self) -> Option<BString> { self.and_then(|s| BString::from_osstr(s)) } }
impl IntoOptBStr for &Option<&OsStr   > { type OptBStr = Option<BString>; fn into(self) -> Option<BString> { self.and_then(|s| BString::from_osstr(s)) } }
impl IntoOptBStr for &Option< PathBuf > { type OptBStr = Option<BString>; fn into(self) -> Option<BString> { self.as_ref().and_then(|s| BString::from_osstr(s)) } }
impl IntoOptBStr for &Option<&PathBuf > { type OptBStr = Option<BString>; fn into(self) -> Option<BString> { self.and_then(|s| BString::from_osstr(s)) } }
impl IntoOptBStr for &Option<&Path    > { type OptBStr = Option<BString>; fn into(self) -> Option<BString> { self.and_then(|s| BString::from_osstr(s)) } }
