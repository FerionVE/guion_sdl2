use guion::core::util::bounds::Offset;
use crate::event::position::*;
use crate::event::destination::*;
use crate::event::consuming::*;
use guion::core::util::bounds::Bounds;
use super::*;

impl<E> GuionEvent<E> for Event where E: Env<Event=Self>, E::EventDest: SDLDestination, E::EventConsuming: SDLConsuming {
    #[inline]
    fn filter(self, subbounds: &Bounds) -> Option<Self> {
        let pos = pos_of(&self.e,self.ws.0,self.ws.1);
        
        let pos = pos.map_or(true, |p| p.is_inside(subbounds));

        if pos {
            Some(self)
        }else{
            None
        }
    }
    #[inline]
    fn consuming(&self) -> bool {
        E::EventConsuming::consuming_of(self)
    }
    #[inline]
    fn destination(&self) -> E::EventDest {
        E::EventDest::destination_of(self)
    }
    #[inline]
    fn position(&self) -> Option<Offset> {
        pos_of(&self.e,self.ws.0,self.ws.1)
    }
}
