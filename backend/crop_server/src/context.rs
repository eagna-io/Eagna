use crop_infra::pg::Pool;

#[derive(Clone)]
pub struct Context {
    pub pg: Pool,
}

impl Context {
    pub fn new(pg: Pool) -> Context {
        Context { pg }
    }
}
