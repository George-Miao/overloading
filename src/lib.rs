#![doc = include_str!("../README.md")]

use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{parse_macro_input, spanned::Spanned, Signature};

extern crate proc_macro;
use syn::ItemFn;

#[proc_macro_attribute]
pub fn overloading(
    attrs: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let ItemFn {
        vis, sig, block, ..
    } = parse_macro_input!(input as ItemFn);

    let error = filter_sig(&sig);
    if !error.is_empty() {
        return error.into();
    }
    let Signature {
        mut ident,
        generics,
        inputs,
        output,
        ..
    } = sig;
    let struct_def = if attrs.is_empty() {
        quote! {
            #[allow(non_camel_case_types)]
            #vis struct #ident;
        }
    } else {
        let new_ident = parse_macro_input!(attrs as Ident);
        ident = new_ident;
        quote!()
    };

    let mut tys = vec![];
    let mut pats = vec![];
    for input in inputs {
        match input {
            syn::FnArg::Receiver(kw) => {
                return syn::Error::new(kw.span(), "Method is not implemented")
                    .into_compile_error()
                    .into();
            }
            syn::FnArg::Typed(typed) => {
                tys.push(typed.ty.clone());
                pats.push(typed.pat);
            }
        }
    }
    let output = match output {
        syn::ReturnType::Default => quote!(()),
        syn::ReturnType::Type(_, ty) => quote!(#ty),
    };
    let pat = if pats.is_empty() {
        quote!(_)
    } else {
        quote!( (#(#pats),*,) )
    };
    let arg = if tys.is_empty() {
        quote!(())
    } else {
        quote!( (#(#tys),*,) )
    };

    quote! {
        #struct_def

        impl #generics std::ops::FnOnce<#arg> for #ident {
            type Output = #output;

            extern "rust-call" fn call_once(self, #pat: #arg) -> Self::Output #block
        }

        impl #generics std::ops::FnMut<#arg> for #ident {
            extern "rust-call" fn call_mut(&mut self, #pat: #arg) -> Self::Output #block
        }

        impl #generics std::ops::Fn<#arg> for #ident {
            extern "rust-call" fn call(&self, #pat: #arg) -> Self::Output #block
        }
    }
    .into()
}

fn filter_sig(sig: &Signature) -> TokenStream {
    let Signature {
        constness,
        asyncness,
        unsafety,
        abi,
        variadic,
        ..
    } = sig;

    if let Some(kw) = constness {
        return syn::Error::new(kw.span(), "Const overloading is not implemented")
            .into_compile_error();
    };
    if let Some(kw) = asyncness {
        return syn::Error::new(kw.span(), "Async overloading is not implemented")
            .into_compile_error();
    }
    if let Some(kw) = unsafety {
        return syn::Error::new(kw.span(), "Unsafe Fn* is not supported").to_compile_error();
    }
    if let Some(kw) = abi {
        return syn::Error::new(kw.span(), "Custom ABI is not supported").to_compile_error();
    }
    if let Some(kw) = variadic {
        return syn::Error::new(kw.span(), "Variadic is not supported").to_compile_error();
    }

    TokenStream::new()
}

#[test]
fn test() {}
