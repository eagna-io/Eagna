use crate::poll::model::Poll;
use uuid::Uuid;

pub struct Contest {
    pub id: ContestId,
    pub polls: Vec<Poll>,
}

pub struct ContestId(pub Uuid);

impl Contest {
    pub fn new() -> Contest {
        Contest {
            id: ContestId::new(),
            polls: Vec::new(),
        }
    }

    pub fn current_poll(&self) -> Option<&Poll> {
        self.polls.last()
    }

    pub fn current_poll_mut(&mut self) -> Option<&mut Poll> {
        self.polls.last_mut()
    }

    pub fn add_poll(&mut self, poll: Poll) -> &Poll {
        self.polls.push(poll);
        self.current_poll().unwrap()
    }
}

impl ContestId {
    pub fn new() -> ContestId {
        ContestId(Uuid::new_v4())
    }
}
