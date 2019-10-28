pub mod repository;
pub use repository::*;

use crate::domain::point::Point;
use crate::primitive::NonEmptyString;
use chrono::{DateTime, Utc};
use getset::Getters;
use std::num::NonZeroU32;
use uuid::Uuid;

/// # Develop Design Note
/// 各モデルのidくらいはNewTypeパターンでもいい。
/// 全てのfieldを独自モデルにするのは冗長
#[derive(Debug, Clone, Getters)]
#[get = "pub"]
pub struct Prize {
    id: PrizeId,
    name: NonEmptyString,
    description: String,
    thumbnail_url: String,
    point: Point,
    // getsetの新しいバージョンではCopyGettersが導入されるはず。
    // それが導入されれば、
    // #[get_copy = "pub"]
    // と書くことでprize.available() が参照でなく値を返すようになる。
    available: bool,
    created: DateTime<Utc>,
}

impl Prize {
    pub fn new(
        name: NonEmptyString,
        description: String,
        thumbnail_url: String,
        point: NonZeroU32,
        available: bool,
    ) -> Prize {
        Prize {
            id: PrizeId::new(),
            name,
            description,
            thumbnail_url,
            point: Point::from(point.get()),
            available,
            created: Utc::now(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, From)]
pub struct PrizeId(Uuid);

impl PrizeId {
    pub fn new() -> PrizeId {
        PrizeId(Uuid::new_v4())
    }

    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}
