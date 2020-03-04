use guion::core::render::link::RenderLink;
use super::*;

impl<S,E> GuionHandler<E> for Handler<S,E> where S: GuionHandler<E>, E: Env + Sync {
    #[inline] 
    fn _render(l: Link<E>, r: &mut RenderLink<E>) -> bool {
        S::_render(l,r)
        //todo!() //TODO impl everything
    }
    #[inline] 
    fn _event(l: Link<E>, e: (EEvent<E>,&Bounds,u64)) {
        S::_event(l,e);
        //todo!()
    }
    #[inline] 
    fn _event_root(l: Link<E>, e: (EEvent<E>,&Bounds,u64)) {
        S::_event_root(l,e);
        //todo!()
    }
    #[inline] 
    fn _size(l: Link<E>) -> ESize<E> {
        //todo!();
        S::_size(l)
    }
}