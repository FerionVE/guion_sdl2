use super::*;

impl<E> GuionQueue<StdEnqueueable<E>> for Queue<E> where E: Env + Sync, /*ECQueue<E>: AsRefMut<Self>*/ {
    fn push(&mut self, v: StdEnqueueable<E>) {
        match v {
            StdEnqueueable::InvalidateWidget { path } => self.invalidate.push(path),
            StdEnqueueable::ValidateWidgetRender { path } => self.validate_render.push(path),
            StdEnqueueable::ValidateWidgetSize { path, size } => self.validate_size.push((path,size)),
            _ => self.mut_fn.push_back(v),
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