use super::*;
use guion::core::path::WPSlice;

impl<E> GuionQueue<E> for Queue<E> where E: Env + Sync, E::Context: GuionContext<E,Queue=Self> {
    fn wake(&self) {
        todo!()
    }
    fn enqueue_render(&self, force: bool) {
        todo!()
    }
    fn enqueue_event(&self, e: (EEvent<E>,&Bounds,u64)) {
        todo!()
    }
    fn enqueue_widget_mut(&self, path: WPSlice<E>, f: fn(&mut E::DynWidget), invalidate: bool) {
        todo!()
    }
    fn enqueue_widget(&self, path: WPSlice<E>, f: fn(&E::DynWidget)) {
        todo!()
    }
    fn enqueue_widget_mut_closure(&self, path: WPSlice<E>, f: impl FnOnce(&mut E::DynWidget), invalidate: bool) {
        todo!()
    }
    fn enqueue_widget_closure(&self, path: WPSlice<E>, f: impl FnOnce(&E::DynWidget)) {
        todo!()
    }
    fn enqueue_widget_invalidate(&self, path: WPSlice<E>) {
        todo!()
    }
    fn enqueue_widget_validate(&self, path: WPSlice<E>) {
        //todo!()
    }
}