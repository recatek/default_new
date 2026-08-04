extern crate proc_macro;

use proc_macro::{TokenStream, TokenTree};

/// Generates a `Default` impl for the given struct or enum that just calls `Self::new()`.
///
/// Specifically:
/// ```rust
/// #struct Foo;
///
/// impl core::default::Default for Foo {
///     #[inline]
///     fn default() -> Self {
///         Self::new()
///     }
/// }
/// ```
#[proc_macro_derive(DefaultNew)]
pub fn derive_default_new(input: TokenStream) -> TokenStream {
    let type_name = parse_type_name(input);
    generate_impl(&type_name)
}

fn parse_type_name(input: TokenStream) -> String {
    let mut tokens = input.into_iter();

    // Skip tokens until we find 'struct' or 'enum' keyword and grab the name
    while let Some(token) = tokens.next() {
        if let TokenTree::Ident(ident) = token {
            if ident.to_string() == "struct" || ident.to_string() == "enum" {
                if let Some(TokenTree::Ident(name)) = tokens.next() {
                    return name.to_string();
                }

                panic!("Expected type name after 'struct' or 'enum' keyword");
            }
        }
    }

    panic!("Expected 'struct' or 'enum' keyword in input");
}

fn generate_impl(type_name: &str) -> TokenStream {
    format!("impl core::default::Default for {type_name} {{ #[inline] fn default() -> Self {{ Self::new() }} }}")
        .parse()
        .expect("Failed to generate TokenStream")
}
