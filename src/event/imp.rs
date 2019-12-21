use guion::core::event::Destination;
use guion::core::util::bounds::Bounds;
use super::*;

impl<E> GuionEvent<E> for Event where E: Env<Event=Self> {
    fn filter(self, subbounds: &Bounds) -> Option<Self> {

    }
    /// True if container widgets should sent this to only one widget  
    fn consuming(&self) -> bool {

    }
    /// Where there Event should be initially injected into the context
    fn destination(&self) -> E::EventDest {
        
    }
}
/// (consuming,dest,)
#[inline]
fn destination_of<D: Destination>(e: &SDLEvent) -> D {
    match e {
        SDLEvent::KeyDown{..} => D::SELECTED,
        SDLEvent::KeyUp{..} => D::SELECTED,
        SDLEvent::TextEditing{..} => D::SELECTED,
        SDLEvent::TextInput{..} => D::SELECTED,
        _ => D::ROOT,
    }
}