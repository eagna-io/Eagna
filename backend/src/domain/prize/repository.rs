use super::{Prize, PrizeId};
use crate::domain::point::Point;
use crate::infra::postgres::{prize::NewPrize, PostgresInfra};
use crate::primitive::NonEmptyString;

#[derive(From)]
pub struct PrizeRepository<'a> {
    postgres: &'a dyn PostgresInfra,
}

impl<'a> PrizeRepository<'a> {
    pub fn save_prize(&self, prize: &Prize) -> Result<(), failure::Error> {
        let new_prize = NewPrize {
            id: &prize.id.as_uuid(),
            name: prize.name.as_str(),
            description: prize.description.as_str(),
            thumbnail_url: prize.thumbnail_url.as_str(),
            point: prize.point.as_u32(),
            available: prize.available,
            created: &prize.created,
        };
        self.postgres.save_prize(new_prize)
    }

    pub fn query_all_prizes(&self) -> Result<Vec<Prize>, failure::Error> {
        let query_prizes = self.postgres.query_all_prizes()?;
        let mut prizes = Vec::with_capacity(query_prizes.len());
        for query_prize in query_prizes {
            prizes.push(Prize {
                id: PrizeId::from(query_prize.id),
                name: NonEmptyString::from_str(query_prize.name)?,
                description: query_prize.description,
                thumbnail_url: query_prize.thumbnail_url,
                point: Point::from(query_prize.point),
                available: query_prize.available,
                created: query_prize.created,
            });
        }
        Ok(prizes)
    }
}
