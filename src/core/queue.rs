use super::*;
use guion::core::widget::Widget;

impl<E> GuionQueue<E> for Queue<E> where E: Env + Sync, E::Context: GuionContext<E,Queue=Self> {
    fn wake(&self) {
        todo!()
    }
    fn enqueue_render(&mut self, force: bool) {
        todo!()
    }
    fn enqueue_event(&mut self, e: (EEvent<E>,&Bounds,u64)) {
        todo!()
    }
    fn enqueue_widget_mut(&mut self, path: E::WidgetPath, f: fn(&mut dyn WidgetMut<E>), invalidate: bool) {
        self.mut_fn.push((path,f,invalidate));
    }
    fn enqueue_widget(&mut self, path: E::WidgetPath, f: fn(&dyn Widget<E>)) {
        todo!()
    }
    fn enqueue_widget_mut_closure(&mut self, path: E::WidgetPath, f: impl FnOnce(&mut dyn WidgetMut<E>)+Sync+'static, invalidate: bool) {
        todo!()
    }
    fn enqueue_widget_closure(&mut self, path: E::WidgetPath, f: impl FnOnce(&dyn Widget<E>)+Sync+'static) {
        todo!()
    }
    fn enqueue_widget_invalidate(&mut self, path: E::WidgetPath) {
        //todo!()
    }
    fn enqueue_widget_validate(&mut self, path: E::WidgetPath) {
        //todo!()
    }
}

impl<E> AsRefMut<Self> for Queue<E> where E: Env {
    fn as_ref(&self) -> &Self {
        self
    }
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}