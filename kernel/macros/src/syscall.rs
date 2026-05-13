use std::fmt::format;

use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::{Error, Result, spanned::Spanned};

pub fn syscall_impl(input: syn::ItemFn) -> TokenStream {
    input.into_token_stream()
}
/// Checks if syscall is structured
pub fn check_syscall_sig(sig: &syn::ItemFn) -> Result<()> {
    let mut err: Option<Error> = None;
    // Checks if syscall
    match &sig.sig.unsafety {
        None => {
            let s = sig.sig.span();
            merge_err(
                &mut err,
                Error::new(
                    s,
                    format!(
                        "Function `{s:?}` is marked as safe. Needs to be marked as unsafe with the `unsafe` keyword"
                    ),
                ),
            );
        }
        Some(_) => { /*Nothing */ }
    }
    match &sig.sig.abi {
        None => {
            let s = sig.sig.span();
            merge_err(
                &mut err,
                Error::new(s, format!("Function `{s:?}` needs to be marked as `extern \"C\"`")),
            )
        }
        Some(a) if a.name.as_ref().is_some_and(|n| n.value() == "C") => { /* Valid signature */ }
        Some(a) if a.name.is_none() => {
            let s = sig.sig.span();
            merge_err(
                &mut err,
                Error::new(
                    s,
                    format!(
                        "Function `{s:?}` needs to be marked as `extern \"C\"`.\n{}",
                        "Not specifying a abi is trying to get phased out by the Rust team"
                    ),
                ),
            );
        }
        Some(a) if a.name.is_some() => {
            let abi = a.name.as_ref().unwrap().value();
            let s = sig.sig.span();
            merge_err(
                &mut err,
                Error::new(
                    s,
                    format!(
                        "Function `{s:?}` needs to be marked as `extern \"C\"`.\nWas marked with the {abi}"
                    ),
                ),
            );
        }
        Some(_) => {
            // We check both .is_some and .is_none on a.name
            unreachable!("Reached of unchecked abi?")
        }
    };
    //
    match err {
        Some(err) => Err(err),
        None => Ok(()),
    }
}
/// Similar to [`Error.combine()`] but takes a Option<Error>.
/// Allowing us to check all the paths, without
///
/// [`Error.combine()`]: Error::combine
fn merge_err(e: &mut Option<Error>, another: Error) {
    match e {
        Some(mx) => {
            mx.combine(another);
        }
        None => {
            *e = Some(another);
        }
    }
}
