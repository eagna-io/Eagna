use crate::domain::lmsr;
use crate::domain::market::{
    models::{
        ClosedMarket, Market, MarketAttrs, MarketId, MarketStatus, MarketToken, OpenMarket,
        UpcomingMarket,
    },
    num::{AmountCoin, AmountToken},
    order::{MarketOrders, Order},
};
use crate::domain::user::models::{User as _, UserCoinUpdated, UserWithAttrs};
use crate::primitive::{NonEmptyString, NonEmptyVec};
use anyhow::anyhow;
use chrono::{DateTime, Utc};
use thiserror::Error;

pub const MAX_SLIP_RATE: f64 = 0.05;

pub struct MarketManager {}

/*
 * =======================
 * MarketManager::create
 * =======================
 */
impl MarketManager {
    pub fn create(
        title: NonEmptyString,
        desc: String,
        lmsr_b: lmsr::B,
        open: DateTime<Utc>,
        close: DateTime<Utc>,
        tokens: NonEmptyVec<MarketToken>,
    ) -> NewMarket {
        let id = MarketId::new();
        let attrs = MarketAttrs::from((title, desc, lmsr_b, open, close, tokens));
        NewMarket { id, attrs }
    }
}

pub struct NewMarket {
    id: MarketId,
    attrs: MarketAttrs,
}

impl Market for NewMarket {
    fn id(&self) -> MarketId {
        self.id
    }
    fn attrs(&self) -> &MarketAttrs {
        &self.attrs
    }
    fn status(&self) -> MarketStatus {
        MarketStatus::Upcoming
    }
    fn orders(&self) -> &MarketOrders {
        static EMPTY_ORDERS: MarketOrders = MarketOrders::new();
        &EMPTY_ORDERS
    }
}

impl UpcomingMarket for NewMarket {}

/*
 * ====================
 * MarketManager::open
 * ====================
 */
impl MarketManager {
    pub fn open<M>(market: M) -> anyhow::Result<NewOpenMarket<M>>
    where
        M: UpcomingMarket,
    {
        if !market.is_openable() {
            return Err(anyhow!("Market {:?} is not openable", market.id()));
        }

        Ok(NewOpenMarket { inner: market })
    }
}

pub struct NewOpenMarket<M> {
    inner: M,
}

impl<M: Market> Market for NewOpenMarket<M> {
    fn id(&self) -> MarketId {
        self.inner.id()
    }
    fn attrs(&self) -> &MarketAttrs {
        self.inner.attrs()
    }
    fn status(&self) -> MarketStatus {
        MarketStatus::Open
    }
    fn orders(&self) -> &MarketOrders {
        static EMPTY_ORDERS: MarketOrders = MarketOrders::new();
        &EMPTY_ORDERS
    }
}

impl<M: Market> OpenMarket for NewOpenMarket<M> {}

/*
 * ==================
 * MarketManager::buy_token/sell_token
 * ==================
 */
impl MarketManager {
    pub fn buy_token<M, U>(
        market: M,
        user: U,
        token_name: &NonEmptyString,
        amount_token: &AmountToken,
        price_expected: &AmountCoin,
    ) -> Result<(OpenMarketOrderAdded<M>, UserCoinUpdated<U>), AddOrderError<U, M>>
    where
        M: OpenMarket,
        U: UserWithAttrs,
    {
        macro_rules! err {
            ($err_type:expr) => {
                Err(AddOrderError::from((market, user, $err_type)))
            };
        }

        if !market.is_valid_token(token_name) {
            return err!(AddOrderErrorType::InvalidToken);
        }
        if *amount_token <= AmountToken::zero() {
            return err!(AddOrderErrorType::InvalidAmountToken);
        }

        // 購入に必要なコイン量の取得
        let price = market.compute_price_of_buy(token_name, *amount_token);
        if user.coin() < price {
            return err!(AddOrderErrorType::InsufficientCoinBalance);
        }
        if !price.is_around(price_expected, MAX_SLIP_RATE) {
            return err!(AddOrderErrorType::PriceSlip {
                price,
                expected: *price_expected
            });
        }

        // Userのコイン量を更新
        let new_coin = user.coin() - price;
        let updated_user = user.update_coin(new_coin);

        // Marketの更新
        let new_order = Order::new(
            *updated_user.id(),
            token_name.clone(),
            *amount_token,
            -price,
        );
        let new_orders = market.orders().clone().add(new_order);
        let updated_market = OpenMarketOrderAdded {
            inner: market,
            orders: new_orders,
        };

        Ok((updated_market, updated_user))
    }

    pub fn sell_token<M, U>(
        market: M,
        user: U,
        token_name: &NonEmptyString,
        amount_token: &AmountToken, // > 0
        gain_expected: &AmountCoin,
    ) -> Result<(OpenMarketOrderAdded<M>, UserCoinUpdated<U>), AddOrderError<U, M>>
    where
        M: OpenMarket,
        U: UserWithAttrs,
    {
        macro_rules! err {
            ($err_type:expr) => {
                Err(AddOrderError::from((market, user, $err_type)))
            };
        }

        if !market.is_valid_token(token_name) {
            return err!(AddOrderErrorType::InvalidToken);
        }
        if *amount_token <= AmountToken::zero() {
            return err!(AddOrderErrorType::InvalidAmountToken);
        }

        // トークンの残高をチェックする
        let user_token = market.compute_balance_of_user_token(user.id(), token_name);
        if user_token < *amount_token {
            return err!(AddOrderErrorType::InsufficientTokenBalance);
        }

        // 売却によって得られるコイン量
        let gain = market.compute_gain_of_sell(token_name, *amount_token);
        if !gain.is_around(gain_expected, MAX_SLIP_RATE) {
            return err!(AddOrderErrorType::PriceSlip {
                price: gain,
                expected: *gain_expected
            });
        }

        let new_user_coin = user.coin() + gain;
        let updated_user = user.update_coin(new_user_coin);

        let new_order = Order::new(*updated_user.id(), token_name.clone(), -*amount_token, gain);
        let new_orders = market.orders().clone().add(new_order);
        let updated_market = OpenMarketOrderAdded {
            inner: market,
            orders: new_orders,
        };

        Ok((updated_market, updated_user))
    }
}

pub struct OpenMarketOrderAdded<M> {
    inner: M,
    orders: MarketOrders,
}

impl<M> OpenMarketOrderAdded<M> {
    pub fn added_order(&self) -> &Order {
        self.orders.last_order().unwrap()
    }
}

impl<M: Market> OpenMarket for NewClosedMarket<M> {}

impl<M: Market> Market for OpenMarketOrderAdded<M> {
    fn id(&self) -> MarketId {
        self.inner.id()
    }
    fn attrs(&self) -> &MarketAttrs {
        self.inner.attrs()
    }
    fn status(&self) -> MarketStatus {
        MarketStatus::Open
    }
    fn orders(&self) -> &MarketOrders {
        &self.orders
    }
}

#[derive(From)]
pub struct AddOrderError<U, M> {
    pub market: M,
    pub user: U,
    pub source: AddOrderErrorType,
}

#[derive(Debug, Error)]
pub enum AddOrderErrorType {
    #[error("Market does not have a such token")]
    InvalidToken,
    #[error("Invalid amount of token")]
    InvalidAmountToken,
    #[error("User does not have enough token")]
    InsufficientTokenBalance,
    #[error("User does not have enough coin")]
    InsufficientCoinBalance,
    #[error("Token price is {price:?} but expected price is {expected:?}")]
    PriceSlip {
        price: AmountCoin,
        expected: AmountCoin,
    },
}

/*
 * =====================
 * MarketManager::close
 * =====================
 */
impl MarketManager {
    pub fn close<M>(market: M) -> anyhow::Result<NewClosedMarket<M>>
    where
        M: OpenMarket,
    {
        if !market.is_closable() {
            return Err(anyhow!("Market {:?} is not closable", market.id()));
        }
        Ok(NewClosedMarket { inner: market })
    }
}

pub struct NewClosedMarket<M> {
    inner: M,
}

impl<M: Market> Market for NewClosedMarket<M> {
    fn id(&self) -> MarketId {
        self.inner.id()
    }
    fn attrs(&self) -> &MarketAttrs {
        self.inner.attrs()
    }
    fn status(&self) -> MarketStatus {
        MarketStatus::Closed
    }
    fn orders(&self) -> &MarketOrders {
        self.inner.orders()
    }
}

impl<M: Market> ClosedMarket for NewClosedMarket<M> {}
