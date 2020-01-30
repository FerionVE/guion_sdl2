use super::*;

impl<E> GuionQueue<E> for Context where E: Env, E::Context: GuionContext<E,Queue=Self> {
    fn wake(&self) {
        
    }
    fn enqueue_render(&self, force: bool) {

    }
    fn enqueue_event(&self, e: EEvent<E>) {

    }
    fn euqueue_widget_mut(&self, f: impl FnOnce(&mut E::DynWidget)) {
        
    }
}