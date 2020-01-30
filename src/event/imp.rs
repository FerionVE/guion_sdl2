use crate::event::position::*;
use crate::event::destination::*;
use crate::event::consuming::*;
use super::*;

//TODO fix C: Clone requirement
impl<E,K,D,C> GuionEvent<E> for Event<K,D,C> where K: GuionKey + FromInto<Key> + 'static, E: Env, E::Backend: GuionBackend<E,Event=Self>, D: SDLDestination, C: SDLConsuming {
    type Dest = D;
    type Key = K;
    
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
        C::consuming_of(self)
    }
    #[inline]
    fn destination(&self) -> Self::Dest {
        Self::Dest::destination_of(self)
    }
    #[inline]
    fn position(&self) -> Option<Offset> {
        pos_of(&self.e,self.ws.0,self.ws.1)
    }
}
