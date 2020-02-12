use super::*;

impl<E,H> GuionContext<E> for Context<E,H> where E: Env<Context=Self> + Sync, H: GuionHandler<E>, EStyle<E>: Default {
    type Handler = H;
    type Queue = CtxQueue<E>;
    fn queue_mut(&mut self) -> &mut Self::Queue {
        &mut self.queue
    }
    fn queue(&self) -> &Self::Queue {
        &self.queue
    }
    fn _handler_mut(&mut self) -> &mut Self::Handler {
        &mut self.handler
    }
    fn _handler(&self) -> &Self::Handler {
        &self.handler
    }
    fn default_style(&self) -> &EStyle<E> {
        &self.default_style
    }
    fn default_border(&self) -> &Border {
        &self.default_border
    }
}