use super::*;
use guion::core::path::WPSlice;

impl<E> GuionQueue<E> for CtxQueue<E> where E: Env + Sync, E::Context: GuionContext<E,Queue=Self> {
    fn wake(&self) {
        todo!()
    }
    fn enqueue_render(&self, force: bool) {
        todo!()
    }
    fn enqueue_event(&self, e: EEvent<E>) {
        todo!()
    }
    fn enqueue_widget_mut(&self, path: WPSlice<E>, f: impl FnOnce(&mut E::DynWidget)) {
        todo!()
    }
    fn enqueue_widget(&self, path: WPSlice<E>, f: impl FnOnce(&E::DynWidget)) {
        todo!()
    }
}