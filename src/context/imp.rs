use super::*;

impl<E,H> GuionContext<E> for Context<E,H> where E: Env<Context=Self> + Sync, H: GuionHandler<E> {
    type Handler = H;
    type Queue = CtxQueue<E>;
    fn queue_mut(&mut self) -> &mut Self::Queue {
        todo!()
    }
    fn queue(&self) -> &Self::Queue {
        todo!()
    }
    fn _handler_mut(&mut self) -> &mut Self::Handler {
        todo!()
    }
    fn _handler(&self) -> &Self::Handler {
        todo!()
    }
    fn default_style(&self) -> &EStyle<E> {
        todo!()
    }
    fn default_border(&self) -> &Border {
        todo!()
    }
}