// The dangerous bits of the BSTR wrappers.  Only code within this module has
// access to `self.0`, hopefully making it simpler to audit access / verify
// struct invariants are held.

#![allow(unsafe_code)]

use winapi::shared::wtypes::BSTR;
use winapi::shared::wtypesbase::OLECHAR;
use winapi::um::oleauto::*;

use std::convert::TryInto;
use std::ops::{Deref, Drop};
use std::ptr::{null, null_mut, NonNull};



/// ### Length Invariants
/// A valid `BSTR` is always preceeded by a 4-byte length prefix:
/// * This length prefix excludes the terminal `0u16`
/// * This length prefix can be 0
/// * This length prefix can be u32::MAX for Win32 BSTRs
/// * This length prefix **is never u32::MAX for Rust BString**s, to avoid overflows when adding 1 to include the `0u16`
/// * `BSTR`s **cannot** be sliced and remain BSTRs.  You can slice the [u16] unicode [code unit]s instead.
///
/// ### Pointer Invariants
/// * Win32 BSTRs may be null.
/// * **Rust's [BString]s and &[BStr]s are never null** (instead, use [Option]&lt;BString&gt; or [Option]&lt;&amp;BStr&gt;).
    ///
    /// [code unit]:    https://unicode.org/glossary/#code_unit
mod invariants {}



/// `BString` is a non-null, owned, [BSTR] (32-bit length prefixed [UTF-16]ish string).
///
/// [BSTR]:     https://docs.microsoft.com/en-us/previous-versions/windows/desktop/automat/bstr
/// [UTF-16]:   https://en.wikipedia.org/wiki/UTF-16
#[repr(transparent)] pub struct BString(NonNull<OLECHAR>);

impl Deref for BString {
    type Target = BStr;
    fn deref(&self) -> &BStr { unsafe { std::mem::transmute(self.0) } }
}

impl Drop for BString {
    fn drop(&mut self) { unsafe { SysFreeString(self.0.as_ptr()) }; }
}

impl BString {
    /// Create an owned [`BSTR`] from 0 or more [u16] unicode code points.
    ///
    /// [`BSTR`]:   https://docs.microsoft.com/en-us/previous-versions/windows/desktop/automat/bstr
    pub fn from_code_units(mut code_units: impl ExactSizeIterator + Iterator<Item = u16>) -> Option<BString> {
        // NOTE:  It's technically "sound" for ExactSizeIterator to return
        // different lengths between calls.  To guard against such malice, we
        // call `len()` exactly once, and use that for both allocation and for
        // iteration dimensions.
        let len : usize = code_units.len();

        let len32 : u32 = len.try_into().ok()?;
        if len32 == std::u32::MAX { return None; } // Don't allow construction of strings where .len0() would overflow

        let bstr = unsafe { SysAllocStringLen(null(), len32) }; // Allocates [u16; len+1]

        // Important: early bail if bstr was null!
        // Important: free bstr if code_units.next() panics!
        let r = BString(NonNull::new(bstr)?);

        for off in 0..len {
            // Safe: off < len < len+1 == bstr alloc size
            unsafe { *bstr.add(off) = code_units.next().unwrap_or(0u16) };
        }
        // Safe: len < len+1 == bstr alloc size
        unsafe { *bstr.add(len) = 0u16 };
        Some(r)
    }
}



/// `&BStr` is a non-null, borrowed, [BSTR] (32-bit length prefixed [UTF-16]ish string).  Unlike &[OsStr](std::ffi::OsStr) or &[str], this is **not** a [DST]!
///
/// [BSTR]:     https://docs.microsoft.com/en-us/previous-versions/windows/desktop/automat/bstr
/// [DST]:      https://doc.rust-lang.org/nomicon/exotic-sizes.html#dynamically-sized-types-dsts
/// [UTF-16]:   https://en.wikipedia.org/wiki/UTF-16
#[repr(transparent)] pub struct BStr(OLECHAR);

impl BStr {
    /// Converts a &amp;[BSTR] into an Option&lt;&amp;[BStr]&gt;.
    /// By requiring a reference, this API [bounds] &amp;[BStr]'s lifetime, helping avoid bugs.
    ///
    /// ### Safety
    ///
    /// * `bstr` must be null, or a valid [BSTR] for the duration of `&BStr`'s lifetime.
    ///
    /// [BSTR]:         https://docs.microsoft.com/en-us/previous-versions/windows/desktop/automat/bstr
    /// [bounds]:       https://doc.rust-lang.org/nomicon/unbounded-lifetimes.html
    pub unsafe fn from_bstr(bstr: &BSTR) -> Option<&BStr> {
        let s : Option<&BStr> = std::mem::transmute(*bstr);
        if s?.len32() == std::u32::MAX { return None; } // Don't allow construction of strings where .len0() would overflow
        s
    }

    /// Converts a [BSTR] into an Option&lt;&amp;[BStr]&gt;.
    ///
    /// ### Safety
    ///
    /// * `bstr` must be null, or a valid [BSTR] for the duration of `&BStr`'s lifetime.
    /// * <span style="color: red">**&BStr's lifetime is [unbounded], an easy source of bugs.  Prefer [from_bstr]!**</span>
    ///
    /// [BSTR]:         https://docs.microsoft.com/en-us/previous-versions/windows/desktop/automat/bstr
    /// [unbounded]:    https://doc.rust-lang.org/nomicon/unbounded-lifetimes.html
    /// [from_bstr]:    #method.from_bstr
    pub unsafe fn from_bstr_unbounded<'b>(bstr: BSTR) -> Option<&'b BStr> {
        let s : Option<&BStr> = std::mem::transmute(bstr);
        if s?.len32() == std::u32::MAX { return None; } // Don't allow construction of strings where .len0() would overflow
        s
    }

    /// The `&BStr` as a winapi-friendly `BSTR`.
    ///
    /// ### Safety
    ///
    /// * `s.as_bstr()` is guaranteed to be `0u16`-terminated
    /// * It is **not** safe to modify the contents of the BSTR through the returned pointer!
    pub fn as_bstr(&self) -> BSTR { unsafe { std::mem::transmute(self) } }

    /// 32-bit length in [u16] unicode [code unit]s, excluding the implicit terminal `0u16`
    ///
    /// [code unit]:    https://unicode.org/glossary/#code_unit
    pub fn len32(&self) -> u32 { unsafe { SysStringLen(self.as_bstr()) } }

    /// The [u16] unicode [code unit]s of the string, including the terminal `0u16`
    ///
    /// [code unit]:    https://unicode.org/glossary/#code_unit
    #[cfg(not(target_pointer_width = "16"))]
    pub fn units0(&self) -> &[u16] { unsafe { std::slice::from_raw_parts(self.as_bstr(), self.len0()) } }
}



/// Utility trait for borrowing function arguments as [BSTR]s
///
/// ### Safety
///
/// By implementing this trait, you promise that [as_bstr_ptr] will return a valid, **non-null** [BSTR].
///
/// [as_bstr_ptr]:  #method.as_bstr_ptr
/// [BSTR]:         https://docs.microsoft.com/en-us/previous-versions/windows/desktop/automat/bstr
pub unsafe trait AsBStrPtr : AsRef<BStr> {
    /// Borrow `self` as a raw [BSTR]
    ///
    /// [BSTR]: https://docs.microsoft.com/en-us/previous-versions/windows/desktop/automat/bstr
    fn as_bstr_ptr(&self) -> BSTR { self.as_ref().as_bstr() }
}

unsafe impl<B: AsRef<BStr>> AsBStrPtr for B {}



/// Utility trait for borrowing function arguments as [BSTR]s or NULL
///
/// ### Safety
///
/// By implementing this trait, you promise that [as_bstr_ptr] will return a valid [BSTR] or null.
///
/// [as_bstr_ptr]:  #method.as_bstr_ptr
/// [BSTR]:         https://docs.microsoft.com/en-us/previous-versions/windows/desktop/automat/bstr
pub unsafe trait AsOptBStrPtr {
    /// Borrow `self` as a raw [BSTR]
    ///
    /// [BSTR]: https://docs.microsoft.com/en-us/previous-versions/windows/desktop/automat/bstr
    fn as_opt_bstr_ptr(&self) -> BSTR;
}

unsafe impl<B: AsRef<BStr>> AsOptBStrPtr for Option<B> {
    fn as_opt_bstr_ptr(&self) -> BSTR { self.as_ref().map_or(null_mut(), |s| s.as_ref().as_bstr()) }
}

unsafe impl<B: AsRef<BStr>> AsOptBStrPtr for B {
    fn as_opt_bstr_ptr(&self) -> BSTR { self.as_ref().as_bstr() }
}



#[test] fn layout() {
    use std::mem::align_of;
    use std::mem::size_of;

    assert_eq!(align_of::<&BStr>(),             align_of::<BSTR>());
    assert_eq!(align_of::<&BString>(),          align_of::<BSTR>());
    assert_eq!(align_of::< BString>(),          align_of::<BSTR>());
    assert_eq!(align_of::<Option<&BStr>>(),     align_of::<BSTR>());
    assert_eq!(align_of::<Option<&BString>>(),  align_of::<BSTR>());
    assert_eq!(align_of::<Option< BString>>(),  align_of::<BSTR>());

    assert_eq!(size_of::<&BStr>(),              size_of::<BSTR>());
    assert_eq!(size_of::<&BString>(),           size_of::<BSTR>());
    assert_eq!(size_of::< BString>(),           size_of::<BSTR>());
    assert_eq!(size_of::<Option<&BStr>>(),      size_of::<BSTR>());
    assert_eq!(size_of::<Option<&BString>>(),   size_of::<BSTR>());
    assert_eq!(size_of::<Option< BString>>(),   size_of::<BSTR>());
}

#[test] fn core_apis() {
    fn dbg<T: std::fmt::Debug>(v: &T) -> String { format!("{:?}", v) }

    let hello_world = "Hello, world!";
    let a = BString::from_code_units(hello_world.encode_utf16().collect::<Vec<_>>().into_iter()).unwrap();
    let b : &BStr = &a;
    let c = b.as_bstr();
    let d = unsafe { BStr::from_bstr(&c) }.unwrap();
    let e = unsafe { BStr::from_bstr_unbounded(c) }.unwrap();

    assert_eq!(dbg(&hello_world), dbg(&a));
    assert_eq!(dbg(&hello_world), dbg(&b));
    assert_eq!(dbg(&hello_world), dbg(&d));
    assert_eq!(dbg(&hello_world), dbg(&e));

    assert_eq!(c, a.as_bstr());
    assert_eq!(c, b.as_bstr());
    assert_eq!(c, d.as_bstr());
    assert_eq!(c, e.as_bstr());
    assert_eq!(a.len(), b.len()); // Plain ASCII
    assert_eq!(a.len(), d.len()); // Plain ASCII
    assert_eq!(a.len(), e.len()); // Plain ASCII
    assert_eq!(a.len(), d.len0()-1);
    assert_eq!(a.len(), e.len0()-1);
    assert_eq!(d.len(), d.len32() as usize);
    assert_eq!(e.len(), e.len32() as usize);
    assert_eq!(d.len0(), d.len320() as usize);
    assert_eq!(e.len0(), e.len320() as usize);

    assert!(hello_world.encode_utf16().eq(d.units().iter().copied()));
    assert!(hello_world.encode_utf16().eq(e.units().iter().copied()));
    assert!(hello_world.encode_utf16().chain(Some(0)).eq(d.units0().iter().copied()));
    assert!(hello_world.encode_utf16().chain(Some(0)).eq(e.units0().iter().copied()));
}