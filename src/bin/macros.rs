// macros.rs, Examples from the book "Rust Programming Language"
// Copyright (C) 2024, Savio Sena <savio.sena@gmail.com>

#![allow(unused_mut)]

//! # Macros

/// ## The Difference Between Macros and Functions
///
/// The term _macro_ refers to a family of features in Rust: _declarative_ macros
/// with `macro_rules!` and three kinds of _procedural_ macros:
///
///   - Custom `#[derive]` macros that specify code added with the `derive`
///     attribute used on structs and enums.
///   - Attribute-like macros that define custom attributes usable on any item
///   - Function-like macros that look like functions calls but operate on the
///     tokens specified as their argument.
///
/// Fundamentally, macros are a way of writing code that writes other code,
/// which is known as _metaprogramming_. Macros _expand_ to produce more code
/// than the code you’ve written manually.
///
/// Metaprogramming is useful for reducing the amount of code you have to write
/// and maintain, which is also one of the roles of functions. However, macros
/// have some additional powers that functions don’t.
///
/// A function signature _must declare the number and type of parameters_ the
/// function has. Macros, on the other hand, can take a **variable number of
/// parameters**.
///
/// The downside of writing macros is that you write code that writes code and
/// due to this indirection, macro definitions are generally more difficult to
/// _read_, _understand_, and _maintain_ than function definitions.
///
/// Another important difference between macros and functions is that you must
/// _define_ macros or _bring them into scope_ **before you call them** in a
/// file, as opposed to functions you can define anywhere and call anywhere.
///
/// ## Declarative Macros with `macro_rules!` for General Metaprogramming
///
/// The most widely used form of macros in Rust is the _declarative macro_.
/// These are also sometimes referred to as "macros by example", "`macro_rules!`
/// macros", or just plain "macros".
///
/// At their core, declarative macros allow you to write something similar to
/// Rust `match` expressions. These expressions are control structures that take
/// an expression, compare the resulting value to patterns, and then run the
/// code associated with the matching pattern. Macros also compare a value to
/// patterns that are associated with particular code: in this situation, the
/// value is the literal Rust source code passed to the macro; the patterns are
/// compared with the structure of that source code; and the code associated
/// with each pattern, when matched, replaces the code passed to the macro. This
/// all happens during compilation.
/// ```
/// #[macro_export]
/// macro_rules! my_vec {
///     ( $( $x:expr ),* ) => {
///         {
///             let mut temp_vec = Vec::new();
///             $(
///                 temp_vec.push($x);
///             )*
///             temp_vec
///         }
///     };
/// }
/// ```
/// The `#[macro_export]` annotation indicates that this macro should be made
/// available whenever the crate in which the macro is defined is brought into
/// scope.
///
/// We then start the macro definition with `macro_rules!` and the name of the
/// macro we’re defining _without_ the exclamation mark.
///
/// The structure in the `my_vec!` body is similar to the structure of a `match`
/// expression. We have one arm with the pattern `( $( $x:expr ),* )`, followed
/// by `=>` and the block of code associated with this pattern. If the pattern
/// matches, the associated block of code will be emitted. Given that this is
/// the only pattern in this macro, there is only one valid way to match; any
/// other pattern will result in an error. More complex macros will have more
/// than one arm.
///
/// Valid pattern syntax in macro definitions is different than the pattern
/// syntax we've covered before, because macro patterns are matched against Rust
/// code rather than values. Declarative expressions can match against
/// expressions (`expr`), types (`ty`), and even entire items (`item`). To see
/// the full macro pattern syntax check the link below:
///
/// - [The Rust Reference: Macros By Example](https://doc.rust-lang.org/reference/macros-by-example.html)
///
/// Let's go back to analyzing the pattern: `( $( $x:expr ),* )`. First, we use
/// a set of parentheses to encompass the whole pattern.
///
/// We use a _dollar sign_ (`$`) to declare a variable in the macro system that
/// will contain the Rust code matching the pattern. The dollar sign makes it
/// clear this is a _macro variable_ as opposed to a regular Rust variable.
///
/// Next comes a set of parentheses that _captures_ values that match the
/// pattern within the parentheses for use in the replacement code. Within `$()`
/// is `$x:expr`, which matches any Rust expression and gives the expression the
/// name `$x`.
///
/// The comma following `$()` indicates that a literal comma separator character
/// could optionally appear after the code that matches the code in `$()`. The
/// `*` specifies that the pattern matches _zero or more_ of whatever precedes
/// the `*`.
///
/// When we call this macro with `my_vec![1, 2, 3];`, the `$x` pattern matches
/// _three times_ with the three expressions `1`, `2`, and `3`.
///
/// Now let’s look at the pattern in the body of the code associated with this
/// arm: `temp_vec.push()` within `$()*` is generated for each part that matches
/// `$()` in the pattern zero or more times depending on how many times the
/// pattern matches.
///
/// The `$x` is replaced with each expression matched. When we call this macro
/// with `vec![1, 2, 3];`, the code generated that replaces this macro call will
/// be the following:
/// ```
/// {
///     let mut temp_vec = Vec::new();
///     temp_vec.push(1);
///     temp_vec.push(2);
///     temp_vec.push(3);
///     temp_vec
/// }
/// ```
///
/// To learn more about how to write macros, consult the online documentation or
/// other resources, such as [The Little Book of Rust Macros](https://veykril.github.io/tlborm/),
/// started by Daniel Keep and continued by Lukas Wirth..

fn declarative_macros() {
    // To define a macro, you use the `macro_rules!` construct.

    #[macro_export]
    macro_rules! my_vec {
        ( $( $x:expr ),* ) => {
            {
                let mut temp_vec = Vec::new();
                $(
                    temp_vec.push($x);
                )*
                    temp_vec
            }
        };
    }

    let v: Vec<u32> = my_vec![1, 2, 3];
    println!("{:?}", v);

    #[macro_export]
    macro_rules! manylet {
        ( $( $i:ident ),* = $e:expr ) => {
            $(
                let mut $i = &$e;
            )*
        }
    }

    let s = String::from("foo");
    manylet!(x, y = s);
    println!("{x} == {y}");
}


use macros::HelloMacro;
use macros_derive::HelloMacro;

/// ## Procedural Macros for Generating Code from Attributes
///
/// The second form of macros is the _procedural macro_, which acts more like a
/// function (and is a type of procedure). It accepts some code as input,
/// operate on that code, and produce some code as an output rather than
/// matching against patterns and replacing the code with other code as
/// declarative macros do.
///
/// The three kinds of procedural macros are:
///
///  - Custom `derive`
///  - Attribute-like
///  - Function-like
///
/// _When creating procedural macros, the definitions must reside in their own
/// crate with a special crate type. This is for complex technical reasons that
/// we hope to eliminate in the future._
///
/// ```
/// use proc_macro;
///
/// #[some_attribute]
/// pub fn some_name(input: TokenStream) -> TokenStream {
/// }
/// ```
///
/// `some_attribute` is a placeholder for using a specific macro variety.
///
/// The function that defines a procedural macro takes a `TokenStream` as an
/// input and produces a `TokenStream` as an output.
///
/// The `TokenStream` type is defined by the `proc_macro` crate that is included
/// with Rust and represents a sequence of tokens.
///
/// This is the core of the macro: the source code that the macro is operating
/// on makes up the input `TokenStream`, and the code the macro produces is the
/// output `TokenStream`. The function also has an attribute attached to it that
/// specifies which kind of procedural macro we’re creating.
///
/// We can have multiple kinds of procedural macros in the same crate. Let’s
/// look at the different kinds of procedural macros. We’ll start with a custom
/// derive macro and then explain the small dissimilarities that make the other
/// forms different.
///
/// ### How to Write a Custom `derive` Macro
///
/// At the time of this writing, procedural macros need to be in their own
/// crate. Eventually, this restriction might be lifted. The convention for
/// structuring crates and macro crates is as follows: for a crate named `foo`,
/// a custom derive procedural macro crate is called `foo_derive`.

fn procedural_macros() {
    #[derive(HelloMacro)]
    struct Pancakes;

    Pancakes::hello();
}

use macros_derive::route;

/// ## Attribute-like Macros
///
/// Attribute-like macros are similar to custom derive macros, but instead of
/// generating code for the `derive` attribute, they allow you to create new
/// attributes. They’re also more flexible: `derive` only works for structs and
/// enums; attributes can be applied to other items as well, such as functions.
fn attribute_macros() {
    #[route(GET, "/")]
    fn index() {
        println!("index()");
    }
    index();
}

use macros_derive::eval;

/// ## Function-like Macros
///
/// Function-like macros define macros that look like function calls. Similarly
/// to `macro_rules!` macros, they’re more flexible than functions; for example,
/// they can take an unknown number of arguments. However, `macro_rules!` macros
/// can be defined only using the match-like syntax we discussed earlier.
///
/// Function-like macros take a `TokenStream` parameter and their definition
/// manipulates that `TokenStream` using Rust code as the other two types of
/// procedural macros do.
///
/// An example of a function-like macro is an `sql!` macro that might be called
/// like so:
/// ```no_run
/// let sql = sql!(SELECT * FROM posts WHERE id=1);
/// ```
/// This macro would parse the SQL statement inside it and check that it’s
/// syntactically correct, which is much more complex processing than a
/// `macro_rules!` macro can do. The `sql!` macro would be defined like this:
/// ```no_run
/// #[proc_macro]
/// pub fn sql(input: TokenStream) -> TokenStream {}
/// ```
/// This definition is similar to the custom derive macro’s signature: we
/// receive the tokens that are inside the parentheses and return the code we
/// wanted to generate.
fn function_macros() {
    eval!(println!("function_macro!"));
}

fn main() {
    println!("-=- declarative_macros() -=-");
    declarative_macros();

    println!("-=- procedural_macros() -=-");
    procedural_macros();

    println!("-=- attribute_macros() -=-");
    attribute_macros();

    println!("-=- function_macros() -=-");
    function_macros();
}
