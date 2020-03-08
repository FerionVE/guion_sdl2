use crate::event::position::*;
use crate::event::destination::*;
use crate::event::consuming::*;
use super::*;
use guion::core::event::Variant;

//TODO fix C: Clone requirement
impl<E> Variant<E> for Event where E: Env, EEDest<E>: SDLDestination {
    #[inline]
    fn position(&self) -> Option<Offset> {
        pos_of(self,self.ws.0,self.ws.1)
    }
    #[inline]
    fn consuming(&self) -> bool {
        todo!()
    }
    #[inline]
    fn destination(&self) -> EEDest<E> {
        SDLDestination::destination_of(self)
    }
}
