use guion::{EventResp, render::link::RenderLink, event::compound::EventCompound};
use super::*;

impl<S,E> GuionHandler<E> for Handler<S,E> where S: GuionHandler<E>, E: Env + Sync {
    #[inline] 
    fn _render(l: Link<E>, r: &mut RenderLink<E>) {
        S::_render(l,r)
        //todo!() //TODO impl everything
    }
    #[inline] 
    fn _event_direct(l: Link<E>, e: &EventCompound<E>) -> EventResp {
        S::_event_direct(l,e)
    }
    #[inline] 
    fn _event_root(l: Link<E>, e: &EventCompound<E>) -> EventResp {
        S::_event_root(l,e)
    }
    #[inline] 
    fn _size(l: Link<E>, e: &ESVariant<E>) -> ESize<E> {
        //todo!();
        S::_size(l,e)
    }
    #[inline]
    fn _send_event(l: Link<E>, e: &EventCompound<E>, child: E::WidgetPath) -> Result<EventResp,()> {
        S::_send_event(l,e,child)
    }
}
