pub struct RewardRecord(HashMap<UserId, Point>);

impl RewardRecord {
    pub fn new(raw_records: HashMap<UserId, Point>) -> RewardRecord {
        RewardRecord(raw_records)
    }
}
