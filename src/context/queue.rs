use guion::core::ctx::aliases::EEvent;
use guion::core::env::Env;
use guion::core::ctx::queue::{Queue as GuionQueue,Enqueue};
use super::*;

impl<E> GuionQueue<E> for Context where E: Env {
    fn wake(&self) {

    }
    fn enqueue_render(&self, force: bool) {

    }
    fn enqueue_event(&self, e: EEvent<E>) {

    }
    fn euqueue_widget_mut(&self, f: impl FnOnce(&mut E::DynWidget)) {
        
    }
}