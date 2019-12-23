pub trait AsInner<T> {
    fn inner(&self) -> &T;
    fn inner_mut(&mut self) -> &mut T;
}

impl<T,I> AsInner<I> for T where T: AsRef<I> + AsMut<I> {
    #[inline]
    fn inner(&self) -> &I {
        self.as_ref()
    }
    #[inline]
    fn inner_mut(&mut self) -> &mut I {
        self.as_mut()
    }
}