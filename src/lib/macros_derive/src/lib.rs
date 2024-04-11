// lib.rs, Examples from the book "Rust Programming Language"
// Copyright (C) 2022-2024, Savio Sena <savio.sena@gmail.com>

use proc_macro::TokenStream;
use quote::quote;
use syn;

/// The `macros_derive` function will be called when a user of our library
/// specifies `#[derive(HelloMacro)]` on a type.
///
/// The code in this function will be the same for almost every procedural macro
/// crate you see or create. The code you specify in the body of the inner
/// function (`impl_macros`) will be different depending on your procedural
/// macro’s purpose.
///
/// The `parse` function in `syn` takes a `TokenStream` and returns a
/// `DeriveInput` struct representing the parsed Rust code.
///
/// The lines below shows the relevant parts of the `DeriveInput` struct we get
/// from parsing the `struct Pancakes;` string:
/// ```no_run
/// DeriveInput {
///     // --snip--
///     ident: Ident {
///         ident: "Pancakes",
///         span: #0 bytes(95..103)
///     },
///     data: Struct(
///         DataStruct {
///             struct_token: Struct,
///             fields: Unit,
///             semi_token: Some(
///                 Semi
///             )
///         }
///     )
/// }
/// ```
/// For more information check the documentation for
/// [`DeriveInput`](https://docs.rs/syn/1.0.109/syn/struct.DeriveInput.html).
///
/// Note that the output for our derive macro is also a `TokenStream`. The
/// returned `TokenStream` is added to the code that our crate users write, so
/// when they compile their crate, they'll get the extra functionality that we
/// provide in the modified `TokenStream`.

#[proc_macro_derive(HelloMacro)]
pub fn macros_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree that we can
    // manipulate. We call `unwrap` because we must return a `TokenStream`
    // rather than `Result`. In production code you should provide more specific
    // error messages about what went wrong by using `panic!` or `expect`.
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_macros(&ast)
}

/// The `quote!` macro lets us define the Rust code that we want to return. It
/// provides some very cool templating mechanics: we can enter `#name`, and
/// `quote!` will replace it with the value in the variable `name`.
///
/// For more information check out [the `quote` crate's docs](https://docs.rs/quote).

fn impl_macros(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl HelloMacro for #name {
            fn hello() {
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };
    gen.into() // convert it to `TokenStream`.
}

/// Example for the `attribute_macros` in `macros.rs`.
///
/// Here, we have two parameters of type `TokenStream`. The first is for the
/// contents of the attribute: the `GET, "/"` part. The second is the body of the
/// item the attribute is attached to: in this case, `fn index() {}` and the rest
/// of the function’s body.
#[proc_macro_attribute]
pub fn route(_: TokenStream, item: TokenStream) -> TokenStream {
    item // for the sake of simplicity just return the body of the item attached
         // to the item.
}

#[proc_macro]
pub fn eval(input: TokenStream) -> TokenStream {
    input
}
