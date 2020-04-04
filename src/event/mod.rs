use crate::event::key::Key;
use crate::event::destination::SDLDestination;
use super::*;

pub mod imp;
pub mod destination;
pub mod consuming;
pub mod position;
pub mod key;
pub mod cast;

#[derive(Clone)]
pub struct Event {
    pub e: SDLEvent,
    ws: (f32,f32),
}

impl Deref for Event {
    type Target = SDLEvent;

    fn deref(&self) -> &Self::Target {
        &self.e
    }
}

impl DerefMut for Event {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.e
    }
}