use crate::as_inner::AsInner;
use std::marker::PhantomData;
use guion::core::ctx::AsHandler;
use guion::core::ctx::handler::Handler as GuionHandler;
use guion::core::ctx::Context;

pub mod imp;

pub struct Handler<S,C> where S: GuionHandler<C>, C: Context, C::Link: AsHandler<Self,C> + AsHandler<S,C> + 'static {
    pub sup: S,
    pub inner: HandlerInner,
    _c: PhantomData<C>,
}

pub struct HandlerInner {
    
}

impl<S,C> AsInner<HandlerInner> for Handler<S,C> where S: GuionHandler<C>, C: Context, C::Link: AsHandler<Self,C> + AsHandler<S,C> + 'static {
    #[inline]
    fn inner(&self) -> &HandlerInner {
        &self.inner
    }
    #[inline]
    fn inner_mut(&mut self) -> &mut HandlerInner {
        &mut self.inner
    }
}