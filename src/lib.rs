//! # Piperize
//!
//! inspired by elixir's pipe functions this gets rid of the boilerplate of writing a new trait if you want to create a dot method on some type
//!
//! ## Before:
//! ```rust
//! pub trait FooThis {
//!     fn foo_this(self) -> i32;
//! }
//!
//! impl FooThis for i32 {
//!     fn foo_this(self) -> i32 {
//!         self + self
//!     }
//! }
//!
//! fn main() {
//!     let foo = 21.foo_this();
//!     assert_eq!(foo, 42);
//! }
//! ```
//!
//! ## After:
//! ```rust
//! #[piperize::piperize]
//! fn foo_this(input: i32) -> i32 {
//!     input + input
//! }
//!
//! fn main() {
//!     let foo = 21.foo_this();
//!     assert_eq!(foo, 42);
//! }
//! ```
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{FnArg, ItemFn, PatType, parse_macro_input, punctuated::Punctuated, token::Comma};

fn get_arg(inputs: &Punctuated<FnArg, Comma>) -> &PatType {
    assert!(
        !inputs.is_empty(),
        "Traitify function cannot take no arguments"
    );
    assert!(
        inputs.len() == 1,
        "Traitify function can only take one argument"
    );
    match inputs.first() {
        Some(FnArg::Typed(pat_type)) => pat_type,
        _ => panic!(
            "Invalid function arguments\n example of a valid function signature: fn foo(a: i32) -> i32"
        ),
    }
}

#[proc_macro_attribute]
pub fn piperize(_: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);

    let fn_name = &input_fn.sig.ident;
    let output = &input_fn.sig.output;
    let body = &input_fn.block;

    let camel_case_fn_name = camel_case(fn_name);

    let arg = get_arg(&input_fn.sig.inputs);
    let arg_name = &arg.pat;
    let arg_type = &arg.ty;

    let generics = &input_fn.sig.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let expanded = quote! {
        pub trait #camel_case_fn_name #generics #where_clause {
            fn #fn_name(self) #output;
        }

        impl #impl_generics #camel_case_fn_name #ty_generics for #arg_type #where_clause {
            fn #fn_name(self) #output {
                let #arg_name = self;
                #body
            }
        }
    };

    expanded.into()
}

fn camel_case(name: &syn::Ident) -> syn::Ident {
    let mut name_str = name.to_string();
    to_camel_case(&mut name_str);
    syn::Ident::new(&name_str, Span::mixed_site())
}

fn to_camel_case(s: &mut String) {
    assert!(s.is_ascii(), "identifier must be valid ascii");

    // SAFETY: we asserted that the string is ascii
    let bytes = unsafe { s.as_bytes_mut() };
    let mut write_index = 0;
    let mut capitalize_next = true;

    for read_index in 0..bytes.len() {
        let b = bytes[read_index];
        match b {
            b'a'..=b'z' => {
                if capitalize_next {
                    bytes[write_index] = b - b'a' + b'A';
                } else {
                    bytes[write_index] = b;
                }
                write_index += 1;
                capitalize_next = false;
            }
            b'A'..=b'Z' | b'0'..=b'9' => {
                bytes[write_index] = b;
                write_index += 1;
                capitalize_next = false;
            }
            _ => {
                capitalize_next = true;
            }
        }
    }

    s.truncate(write_index);
}
