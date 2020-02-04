use sdl2::EventPump;
use sdl2::EventSubsystem;
use sdl2::TimerSubsystem;
use sdl2::VideoSubsystem;
use sdl2::Sdl;
use super::*;
use handler::HandlerInner;

pub mod queue;
pub mod imp;

//TODO make fields private
pub struct Context<E,H> where E: Env<Context=Self>, H: GuionHandler<E> + AsRefMut<HandlerInner> {
    pub sdl: Sdl,
    pub video: VideoSubsystem,
    pub timer: TimerSubsystem,
    pub event: EventSubsystem,
    pub pump: EventPump,
    pub _e: PhantomData<E>,
    pub handler: H,
}