use core::fmt;
use std::error::Error;

use crate::expr::Expr;

macro_rules! impl_error{
    ($ety:ty, $self:ident, $($fmt:tt)*) => {
        impl fmt::Display for $ety {
            fn fmt(&$self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, $($fmt)*)
            }
        }

        impl Error for $ety {}
    }
}

macro_rules! err_enum{
    ($name:ident, $($variants:ident),+) => {
        #[derive(Debug)]
        pub enum $name {
            $(
                $variants($variants)
            ),+
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    $(
                        Self::$variants(x) => write!(f, "{x}")
                    ),+
                }
            }
        }

        impl Error for $name {}

        $(
          impl From<$variants> for $name {
              fn from(x: $variants) -> Self {
                  Self::$variants(x)
              }
          }
        )+

    }
}

// =======================================================================================
#[derive(Debug)]
pub struct CastError {
    desired: &'static str,
    actual: Expr,
}

impl CastError {
    pub fn new(desired: &'static str, actual: Expr) -> Self {
        Self { desired, actual }
    }
}

pub type CastResult<T> = std::result::Result<T, CastError>;
impl_error! { CastError, self, "tried to get {:?} as {}", self.actual, self.desired }

// =======================================================================================

#[derive(Debug)]
pub struct ListEmptyError;
impl_error! { ListEmptyError, self, "List is empty"}

err_enum! { HeadTailSplitError, CastError, ListEmptyError }

// =======================================================================================

#[derive(Debug)]
pub struct ElemNumberError {
    actual: Expr,
}

impl ElemNumberError {
    pub fn new(actual: Expr) -> Self {
        Self { actual }
    }
}

impl_error! { ElemNumberError, self, "wrong number of elements for this operation. {:?}", self.actual }
err_enum! { PairSplitError, CastError, ElemNumberError }
