use super::*;
use guion::core::{ctx::StdEnqueueable, widget::Widget};

impl<E> GuionQueue<StdEnqueueable<E>> for Queue<E> where E: Env + Sync, E::Context: GuionContext<E,Queue=Self> {
    fn push(&mut self, v: StdEnqueueable<E>) {
        match v {
            StdEnqueueable::Render { force } => (),
            StdEnqueueable::Event { event, ts } => (),
            StdEnqueueable::MutateWidget { path, f, invalidate } => 
            self.mut_fn.push_back((path,f,invalidate)),
            StdEnqueueable::MutateWidgetClosure { path, f, invalidate } => (),
            StdEnqueueable::MutateRoot { f } => (),
            StdEnqueueable::MutateRootClosure { f } => (),
            StdEnqueueable::AccessWidget { path, f } => (),
            StdEnqueueable::AccessWidgetClosure { path, f } => (),
            StdEnqueueable::AccessRoot { f } => (),
            StdEnqueueable::AccessRootClosure { f } => (),
            StdEnqueueable::InvalidateWidget { path } => (),
            StdEnqueueable::ValidateWidgetRender { path } => (),
            StdEnqueueable::ValidateWidgetSize { path, size } => (),
        }
    }
    fn send(&self, v: StdEnqueueable<E>) {
        todo!()
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