pub trait Store {
    type Error: failure::AsFail + std::fmt::Debug;

    fn commit(self) -> Result<(), Self::Error>;
}

pub trait StoreFactory<S: Store> {
    fn establish(&self) -> S;
}
