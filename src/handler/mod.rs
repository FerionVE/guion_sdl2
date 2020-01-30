use super::*;

pub mod imp;

pub struct Handler<S,E> where S: GuionHandler<E>, E: Env, ECHandler<E>: AsHandler<Self,E> + 'static {
    pub sup: S,
    pub inner: HandlerInner,
    _c: PhantomData<E>,
}

pub struct HandlerInner {
    
}

impl<S,E> AsRefMut<HandlerInner> for Handler<S,E> where S: GuionHandler<E>, E: Env, ECHandler<E>: AsHandler<Self,E> + 'static {
    #[inline]
    fn as_ref(&self) -> &HandlerInner {
        &self.inner
    }
    #[inline]
    fn as_mut(&mut self) -> &mut HandlerInner {
        &mut self.inner
    }
}

impl<S,E> AsHandler<Self,E> for Handler<S,E> where S: GuionHandler<E>, E: Env, E::Context: GuionContext<E,Handler=Self> {
    fn as_mut(c: &mut E::Context) -> &mut Self {
        c._handler_mut()
    }
    fn as_ref(c: &E::Context) -> &Self {
        c._handler()
    }
}

impl<S,E> AsHandler<S,E> for Handler<S,E> where S: GuionHandler<E>, E: Env, E::Context: GuionContext<E,Handler=Self> {
    fn as_mut(c: &mut E::Context) -> &mut S {
        &mut c._handler_mut().sup
    }
    fn as_ref(c: &E::Context) -> &S {
        &c._handler().sup
    }
}