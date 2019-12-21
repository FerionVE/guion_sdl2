use std::ops::DerefMut;
use std::marker::PhantomData;
use std::ops::Deref;
use guion::core::ctx::Env;
use guion::core::event::Event as GuionEvent;
use sdl2::event::Event as SDLEvent;

pub mod imp;

#[derive(Clone)]
pub struct Event {
    pub e: SDLEvent,
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