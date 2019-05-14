#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct AmountToken(pub i32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct AmountCoin(pub i32);

impl AmountCoin {
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

macro_rules! impl_ops {
    ($target: ty, $constructor: expr) => {
        impl std::ops::Add for $target {
            type Output = $target;

            fn add(self, rhs: Self) -> Self::Output {
                $constructor(self.0 + rhs.0)
            }
        }

        impl<'a> std::ops::Add for &'a $target {
            type Output = $target;

            fn add(self, rhs: Self) -> Self::Output {
                $constructor(self.0 + rhs.0)
            }
        }

        impl<'a> std::ops::Add<$target> for &'a $target {
            type Output = $target;

            fn add(self, rhs: $target) -> Self::Output {
                $constructor(self.0 + rhs.0)
            }
        }

        impl std::ops::AddAssign for $target {
            fn add_assign(&mut self, rhs: Self) {
                self.0 += rhs.0
            }
        }

        impl std::iter::Sum for $target {
            fn sum<I>(iter: I) -> Self
            where
                I: Iterator<Item = Self>,
            {
                $constructor(iter.map(|a| a.0).sum())
            }
        }
    };
}

impl_ops!(AmountToken, AmountToken);
impl_ops!(AmountCoin, AmountCoin);
