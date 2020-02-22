use sdl2::EventPump;
use sdl2::EventSubsystem;
use sdl2::TimerSubsystem;
use sdl2::VideoSubsystem;
use sdl2::Sdl;
use super::*;

pub mod queue;
//pub mod imp;

//TODO make fields private
pub struct Core<E> where E: Env + Sync {
    pub sdl: Sdl,
    pub video: VideoSubsystem,
    pub timer: TimerSubsystem,
    pub event: EventSubsystem,
    pub pump: EventPump,
    pub queue: Queue<E>,
    pub default_border: Border,
    pub default_style: EStyle<E>,
}

pub struct Queue<E> where E: Env {
    pub event: EventSubsystem,
    pub timer: TimerSubsystem,
    _e: PhantomData<E>,
}

impl<E> Core<E> where E: Env + Sync {
    pub fn from_sdl2(sdl: Sdl) -> Result<Self,String> {
        let event = sdl.event()?;
        let pump = sdl.event_pump()?;
        let timer = sdl.timer()?;
        let queue = Queue{
            event: event.clone(),
            timer: timer.clone(),
            _e: PhantomData,
        };
        let video = sdl.video()?;

        let default_border = Border::new(4,4,4,4);
        let default_style = EStyle::<E>::static_default();

        Ok(
            Self {
                event,pump,timer,queue,video,sdl,default_border,default_style
            }
        )
    }
}