pub mod contest;

use self::contest::ContestManager;

use crop_domain::contest::model::Contest;

#[derive(Clone)]
pub struct Context {
    contest: ContestManager,
}

impl Context {
    pub fn new(contest: Contest) -> Context {
        Context {
            contest: ContestManager::new(contest),
        }
    }

    pub fn contest_manager(&self) -> ContestManager {
        self.contest.clone()
    }
}
