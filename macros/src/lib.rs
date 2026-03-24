extern crate proc_macro;
use proc_macro::{TokenStream, TokenTree};

/// Generates a `Default` impl for the given struct that just calls `Self::new()`.
/// 
/// Specifically:
/// ```
/// impl Default for Foo {
///     #[inline]
///     fn default() -> Self {
///         Self::new()
///     }
/// }
/// ```
#[proc_macro_derive(DefaultNew)]
pub fn derive_default_new(input: TokenStream) -> TokenStream {
    let struct_name = parse_struct_name(input);
    generate_impl(&struct_name)
}

fn parse_struct_name(input: TokenStream) -> String {
    let mut tokens = input.into_iter();

    // Skip tokens until we find 'struct' keyword and grab the name
    while let Some(token) = tokens.next() {
        if let TokenTree::Ident(ident) = token {
            if ident.to_string() == "struct" {
                if let Some(TokenTree::Ident(name)) = tokens.next() {
                    return name.to_string();
                }

                panic!("Expected struct name after 'struct' keyword");
            }
        }
    }

    panic!("Expected 'struct' keyword in input");
}

fn generate_impl(struct_name: &str) -> TokenStream {
    format!(
        "impl Default for {} {{ #[inline] fn default() -> Self {{ Self::new() }} }}",
        struct_name
    )
    .parse()
    .expect("Failed to generate TokenStream")
}
