use super::{Prize, PrizeId};
use crate::domain::point::Point;
use crate::infra::postgres::{
    prize::{NewPrize, QueryPrize},
    PostgresInfra,
};
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
        Ok(self
            .postgres
            .query_all_prizes()?
            .into_iter()
            .map(convert_infra_prize_into_model)
            .collect())
    }

    pub fn query_prize(&self, prize_id: &PrizeId) -> Result<Option<Prize>, failure::Error> {
        Ok(self
            .postgres
            .query_prize(prize_id.as_uuid())?
            .map(convert_infra_prize_into_model))
    }
}

fn convert_infra_prize_into_model(prize: QueryPrize) -> Prize {
    Prize {
        id: PrizeId::from(prize.id),
        name: NonEmptyString::from_str(prize.name).expect("Prize name MUST NOT empty"),
        description: prize.description,
        thumbnail_url: prize.thumbnail_url,
        point: Point::from(prize.point),
        available: prize.available,
        created: prize.created,
    }
}
