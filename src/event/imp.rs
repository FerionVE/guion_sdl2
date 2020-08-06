use crate::event::position::*;
use crate::event::destination::*;
use super::*;
use guion::event::variant::Variant;
use std::fmt::Debug;

//TODO fix C: Clone requirement
impl<E> Variant<E> for Event where E: Env, EEDest<E>: SDLDestination {
    #[inline]
    fn in_bounds(&self, b: &Bounds) -> bool {
        pos_of(self,self.ws.0,self.ws.1)
            .map_or(true,|o| o.is_inside(b) )
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

impl Debug for Event {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}
