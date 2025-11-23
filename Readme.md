# Quick S-Expressions Parser (QSP)

This crate implements as small parser for token streams holding S-Expressions.
The S-Expression syntax is very minimal and not conforming to any standard.
The only dependency is [proc-macro2](https://docs.rs/proc-macro2/latest/proc_macro2/),
so it should compile very fast.

## Why ?

The goal of this library is to make it as convenient as possible to write
a whole range of proc macros. Writing a proc macros usually takes a
lot of effort.
Most of that effort goes into parsing the input, typically using
[syn](https://docs.rs/syn/latest/syn/), which is incredibly powerful, but
very tedious, and slow to compile. If you don't need to bother yourself with
parsing, and have an easy time accessing the results of said parsing, most
of the trouble of writing proc-macros is gone already.

Additionally, QSP is very simple and only has a single dependency, which
should make it compile pretty fast.

Sadly, you will still have to create a macro-crate for your proc-macro.
To reach true lisp-macro-convenience, we still require
[in-package proc-macros](https://github.com/rust-lang/rfcs/pull/3826)

## Examples

You can run this by running `cargo run --example pipe`. It implements
a simple pipe macro. If you were to put this into a macro-crate and name
it `pipe`, an invocation in actual code would look just like the `input_string` except
with `pipe!` before the opening parenthesis. And you would need to turn the result
string into a `TokenStream` again, of course.

```rust
use anyhow::Result;
use proc_macro2::TokenStream;
use qsp::Expr;

fn main() -> Result<()> {
    let input_string = r#"
    ( input
        (.strip())
        count_vowels
        { |x| {println!("There are {x} vowels in {input}"); x}}
        ( * 2)
        { |x| println!("you get {x} points for that") }
    )
    "#;

    let token_stream: TokenStream = input_string.parse().unwrap();
    let ast = qsp::parse(token_stream).unwrap();
    let (input, steps) = ast.head_tail_split()?;
    let mut call = input.to_string();
    for step in steps {
        match step {
            Expr::Literal(_) => {
                panic!("steps cannot be literals");
            }
            Expr::Identifier(ident) => {
                call = format!("{ident}({call})");
            }
            Expr::Operator(_) => {
                panic!("steps cannot be operators");
            }
            Expr::RustExpr(token_tree) => {
                call = format!("({token_tree})({call})");
            }
            elems @ Expr::List(_) => call = format!("({call}) {elems}"),
        }
    }

    println!("Resulting call:\n{call}");
    Ok(())
}
  
```

The following functions are defined on the `Expr` type, which make it
very easy to use. The error messages contain as much information as possible,
and should make it easy to catch mistakes, even if you just liberally use `?`
everywhere.

- `as_literal(&self) -> CastResult<&Literal> `
- `as_str_lit(&self) -> CastResult<StrLit> `
- `as_identifier(&self) -> CastResult<&Ident> `
- `as_operator(&self) -> CastResult<&TokenTree> `
- `as_rust_expr(&self) -> CastResult<&TokenTree> `
- `as_slice(&self) -> CastResult<&[Expr]> `
- `head_tail_split(&self) -> Result<(&Expr, BorrowedList<'_>), HeadTailSplitError> `
- `pair_split(&self) -> Result<(&Expr, &Expr), PairSplitError> `
- `try_flat_map<F, T, E, R>(&self, f: F) -> Result<Vec<T>, TryFlatMapError<E>`

`BorrowedList` reimplements all list-related functions.

## State

This is still a proof of concept. I intend to use it the next time I need
a proc-macro, but that hasn't happened yet. It currently serves as an example
of an idea.
