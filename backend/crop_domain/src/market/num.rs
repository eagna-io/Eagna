use std::{iter::Sum, ops::Mul};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Add, Sub, Neg, AddAssign, From)]
pub struct AmountToken(pub(super) i32);

impl AmountToken {
    pub const fn zero() -> AmountToken {
        AmountToken(0)
    }

    pub fn as_i32(&self) -> i32 {
        self.0
    }
}

macro_rules! impl_sum {
    ($ty: ident) => {
        impl Sum<$ty> for $ty {
            fn sum<I>(iter: I) -> $ty
            where
                I: Iterator<Item = $ty>,
            {
                return iter.fold($ty::zero(), |a, b| a + b);
            }
        }

        impl<'a> Sum<&'a $ty> for $ty {
            fn sum<I>(iter: I) -> $ty
            where
                I: Iterator<Item = &'a $ty>,
            {
                return iter.fold($ty::zero(), |a, b| a + *b);
            }
        }
    };
}

impl_sum!(AmountToken);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Add, Sub, Neg, AddAssign, From)]
pub struct AmountCoin(pub(super) i32);

impl AmountCoin {
    pub const fn zero() -> AmountCoin {
        AmountCoin(0)
    }

    pub fn as_i32(&self) -> i32 {
        self.0
    }

    /// self が target の誤差 err_percent 以内に収まっているかを判断する
    pub fn is_around(&self, target: &AmountCoin, err_percent: f64) -> bool {
        // 符号が一致するかチェック
        if self.0.signum() != target.0.signum() {
            return false;
        }

        // 絶対値が範囲に収まっているかチェック
        let self_abs = self.0.abs() as f64;
        let target_min_abs = target.0.abs() as f64 * (1_f64 - err_percent);
        let target_max_abs = target.0.abs() as f64 * (1_f64 + err_percent);
        if target_min_abs < self_abs && self_abs < target_max_abs {
            true
        } else {
            false
        }
    }
}

impl_sum!(AmountCoin);

impl<T> Mul<T> for AmountCoin
where
    i32: Mul<T, Output = i32>,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self {
        AmountCoin(self.0 * rhs)
    }
}
