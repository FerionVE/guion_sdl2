use super::*;
use crate::core::Core;

pub mod imp;

pub struct Handler<S,E> where S: GHandler<E>, E: Env + Sync + 'static {
    pub sup: S,
    pub inner: Core<E>,
}

impl<S,E> Handler<S,E> where S: GHandler<E>, E: Env + Sync + 'static {
    #[inline]
    pub fn new(inner: Core<E>, sup: S) -> Self {
        Self{
            sup,
            inner,
        }
    }
}
