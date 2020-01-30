use crate::event::key::Key;
use crate::event::consuming::SDLConsuming;
use crate::event::destination::SDLDestination;
use super::*;

pub mod imp;
pub mod destination;
pub mod consuming;
pub mod position;
pub mod support;
pub mod key;

#[derive(Clone)]
pub struct Event<K,D,C> where K: GuionKey + FromInto<Key> + 'static, D: SDLDestination, C: SDLConsuming {
    pub e: SDLEvent,
    ws: (f32,f32),
    _m: PhantomData<(K,D,C)>,
}

impl<K,D,C> Deref for Event<K,D,C> where K: GuionKey + FromInto<Key> + 'static, D: SDLDestination, C: SDLConsuming {
    type Target = SDLEvent;

    fn deref(&self) -> &Self::Target {
        &self.e
    }
}

impl<K,D,C> DerefMut for Event<K,D,C> where K: GuionKey + FromInto<Key> + 'static, D: SDLDestination, C: SDLConsuming {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.e
    }
}