mod contest;

pub use contest::ContestManager;

#[derive(Clone)]
pub struct Context {
    pub contest_manager: ContestManager,
}

impl Context {
    pub fn new() -> Context {
        Context {
            contest_manager: ContestManager::new(),
        }
    }
}
