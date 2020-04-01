use crate::poll::model::Poll;

pub struct Contest {
    polls: Vec<Poll>,
}

impl Contest {
    pub fn new() -> Contest {
        Contest { polls: Vec::new() }
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
