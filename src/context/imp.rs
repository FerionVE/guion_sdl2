use super::*;

impl<E,H> GuionContext<E> for Context<E,H> where E: Env<Context=Self> + Sync, H: GuionHandler<E> + AsRefMut<HandlerInner> {
    type Handler = H;
    type Queue = CtxQueue<E>;
}