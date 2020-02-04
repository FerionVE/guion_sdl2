use super::*;

impl<E,H> GuionQueue<E> for Context<E,H> where E: Env<Context=Self>, H: GuionHandler<E> + AsRefMut<HandlerInner> {
    fn wake(&self) {
        
    }
    fn enqueue_render(&self, force: bool) {

    }
    fn enqueue_event(&self, e: EEvent<E>) {

    }
    fn euqueue_widget_mut(&self, f: impl FnOnce(&mut E::DynWidget)) {
        
    }
}