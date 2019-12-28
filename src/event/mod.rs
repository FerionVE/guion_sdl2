use crate::event::consuming::SDLConsuming;
use crate::event::destination::SDLDestination;
use std::marker::PhantomData;
use std::ops::DerefMut;
use std::ops::Deref;
use guion::core::env::Env;
use guion::core::event::Event as GuionEvent;
use sdl2::event::Event as SDLEvent;

pub mod imp;
pub mod destination;
pub mod consuming;
pub mod position;

#[derive(Clone)]
pub struct Event<K,D,C> where D: SDLDestination, C: SDLConsuming {
    pub e: SDLEvent,
    ws: (f32,f32),
    _m: PhantomData<(K,D,C)>,
}

impl<K,D,C> Deref for Event<K,D,C> where D: SDLDestination, C: SDLConsuming {
    type Target = SDLEvent;

    fn deref(&self) -> &Self::Target {
        &self.e
    }
}

impl<K,D,C> DerefMut for Event<K,D,C> where D: SDLDestination, C: SDLConsuming {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.e
    }
}