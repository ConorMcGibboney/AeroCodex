#![forbid(unsafe_code)]
//! Procedural macro placeholders for Codex registration.
//!
//! These macros currently pass through their input. Implement metadata extraction later.

extern crate proc_macro;

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn codex_tool(_args: TokenStream, input: TokenStream) -> TokenStream {
    input
}

#[proc_macro_attribute]
pub fn codex_entry(_args: TokenStream, input: TokenStream) -> TokenStream {
    input
}

#[proc_macro_attribute]
pub fn codex_dataset(_args: TokenStream, input: TokenStream) -> TokenStream {
    input
}

#[proc_macro_attribute]
pub fn codex_workflow(_args: TokenStream, input: TokenStream) -> TokenStream {
    input
}
