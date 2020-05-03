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

impl Into<domain::contest::ContestStatus> for ContestStatus {
    fn into(self) -> domain::contest::ContestStatus {
        match self {
            ContestStatus::Upcoming => domain::contest::ContestStatus::Upcoming,
            ContestStatus::Open => domain::contest::ContestStatus::Open,
            ContestStatus::Closed => domain::contest::ContestStatus::Closed,
            ContestStatus::Archived => domain::contest::ContestStatus::Archived,
        }
    }
}

impl From<domain::contest::ContestStatus> for ContestStatus {
    fn from(status: domain::contest::ContestStatus) -> Self {
        match status {
            domain::contest::ContestStatus::Upcoming => ContestStatus::Upcoming,
            domain::contest::ContestStatus::Open => ContestStatus::Open,
            domain::contest::ContestStatus::Closed => ContestStatus::Closed,
            domain::contest::ContestStatus::Archived => ContestStatus::Archived,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, DbEnum, Serialize, Deserialize, JsonSchema)]
#[DieselType = "Poll_status"]
pub enum PollStatus {
    Open,
    Closed,
}

impl Into<domain::contest::PollStatus> for PollStatus {
    fn into(self) -> domain::contest::PollStatus {
        match self {
            PollStatus::Open => domain::contest::PollStatus::Open,
            PollStatus::Closed => domain::contest::PollStatus::Closed,
        }
    }
}

impl From<domain::contest::PollStatus> for PollStatus {
    fn from(status: domain::contest::PollStatus) -> Self {
        match status {
            domain::contest::PollStatus::Open => PollStatus::Open,
            domain::contest::PollStatus::Closed => PollStatus::Closed,
        }
    }
}
