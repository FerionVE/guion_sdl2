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
pub struct Context<E,H> where E: Env<Context=Self> + Sync, H: GuionHandler<E> {
    pub sdl: Sdl,
    pub video: VideoSubsystem,
    pub timer: TimerSubsystem,
    pub event: EventSubsystem,
    pub pump: EventPump,
    pub handler: H,
    pub queue: CtxQueue<E>,
}

pub struct CtxQueue<E> where E: Env {
    pub event: EventSubsystem,
    pub timer: TimerSubsystem,
    _e: PhantomData<E>,
}

impl<E,H> Context<E,H> where E: Env<Context=Self> + Sync, H: GuionHandler<E> {
    pub fn from_sdl2(sdl: Sdl, handler: H) -> Result<Self,String> {
        let event = sdl.event()?;
        let pump = sdl.event_pump()?;
        let timer = sdl.timer()?;
        let queue = CtxQueue{
            event: event.clone(),
            timer: timer.clone(),
            _e: PhantomData,
        };
        let video = sdl.video()?;

        Ok(
            Self {
                event,pump,timer,queue,video,handler,sdl,
            }
        )
    }
}