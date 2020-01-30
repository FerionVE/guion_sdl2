use super::*;

impl<S,E> GuionHandler<E> for Handler<S,E> where S: GuionHandler<E>, E: Env, ECHandler<E>: AsHandler<Self,E> + 'static {
    #[inline] 
    fn _render(l: Link<E>, r: (&mut ERenderer<E>,&Bounds)) {
        S::_render(l,r);
        unimplemented!()
    }
    #[inline] 
    fn _event(l: Link<E>, e: (EEvent<E>,&Bounds)) {
        S::_event(l,e);
        unimplemented!()
    }
    #[inline] 
    fn _event_root(l: Link<E>, e: (EEvent<E>,&Bounds)) {
        S::_event_root(l,e);
        unimplemented!()
    }
    #[inline] 
    fn _size(l: Link<E>) -> ESize<E> {
        unimplemented!();
        S::_size(l)
    }
}