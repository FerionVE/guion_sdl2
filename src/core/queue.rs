use super::*;

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
    fn enqueue_widget_mut(&self, path: E::WidgetPath, f: fn(&mut E::DynWidget), invalidate: bool) {
        todo!()
    }
    fn enqueue_widget(&self, path: E::WidgetPath, f: fn(&E::DynWidget)) {
        todo!()
    }
    fn enqueue_widget_mut_closure(&self, path: E::WidgetPath, f: impl FnOnce(&mut E::DynWidget), invalidate: bool) {
        todo!()
    }
    fn enqueue_widget_closure(&self, path: E::WidgetPath, f: impl FnOnce(&E::DynWidget)) {
        todo!()
    }
    fn enqueue_widget_invalidate(&self, path: E::WidgetPath) {
        //todo!()
    }
    fn enqueue_widget_validate(&self, path: E::WidgetPath) {
        //todo!()
    }
}