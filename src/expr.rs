use proc_macro2::{Ident, Literal, TokenTree};

use crate::errors::{
    CastError, CastResult, ElemNumberError, HeadTailSplitError, ListEmptyError, PairSplitError,
};

#[derive(Clone, Debug)]
pub enum Expr {
    Literal(Literal),
    Identifier(Ident),
    Operator(TokenTree),
    RustExpr(TokenTree),
    List(Vec<Expr>),
}

pub struct BorrowedList<'a>(&'a [Expr]);

impl Expr {
    pub fn as_literal(&self) -> CastResult<&Literal> {
        if let Self::Literal(l) = self {
            Ok(l)
        } else {
            Err(CastError::new("literal", self.clone()))
        }
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
}

impl<'a> BorrowedList<'a> {
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
