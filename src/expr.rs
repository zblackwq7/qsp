use proc_macro2::{Ident, Literal, TokenTree};

use crate::{
    Seq,
    errors::{
        CastError, CastResult, ElemNumberError, HeadTailSplitError, ListEmptyError, PairSplitError,
        TryFlatMapError,
    },
};

#[derive(Clone, Debug)]
pub enum Expr {
    Literal(Literal),
    Identifier(Ident),
    Operator(TokenTree),
    RustExpr(TokenTree),
    List(Vec<Expr>),
}

impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Literal(literal) => write!(f, "{literal}"),
            Expr::Identifier(ident) => write!(f, "{ident}"),
            Expr::RustExpr(token_tree) | Expr::Operator(token_tree) => write!(f, "{token_tree}"),
            Expr::List(exprs) => {
                write!(
                    f,
                    "({})",
                    exprs
                        .iter()
                        .map(|x| x.to_string())
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            }
        }
    }
}

pub struct BorrowedList<'a>(&'a [Expr]);
pub struct StrLit(String);

impl Expr {
    pub fn as_literal(&self) -> CastResult<&Literal> {
        if let Self::Literal(l) = self {
            Ok(l)
        } else {
            Err(CastError::new("literal", self.clone()))
        }
    }

    pub fn as_str_lit(&self) -> CastResult<StrLit> {
        if let Self::Literal(l) = self {
            let lit = l.to_string();
            if lit.starts_with('"') && lit.ends_with('"') {
                return Ok(StrLit(lit));
            }
        }
        Err(CastError::new("string literal", self.clone()))
    }

    pub fn as_identifier(&self) -> CastResult<&Ident> {
        if let Self::Identifier(l) = self {
            Ok(l)
        } else {
            Err(CastError::new("identifier", self.clone()))
        }
    }

    pub fn as_operator(&self) -> CastResult<&TokenTree> {
        if let Self::Operator(l) = self {
            Ok(l)
        } else {
            Err(CastError::new("punctuation", self.clone()))
        }
    }

    pub fn as_rust_expr(&self) -> CastResult<&TokenTree> {
        if let Self::RustExpr(l) = self {
            Ok(l)
        } else {
            Err(CastError::new("rust_expr", self.clone()))
        }
    }

    pub fn as_slice(&self) -> CastResult<&[Expr]> {
        if let Self::List(l) = self {
            Ok(l)
        } else {
            Err(CastError::new("slice", self.clone()))
        }
    }

    pub fn head_tail_split(&self) -> Result<(&Expr, BorrowedList), HeadTailSplitError> {
        if let [head, tail @ ..] = self.as_slice()? {
            Ok((head, BorrowedList(tail)))
        } else {
            Err(ListEmptyError)?
        }
    }

    pub fn pair_split(&self) -> Result<(&Expr, &Expr), PairSplitError> {
        if let [fst, snd] = self.as_slice()? {
            Ok((fst, snd))
        } else {
            Err(ElemNumberError::new(self.clone()))?
        }
    }

    pub fn try_flat_map<F, T, E, R>(&self, f: F) -> Result<Seq<T>, TryFlatMapError<E>>
    where
        F: FnMut(&Expr) -> Result<R, E>,
        R: IntoIterator<Item = T>,
    {
        if let Expr::List(l) = self {
            let iters = l.iter().map(f).collect::<Result<Vec<_>, _>>()?;
            Ok(iters.into_iter().flatten().collect())
        } else {
            Err(TryFlatMapError::CastError(CastError::new(
                "literal",
                self.clone(),
            )))
        }
    }
}

impl<'a> BorrowedList<'a> {
    pub fn try_flat_map<F, T, E, R>(&self, f: F) -> Result<Seq<T>, E>
    where
        F: FnMut(&Expr) -> Result<R, E>,
        R: IntoIterator<Item = T>,
    {
        let iters = self.0.iter().map(f).collect::<Result<Vec<_>, _>>()?;
        Ok(iters.into_iter().flatten().collect())
    }

    pub fn as_slice(&self) -> &[Expr] {
        &self.0
    }

    pub fn head_tail_split(&self) -> Result<(&Expr, BorrowedList), ListEmptyError> {
        if let [head, tail @ ..] = self.0 {
            Ok((head, BorrowedList(tail)))
        } else {
            Err(ListEmptyError)
        }
    }

    pub fn pair_split(&self) -> Result<(&Expr, &Expr), ElemNumberError> {
        if let [fst, snd] = self.as_slice() {
            Ok((fst, snd))
        } else {
            Err(ElemNumberError::new(Expr::List(self.0.to_vec())))
        }
    }
}

impl StrLit {
    pub fn contained_string(&self) -> &str {
        &self.0[1..self.0.len() - 1]
    }
}
