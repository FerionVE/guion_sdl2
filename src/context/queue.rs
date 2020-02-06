use super::*;

impl<E> GuionQueue<E> for CtxQueue<E> where E: Env + Sync, E::Context: GuionContext<E,Queue=Self> {
    fn wake(&self) {
        
    }
    fn enqueue_render(&self, force: bool) {

    }
    fn enqueue_event(&self, e: EEvent<E>) {

    }
    fn enqueue_widget_mut(&self, f: impl FnOnce(&mut E::DynWidget)) {
        
    }
}