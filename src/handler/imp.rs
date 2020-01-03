use guion::core::ctx::Widgets;
use guion::core::widget::link::Link;
use guion::core::lazout::Size;
use guion::core::env::Env;
use guion::core::util::bounds::Bounds;
use guion::core::ctx::aliases::*;
use super::*;

impl<S,C> GuionHandler<C> for Handler<S,C> where S: GuionHandler<C>, C: Context, C::Handler: AsHandler<Self,C> + 'static {
    #[inline] 
    fn _render<E>(l: Link<E>, r: (&mut ERenderer<E>,&Bounds)) where E: Env<Context=C>, C: Widgets<E> {
        S::_render::<E>(l,r);
        unimplemented!()
    }
    #[inline] 
    fn _event<E>(l: Link<E>, e: (EEvent<E>,&Bounds)) where E: Env<Context=C>, C: Widgets<E> {
        S::_event::<E>(l,e);
        unimplemented!()
    }
    #[inline] 
    fn _event_root<E>(l: Link<E>, e: (EEvent<E>,&Bounds)) where E: Env<Context=C>, C: Widgets<E> {
        S::_event_root::<E>(l,e);
        unimplemented!()
    }
    #[inline] 
    fn _size<E>(l: Link<E>) -> Size where E: Env<Context=C>, C: Widgets<E> {
        unimplemented!();
        S::_size::<E>(l)
    }
}