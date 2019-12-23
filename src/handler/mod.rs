use guion::core::util::AsRefMut;
use std::marker::PhantomData;
use guion::core::ctx::AsHandler;
use guion::core::ctx::handler::Handler as GuionHandler;
use guion::core::ctx::Context;

pub mod imp;

pub struct Handler<S,C> where S: GuionHandler<C>, C: Context, C::Handler: AsHandler<Self,C> + 'static {
    pub sup: S,
    pub inner: HandlerInner,
    _c: PhantomData<C>,
}

pub struct HandlerInner {
    
}

impl<S,C> AsRefMut<HandlerInner> for Handler<S,C> where S: GuionHandler<C>, C: Context, C::Handler: AsHandler<Self,C> + 'static {
    #[inline]
    fn as_ref(&self) -> &HandlerInner {
        &self.inner
    }
    #[inline]
    fn as_mut(&mut self) -> &mut HandlerInner {
        &mut self.inner
    }
}

impl<S,C> AsHandler<Self,C> for Handler<S,C> where S: GuionHandler<C>, C: Context<Handler=Self> {
    fn as_mut(c: &mut C) -> &mut Self {
        c._handler_mut()
    }
    fn as_ref(c: &C) -> &Self {
        c._handler()
    }
}

impl<S,C> AsHandler<S,C> for Handler<S,C> where S: GuionHandler<C>, C: Context<Handler=Self> {
    fn as_mut(c: &mut C) -> &mut S {
        &mut c._handler_mut().sup
    }
    fn as_ref(c: &C) -> &S {
        &c._handler().sup
    }
}