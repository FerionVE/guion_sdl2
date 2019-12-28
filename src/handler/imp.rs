use guion::core::lazout::Size;
use guion::core::env::Env;
use guion::core::util::bounds::Bounds;
use guion::core::ctx::aliases::*;
use super::*;

impl<S,C> GuionHandler<C> for Handler<S,C> where S: GuionHandler<C>, C: Context, C::Handler: AsHandler<Self,C> + 'static {
    #[inline] 
    fn _render<E: Env>(senf: &mut C, i: &E::WidgetID, r: (&mut ERenderer<E>,&Bounds)) {
        S::_render::<E>(senf,i,r);
        unimplemented!()
    }
    #[inline] 
    fn _event<E: Env>(senf: &mut C, i: &E::WidgetID, e: EEvent<E>) {
        S::_event::<E>(senf,i,e);
        unimplemented!()
    }
    #[inline] 
    fn _event_root<E: Env>(senf: &mut C, i: &E::WidgetID, e: EEvent<E>) {
        Self::_event::<E>(senf,i,e);
        unimplemented!()
    }
    #[inline] 
    fn _size<E: Env>(senf: &mut C, i: &E::WidgetID) -> Size {
        unimplemented!();
        S::_size::<E>(senf,i)
    }
}