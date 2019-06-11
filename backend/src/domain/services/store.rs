pub trait Store {
    type Error: failure::AsFail + std::fmt::Debug;
}

pub trait StoreFactory<S: Store> {
    fn transaction<F, T, E>(&self, f: F) -> Result<T, E>
    where
        F: for<'a> FnOnce(&'a mut S) -> Result<T, E>,
        E: From<S::Error>;
}
