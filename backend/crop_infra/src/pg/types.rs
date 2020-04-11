use schemars::JsonSchema;
use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, DbEnum, Serialize, JsonSchema)]
#[DieselType = "Contest_status"]
pub enum ContestStatus {
    Upcoming,
    Open,
    Closed,
    Archived,
}
