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
//!
//! ## Using multiple arguments:
//!
//! ```rust
//! #[piperize::piperize]
//! fn my_add(a: i32, b: i32) -> i32 {
//!     a + b
//! }
//!
//! fn main() {
//!     let foo = 21.my_add(21);
//!     assert_eq!(foo, 42);
//! }
//! ```
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{FnArg, ItemFn, PatType, parse_macro_input, punctuated::Punctuated, token::Comma};

fn args_to_split(inputs: &Punctuated<FnArg, Comma>) -> (&PatType, Vec<&FnArg>) {
    assert!(
        !inputs.is_empty(),
        "Piperize function cannot take no arguments"
    );
    assert!(
        !inputs.iter().any(|i| match i {
            FnArg::Receiver(_) => true,
            FnArg::Typed(_) => false,
        }),
        "Function arguments cannot be \"self\""
    );
    let Some(FnArg::Typed(first_arg)) = inputs.first() else {
        panic!(
            "Invalid function arguments\n example of a valid function signature: fn foo(a: i32) -> i32"
        )
    };
    let rest_args = inputs.iter().skip(1).collect();
    (first_arg, rest_args)
}

#[proc_macro_attribute]
pub fn piperize(_: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);

    let fn_name = &input_fn.sig.ident;
    let output = &input_fn.sig.output;
    let body = &input_fn.block;
    let visibility = &input_fn.vis;

    let camel_case_fn_name = camel_case(fn_name);

    let arg_split = args_to_split(&input_fn.sig.inputs);
    let first_arg = arg_split.0;
    let first_arg_name = &first_arg.pat;
    let first_arg_type = &first_arg.ty;
    let rest_args = arg_split.1;
    let mut rest = Punctuated::<&FnArg, Comma>::new();
    for rest_arg in rest_args {
        rest.push(rest_arg);
    }

    let generics = &input_fn.sig.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let expanded = quote! {
        #input_fn

        #visibility trait #camel_case_fn_name #generics #where_clause {
            fn #fn_name(self, #rest) #output;
        }

        impl #impl_generics #camel_case_fn_name #ty_generics for #first_arg_type #where_clause {
            fn #fn_name(self, #rest) #output {
                let #first_arg_name = self;
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
