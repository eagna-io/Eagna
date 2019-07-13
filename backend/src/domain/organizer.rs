use crate::infra::PostgresInfra;
use uuid::Uuid;

#[derive(Debug, Serialize)]
/// アプリケーション層がこのモデルを取得するためには
/// リポジトリを通じて取得するしかない。
pub struct Organizer {
    id: OrganizerId,
    name: OrganizerName,
    sumbnail_url: OrganizerSumbnailUrl,
}

impl Organizer {
    pub fn id(&self) -> &OrganizerId {
        &self.id
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, From)]
pub struct OrganizerId(Uuid);

impl OrganizerId {
    pub fn to_str(&self) -> impl AsRef<str> {
        let mut str_buf = Uuid::encode_buffer();
        let tmp_str = self.0.to_simple_ref().encode_lower(&mut str_buf);
        arrayvec::ArrayString::<[u8; uuid::adapter::Simple::LENGTH]>::from(tmp_str).unwrap()
    }

    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

#[derive(Debug, Serialize, From)]
pub struct OrganizerName(String);

#[derive(Debug, Serialize, From)]
pub struct OrganizerSumbnailUrl(String);

#[derive(From)]
pub struct OrganizerRepository<'a> {
    postgres: &'a dyn PostgresInfra,
}

impl<'a> OrganizerRepository<'a> {
    pub fn query_organizer(
        &self,
        organizer_id: &OrganizerId,
    ) -> Result<Option<Organizer>, failure::Error> {
        if let Some(query_res) = self.postgres.query_organizer(organizer_id.to_str().as_ref())? {
            Ok(Some(Organizer {
                id: OrganizerId::from(query_res.id),
                name: OrganizerName::from(query_res.name),
                sumbnail_url: OrganizerSumbnailUrl::from(query_res.sumbnail_url),
            }))
        } else {
            Ok(None)
        }
    }
}
