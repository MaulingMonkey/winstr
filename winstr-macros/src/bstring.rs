#![cfg(feature = "bstr")]

use proc_macro::{TokenStream, TokenTree, Delimiter, Group, Ident, Literal, Punct, Spacing, Span};

use std::convert::{TryFrom, TryInto};
use std::iter::FromIterator;



pub(super) fn bstr_impl(input: TokenStream) -> TokenStream {
    let mut input = input.into_iter();

    let crate_ = match input.next() {
        Some(TokenTree::Group(g)) => match g.delimiter() {
            Delimiter::Brace        => return compile_error("expected `($crate)` as first token, got `{ ... }`", g.span()).into(),
            Delimiter::Bracket      => return compile_error("expected `($crate)` as first token, got `[ ... ]`", g.span()).into(),
            Delimiter::None         => return compile_error("expected `($crate)` as first token, got `Ø ... Ø`", g.span()).into(),
            Delimiter::Parenthesis  => g.stream(),
        },
        Some(tt)    => return compile_error(format!("expected `($crate)` as first token, got `{}`", tt), tt.span()).into(),
        None        => return compile_error("expected `($crate)` as first token, got nothing", Span::call_site()).into(),
    };

    let literal = match input.next() {
        Some(TokenTree::Literal(lit)) => {
            if let Some(unexpected) = input.next() {
                return compile_error(format!("bstr!(...) expects a single string argument, unexpected `{}` token after said argument", unexpected), unexpected.span()).into();
            }
            lit
        },
        Some(TokenTree::Group(g)) => match g.delimiter() {
            Delimiter::Brace        => return compile_error("expected `\"string\"` as second token, got `{ ... }`", g.span()).into(),
            Delimiter::Bracket      => return compile_error("expected `\"string\"` as second token, got `[ ... ]`", g.span()).into(),
            Delimiter::None         => return compile_error("expected `\"string\"` as second token, got `Ø ... Ø`", g.span()).into(),
            Delimiter::Parenthesis  => return compile_error("expected `\"string\"` as second token, got `( ... )`", g.span()).into(),
        },
        Some(tt)    => return compile_error(format!("expected `\"string\"` as second token, got `{}`", tt), tt.span()).into(),
        None        => return compile_error("expected string argument to bstr!() macro", Span::call_site()).into(),
    };

    let parsed_literal = match parse_str(&literal) {
        Ok(r) => r,
        Err(err) => return err,
    };

    let s = literal.span();
    let mut o = TokenStream::new();
    o.extend(crate_);
    o.extend(vec![
        ttp(':', Spacing::Joint, s),
        ttp(':', Spacing::Joint, s),
        ttid("BStr", s),
        ttp(':', Spacing::Joint, s),
        ttp(':', Spacing::Joint, s),
        ttid("bstr_macro_impl_detail", s),
        ttg(Delimiter::Parenthesis, s, vec![parsed_literal])
    ].into_iter());

    o
}

fn parse_str(literal: &Literal) -> Result<TokenTree, TokenStream> {
    let s = literal.span();

    let literal = literal.to_string();
    let literal = literal
        .strip_prefix("\"").ok_or_else(|| compile_error("expected string literal to start with `\"`", s))?
        .strip_suffix("\"").ok_or_else(|| compile_error("expected string literal to end with `\"`", s))?;

    let mut utf16 = Vec::new();
    let mut chars = literal.chars();
    while let Some(ch) = chars.next() {
        match ch {
            '\\' => {
                match chars.next() {
                    Some('0') => utf16.push(b'\0' as u16),
                    Some('t') => utf16.push(b'\t' as u16),
                    Some('n') => utf16.push(b'\n' as u16),
                    Some('r') => utf16.push(b'\r' as u16),
                    Some('\\') => utf16.push(b'\\' as u16),
                    Some('\'') => utf16.push(b'\'' as u16),
                    Some('\"') => utf16.push(b'\"' as u16),
                    Some('x') => {
                        let mut v = 0u16;
                        for _ in 0..2 {
                            let ch = chars.next().ok_or_else(|| compile_error("expected two hexidecimal characters after `\\x` escape sequence", s))?;
                            v = v * 16 + match ch {
                                ch @ '0' ..= '9'    => ch as u16 - b'0' as u16,
                                ch @ 'a' ..= 'f'    => ch as u16 - b'a' as u16 + 10,
                                ch @ 'A' ..= 'F'    => ch as u16 - b'A' as u16 + 10,
                                _                   => Err(compile_error("expected two hexidecimal characters after `\\x` escape sequence", s))?,
                            };
                        }
                        utf16.push(v);
                    },
                    Some('u') => {
                        let mut v = 0u32;
                        if chars.next() != Some('{') { Err(compile_error("expected `{` after `\\u` escape sequence", s))? }
                        for i in 0..7 {
                            let ch = chars.next().ok_or_else(|| compile_error("expected 1-6 hexidecimal characters in `\\u{...}` escape sequence", s))?;
                            v = v * 16 + match ch {
                                ch @ '0' ..= '9' if i != 6  => ch as u32 - b'0' as u32,
                                ch @ 'a' ..= 'f' if i != 6  => ch as u32 - b'a' as u32 + 10,
                                ch @ 'A' ..= 'F' if i != 6  => ch as u32 - b'A' as u32 + 10,
                                '}'              if i != 0  => break,
                                _                           => Err(compile_error("expected 1-6 hexidecimal characters in `\\u{...}` escape sequence", s))?,
                            };
                        }
                        let ch = char::try_from(v).map_err(|_| compile_error(format!("invalid unicode codepoint U+{:04X} in `\\u{{...}}` escape sequence", v), s))?;
                        let mut buf = [0, 0];
                        utf16.extend(ch.encode_utf16(&mut buf[..]).iter().copied());
                    },
                    Some(ch) => {
                        let mut buf = [0, 0];
                        utf16.extend(ch.encode_utf16(&mut buf[..]).iter().copied());
                    },
                    None => return Err(compile_error("expected character after `\\` in string", s).into()),
                }
            },
            ch => {
                let mut buf = [0, 0];
                utf16.extend(ch.encode_utf16(&mut buf[..]).iter().copied());
            },
        }
    }

    let cu_len32 : u32 = utf16.len().try_into().map_err(|_| compile_error("expected < 4GB bstr", s))?;
    if cu_len32 >= std::u32::MAX/2 { return Err(compile_error("expected < 4GB bstr", s).into()); }
    let bytes_len32 = 2 * cu_len32; // length prefix is *in bytes*, not code units!

    let mut tokens = vec![
        ttn(bytes_len32, s),
        ttp(',', Spacing::Joint, s),
    ];

    for cu in utf16[..].chunks(2) {
        let a = cu.get(0).copied().unwrap_or(0u16).to_ne_bytes();
        let b = cu.get(1).copied().unwrap_or(0u16).to_ne_bytes();
        let combined = u32::from_ne_bytes([a[0], a[1], b[0], b[1]]);
        tokens.push(ttn(combined, s));
        tokens.push(ttp(',', Spacing::Joint, s));
    }

    tokens.push(ttn(0, s));

    Ok(ttg(Delimiter::None, s, vec![
        ttp('&', Spacing::Joint, s),
        ttg(Delimiter::Bracket, s, tokens),
    ]))
}

fn ttid(string: &str, span: Span) -> TokenTree {
    Ident::new(string, span).into()
}

fn ttp(ch: char, spacing: Spacing, span: Span) -> TokenTree {
    let mut o = Punct::new(ch, spacing);
    o.set_span(span);
    o.into()
}

fn ttg(delimiter: Delimiter, span: Span, tts: impl IntoIterator<Item = TokenTree>) -> TokenTree {
    let mut o = Group::new(delimiter, TokenStream::from_iter(tts.into_iter()));
    o.set_span(span);
    o.into()
}

fn tts(str: impl AsRef<str>, span: Span) -> TokenTree {
    let mut o = Literal::string(str.as_ref());
    o.set_span(span);
    o.into()
}

fn ttn(n: u32, span: Span) -> TokenTree {
    let mut o = Literal::u32_unsuffixed(n);
    o.set_span(span);
    o.into()
}

fn compile_error(error: impl AsRef<str>, s: Span) -> TokenTree {
    ttg(Delimiter::None, s, vec![
        ttid("core", s),
        ttp(':', Spacing::Joint, s),
        ttp(':', Spacing::Joint, s),
        ttid("compile_error", s),
        ttp('!', Spacing::Joint, s),
        ttg(Delimiter::Parenthesis, s, vec![
            tts(error.as_ref(), s),
        ]),
    ])
}
