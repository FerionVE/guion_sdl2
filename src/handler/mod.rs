use std::marker::PhantomData;
use guion::core::ctx::AsHandler;
use guion::core::ctx::handler::Handler as GuionHandler;
use guion::core::ctx::Context;

pub mod imp;

pub struct Handler<S,C> where S: GuionHandler<C>, C: Context, C::Link: AsHandler<Self,C> + AsHandler<S,C> + 'static {
    pub sup: S,
    pub hi: HandlerInner,
    _c: PhantomData<C>,
}

pub struct HandlerInner {

}

pub trait AsSDLHandler<C>: Sized where C: Context<Link=Self> {
    fn as_ref(&self) -> &HandlerInner;
    fn as_mut(&mut self) -> &mut HandlerInner;
}