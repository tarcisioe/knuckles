pub trait Strong<T> {
    fn get(self) -> T;
    fn get_ref(&self) -> &T;
}
