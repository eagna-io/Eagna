#[derive(Debug, Clone, PartialEq, Eq, DbEnum)]
#[DieselType = "Contest_status"]
pub enum ContestStatus {
    Upcoming,
    Open,
    Closed,
    Archived,
}
