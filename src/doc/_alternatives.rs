//! Alternatives crates I'm competing with
//!
//! ### widestring
//!
//! \[[docs.rs](https://docs.rs/widestring/), [github](https://github.com/starkat99/widestring-rs)\]
//!
//! Pros:
//! * More widely used
//! * More widely audited
//! * More types (UTF-32)
//!
//! Cons:
//! * Thousands of lines of code is annoying to audit
//! * Rife with `unsafe` fns, [inconsistent docs](https://github.com/starkat99/widestring-rs/issues/18) about safety guarantees
//! * No `BSTR` support
//!
//! ### oaidl
//!
//! \[[docs.rs](https://docs.rs/oaidl/), [github](https://github.com/zerothlaw/oaidl-safe)\]
//!
//! Pros:
//! * Technically has `BSTR` support... techically
//!
//! Cons:
//! * Even more code
//! * [Unsound](https://github.com/ZerothLaw/oaidl-safe/issues/6)
//! * BSTR support is extremely limited, allocation-prone
//!
//! ### ???
//!
//! Feel free to file issues for more alternatives - competition is good!
