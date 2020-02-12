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
pub struct Context<E,H> where E: Env<Context=Self> + Sync, H: GuionHandler<E>, EStyle<E>: Default {
    pub sdl: Sdl,
    pub video: VideoSubsystem,
    pub timer: TimerSubsystem,
    pub event: EventSubsystem,
    pub pump: EventPump,
    pub handler: H,
    pub queue: CtxQueue<E>,
    pub default_border: Border,
    pub default_style: EStyle<E>,
}

pub struct CtxQueue<E> where E: Env {
    pub event: EventSubsystem,
    pub timer: TimerSubsystem,
    _e: PhantomData<E>,
}

impl<E,H> Context<E,H> where E: Env<Context=Self> + Sync, H: GuionHandler<E>, EStyle<E>: Default {
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

        let default_border = Border::new(4,4,4,4);
        let default_style = Default::default();

        Ok(
            Self {
                event,pump,timer,queue,video,handler,sdl,default_border,default_style
            }
        )
    }
}