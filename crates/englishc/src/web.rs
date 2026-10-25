//! A tiny C ABI for the browser playground, with no bindgen.
//!
//! JavaScript calls `alloc`, writes UTF-8 into the buffer, calls one of the
//! entry points, then reads `output_len()` bytes from the returned pointer.

use std::cell::RefCell;

use crate::diagnostic::Level;
use crate::render::{Html, Renderer, summary};
use crate::{Config, check, explain, fix, fixable};

thread_local! {
    static OUTPUT: RefCell<Vec<u8>> = const { RefCell::new(Vec::new()) };
}

#[unsafe(no_mangle)]
pub extern "C" fn alloc(len: usize) -> *mut u8 {
    let mut buffer = Vec::<u8>::with_capacity(len);
    let ptr = buffer.as_mut_ptr();
    std::mem::forget(buffer);
    ptr
}

/// # Safety
/// `ptr` must come from `alloc(len)`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn dealloc(ptr: *mut u8, len: usize) {
    drop(unsafe { Vec::from_raw_parts(ptr, 0, len) });
}

#[unsafe(no_mangle)]
pub extern "C" fn output_len() -> usize {
    OUTPUT.with(|o| o.borrow().len())
}

fn input<'a>(ptr: *const u8, len: usize) -> &'a str {
    let bytes = unsafe { std::slice::from_raw_parts(ptr, len) };
    std::str::from_utf8(bytes).unwrap_or("")
}

fn output(text: String) -> *const u8 {
    OUTPUT.with(|o| {
        *o.borrow_mut() = text.into_bytes();
        o.borrow().as_ptr()
    })
}

fn json(s: &str) -> String {
    let mut out = String::from("\"");
    for c in s.chars() {
        match c {
            '"' => out += "\\\"",
            '\\' => out += "\\\\",
            '\n' => out += "\\n",
            '\r' => out += "\\r",
            '\t' => out += "\\t",
            c if (c as u32) < 0x20 => out += &format!("\\u{:04x}", c as u32),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

/// UTF-16 offset of a byte offset, because that is how JavaScript counts.
fn utf16(src: &str, byte: usize) -> usize {
    src[..byte.min(src.len())].encode_utf16().count()
}

/// Check `src` and return JSON: one entry per diagnostic with its rendered
/// HTML and its UTF-16 span (for underlining in the editor), plus the
/// summary line.
///
/// # Safety
/// `ptr` must point to `len` readable bytes.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn check_json(ptr: *const u8, len: usize) -> *const u8 {
    let src = input(ptr, len);
    let diagnostics = check(src, &Config::default());
    let renderer = Renderer::new(src, "essay.txt", &Html);
    let mut html = String::new();
    let errors = diagnostics
        .iter()
        .filter(|d| d.level == Level::Error)
        .count();
    let warnings = diagnostics.len() - errors;
    if let Some(line) = summary("essay.txt", errors, warnings, fixable(&diagnostics), &Html) {
        html += &line;
    }
    let items: Vec<String> = diagnostics
        .iter()
        .map(|d| {
            let span = d.span();
            format!(
                "{{\"start\":{},\"end\":{},\"level\":{},\"code\":{},\"lint\":{},\"message\":{},\"html\":{}}}",
                utf16(src, span.start),
                utf16(src, span.end),
                json(if d.level == Level::Error {
                    "error"
                } else {
                    "warning"
                }),
                d.code.map_or("null".to_string(), json),
                json(d.lint),
                json(&d.message),
                json(&renderer.diagnostic(d))
            )
        })
        .collect();
    output(format!(
        "{{\"errors\":{errors},\"warnings\":{warnings},\"fixable\":{},\"summary\":{},\"diagnostics\":[{}]}}",
        fixable(&diagnostics),
        json(&html),
        items.join(",")
    ))
}

/// Apply every machine-applicable fix and return the new text.
///
/// # Safety
/// `ptr` must point to `len` readable bytes.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fix_text(ptr: *const u8, len: usize) -> *const u8 {
    output(fix(input(ptr, len), &Config::default()).0)
}

/// Every lint, as JSON.
#[unsafe(no_mangle)]
pub extern "C" fn lints_json() -> *const u8 {
    let items: Vec<String> = crate::lints::LINTS
        .iter()
        .map(|l| {
            format!(
                "{{\"name\":{},\"code\":{},\"level\":{},\"summary\":{}}}",
                json(l.name),
                l.code.map_or("null".to_string(), json),
                json(if l.level == Level::Error {
                    "error"
                } else {
                    "warning"
                }),
                json(l.summary)
            )
        })
        .collect();
    output(format!("[{}]", items.join(",")))
}

/// `englishc --explain`.
///
/// # Safety
/// `ptr` must point to `len` readable bytes.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn explain_code(ptr: *const u8, len: usize) -> *const u8 {
    output(explain::explain(input(ptr, len)).unwrap_or("").to_string())
}
