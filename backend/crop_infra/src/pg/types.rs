use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, DbEnum, Serialize, Deserialize, JsonSchema)]
#[DieselType = "Contest_status"]
pub enum ContestStatus {
    Upcoming,
    Open,
    Closed,
    Archived,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, DbEnum, Serialize, Deserialize, JsonSchema)]
#[DieselType = "Poll_status"]
pub enum PollStatus {
    Open,
    Closed,
}
