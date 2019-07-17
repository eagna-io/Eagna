use super::{schema::organizers, Postgres};
use diesel::{prelude::*, result::Error as PgError};
use uuid::Uuid;

pub trait PostgresOrganizerInfra {
    fn query_organizer(&self, organizer_id: &Uuid)
        -> Result<Option<QueryOrganizer>, failure::Error>;
}

pub struct QueryOrganizer {
    pub id: Uuid,
    pub name: String,
    pub sumbnail_url: String,
}

impl PostgresOrganizerInfra for Postgres {
    fn query_organizer(
        &self,
        organizer_id: &Uuid,
    ) -> Result<Option<QueryOrganizer>, failure::Error> {
        match organizers::table
            .filter(organizers::id.eq(organizer_id))
            .first::<QueryableOrganizer>(&self.conn)
        {
            Ok(query_res) => Ok(Some(QueryOrganizer {
                id: query_res.id,
                name: query_res.name,
                sumbnail_url: query_res.sumbnail_url,
            })),
            Err(PgError::NotFound) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }
}

#[derive(Queryable)]
struct QueryableOrganizer {
    id: Uuid,
    name: String,
    sumbnail_url: String,
}
