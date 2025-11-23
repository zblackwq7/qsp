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
