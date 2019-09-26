//! # Develop Design Note
//! ドメインモデルはインフラ層やアプリケーション層に関する知識を持たない。
//! つまり、インフラ層のモデルからの変換や、
//! アプリケーション層のモデルへの変換はここでは行わない。
//! インフラ層のモデルからの変換はrepositoryで、
//! アプリケーション層のモデルへの変換はアプリケーション層で行う。
pub mod repository;
pub use repository::*;

use crate::primitive::NonEmptyString;
use chrono::{DateTime, Utc};
use getset::Getters;
use uuid::Uuid;

#[derive(Debug, Clone, Getters)]
pub struct Prize {
    id: Uuid,
    name: NonEmptyString,
    description: String,
    thumbnail_url: String,
    price: u32,
    available: bool,
    created: DateTime<Utc>,
}

impl Prize {
    pub fn new(
        name: NonEmptyString,
        description: String,
        thumbnail_url: String,
        price: u32,
        available: bool,
    ) -> Prize {
        Prize {
            id: Uuid::new_v4(),
            name,
            description,
            thumbnail_url,
            price,
            available,
            created: Utc::now(),
        }
    }
}
