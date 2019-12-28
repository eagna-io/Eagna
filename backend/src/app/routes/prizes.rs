mod get;
mod post;
pub use get::get_list;
pub use post::post;

use crate::domain::prize::Prize;
use chrono::{DateTime, Utc};
use uuid::Uuid;

/// GET や POST の結果として返される構造体
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct ResPrize<'a> {
    id: &'a Uuid,
    name: &'a str,
    description: &'a str,
    thumbnail_url: &'a str,
    point: u32,
    available: bool,
    created: &'a DateTime<Utc>,
}

impl<'a> From<&'a Prize> for ResPrize<'a> {
    fn from(prize: &'a Prize) -> ResPrize<'a> {
        ResPrize {
            id: prize.id().as_uuid(),
            name: prize.name().as_str(),
            description: prize.description().as_str(),
            thumbnail_url: prize.thumbnail_url().as_str(),
            point: prize.point().as_u32(),
            available: *prize.available(),
            created: prize.created(),
        }
    }
}
