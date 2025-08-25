use proc_macro2::{Delimiter, Group, TokenStream, TokenTree};

use crate::expr::Expr;

use std::iter::once;

pub struct ParseMatch {
    stream: TokenStream,
    value: Expr,
}

pub enum ParseResult {
    Match(ParseMatch),
    Missmatch(TokenStream),
    InputEmpty,
}

fn parse_single_elem(
    input: TokenStream,
    f: impl FnOnce(TokenTree) -> Result<Expr, TokenTree>,
) -> ParseResult {
    let mut iter = input.into_iter();
    let elem = iter.next();
    if let Some(elem) = elem {
        match f(elem) {
            Ok(expr) => ParseResult::Match(ParseMatch {
                stream: iter.collect(),
                value: expr,
            }),
            Err(elem) => ParseResult::Missmatch(once(elem).chain(iter).collect()),
        }
    } else {
        ParseResult::InputEmpty
    }
}

fn parse_literal(input: TokenStream) -> ParseResult {
    parse_single_elem(input, |elem| {
        if let TokenTree::Literal(lit) = elem {
            Ok(Expr::Literal(lit))
        } else {
            Err(elem)
        }
    })
}

fn parse_ident(input: TokenStream) -> ParseResult {
    parse_single_elem(input, |elem| {
        if let TokenTree::Ident(ident) = elem {
            Ok(Expr::Identifier(ident))
        } else {
            Err(elem)
        }
    })
}

fn parse_operator(input: TokenStream) -> ParseResult {
    if input.is_empty() {
        return ParseResult::InputEmpty;
    }

    // eat all all puncutation elements,
    // store the first non-matching one to add it back later
    let mut iter = input.into_iter();
    let mut parsed = vec![];
    let mut first_non_match = None;
    while let Some(tt) = iter.next() {
        if let TokenTree::Punct(_) = tt {
            parsed.push(tt);
        } else {
            first_non_match = Some(tt);
            break;
        }
    }

    if parsed.is_empty() {
        ParseResult::Missmatch(first_non_match.into_iter().chain(iter).collect())
    } else {
        // set a common span for all pieces of the operator
        let span = parsed[0]
            .span()
            .join(parsed.last().unwrap().span())
            .unwrap();
        let stream = parsed.into_iter().collect();
        let mut res: TokenTree = TokenTree::Group(Group::new(Delimiter::None, stream));
        res.set_span(span);

        let remaining_stream = first_non_match.into_iter().chain(iter).collect();
        ParseResult::Match(ParseMatch {
            stream: remaining_stream,
            value: Expr::Operator(res),
        })
    }
}

fn parse_rust_expr(input: TokenStream) -> ParseResult {
    parse_single_elem(input, |elem| {
        if let TokenTree::Group(g) = &elem
            && g.delimiter() == Delimiter::Brace
        {
            Ok(Expr::RustExpr(elem))
        } else {
            Err(elem)
        }
    })
}

fn parse_list(input: TokenStream) -> ParseResult {
    if input.is_empty() {
        return ParseResult::InputEmpty;
    }

    let mut iter = input.into_iter();
    let elem = iter
        .next()
        .expect("Input emtpy. Did you remove the check above?");
    if let TokenTree::Group(g) = &elem
        && g.delimiter() == Delimiter::Parenthesis
    {
        // this appears to be a list, let's parse the children
        let mut res = vec![];
        let mut child_stream = g.stream();

        let matched = loop {
            match parse_expression(child_stream) {
                ParseResult::Match(parse_match) => {
                    res.push(parse_match.value);
                    child_stream = parse_match.stream;
                }
                ParseResult::Missmatch(_) => break false,
                ParseResult::InputEmpty => break true,
            }
        };

        if matched {
            ParseResult::Match(ParseMatch {
                stream: iter.collect(),
                value: Expr::List(res),
            })
        } else {
            ParseResult::Missmatch(once(elem).chain(iter).collect())
        }
    } else {
        ParseResult::Missmatch(once(elem).chain(iter).collect())
    }
}

pub fn parse_expression(input: TokenStream) -> ParseResult {
    parse_first_matching(
        &[
            parse_literal,
            parse_ident,
            parse_operator,
            parse_rust_expr,
            parse_list,
        ],
        input,
    )
}

fn parse_first_matching(
    parsers: &[fn(TokenStream) -> ParseResult],
    mut input: TokenStream,
) -> ParseResult {
    for parser in parsers {
        match parser(input) {
            res @ (ParseResult::Match(_) | ParseResult::InputEmpty) => {
                return res;
            }
            ParseResult::Missmatch(token_stream) => {
                input = token_stream;
            }
        }
    }

    ParseResult::Missmatch(input)
}

pub fn parse(stream: impl Into<TokenStream>) -> ParseResult {
    parse_expression(stream.into())
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    fn parse(src: &str) -> Expr {
        let Ok(ts) = TokenStream::from_str(src) else {
            panic!("Couldnt parse `{src}` to tokenstream");
        };

        match super::parse(ts) {
            ParseResult::Match(parse_match) => {
                if parse_match.stream.into_iter().next().is_some() {
                    panic!("Could parse `src` as Expr, but result stream isn't empty");
                }
                parse_match.value
            }
            ParseResult::Missmatch(_) => {
                panic!("Couldn't parse `{src}` as Expr")
            }
            ParseResult::InputEmpty => {
                panic!("Input empty")
            }
        }
    }

    #[test]
    fn test_lit_parser() {
        let res = parse("\"lit\"");
        assert_eq!(res.as_literal().unwrap().to_string(), "\"lit\"");

        let res = parse("5");
        assert_eq!(res.as_literal().unwrap().to_string(), "5");
    }

    #[test]
    fn test_ident_parser() {
        let res = parse("ident");
        assert_eq!(res.as_identifier().unwrap().to_string(), "ident");
    }

    #[test]
    fn test_punct_parser() {
        let res = parse("->");
        assert_eq!(res.as_operator().unwrap().to_string(), "->");
    }

    #[test]
    fn test_rust_expr_parser() {
        let res = parse("{ 7 + 8 }");
        assert_eq!(res.as_rust_expr().unwrap().to_string(), "{ 7 + 8 }");
    }

    #[test]
    fn test_expr_parser() {
        let res = parse(r#"(get ("foo" (bar -> 5)))"#);
        let (get, inner) = res.pair_split().unwrap();
        assert_eq!(get.as_identifier().unwrap().to_string(), "get");
        let (foo, inner_) = inner.pair_split().unwrap();
        assert_eq!(foo.as_literal().unwrap().to_string(), r#""foo""#);
        let [bar, to, five] = inner_.as_slice().unwrap() else {
            panic!("inner_ has unexpected format: {inner_:#?}");
        };
        assert_eq!(bar.as_identifier().unwrap().to_string(), "bar");
        assert_eq!(to.as_operator().unwrap().to_string(), "->");
        assert_eq!(five.as_literal().unwrap().to_string(), "5");
    }
}
