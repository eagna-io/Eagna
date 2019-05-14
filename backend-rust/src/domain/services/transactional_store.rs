pub trait TransactionalStore {
    fn transaction<F, T, E>(&self, f: F) -> Result<T, E>
    where
        F: FnOnce() -> Result<T, E>;
}
