use sdl2::EventPump;
use sdl2::EventSubsystem;
use sdl2::TimerSubsystem;
use sdl2::VideoSubsystem;
use sdl2::Sdl;
use super::*;
use guion::core::{ctx::invalidate, render::link::RenderLink};

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
    pub validate: Vec<E::WidgetPath>,
    pub invalidate: Vec<E::WidgetPath>,
}

impl<E> Core<E> where E: Env + Sync {
    pub fn from_sdl2(sdl: Sdl) -> Result<Self,String> {
        let event = sdl.event()?;
        let pump = sdl.event_pump()?;
        let timer = sdl.timer()?;
        let queue = Queue{
            event: event.clone(),
            timer: timer.clone(),
            validate: Vec::with_capacity(4096),
            invalidate: Vec::with_capacity(4096),
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

/// render widget and process validation
pub fn render<E>(r: &mut RenderLink<E>, w: E::WidgetPath, stor: &mut E::Storage, c: &mut E::Context) where E: Env, ECQueue<E>: AsRefMut<Queue<E>> {
    {
        let q = c.queue_mut().as_mut();
        for p in &q.invalidate {
            invalidate::<E>(stor,p.refc()).expect("Lost Widget in invalidate");
        }
        q.invalidate.clear();
    }
    let w = c.link(
        stor.widget(w).expect("Lost Widget in render")
    );
    r.render_widget(w);
    {
        let q = c.queue_mut().as_mut();
        for p in &q.validate {
            invalidate::<E>(stor,p.refc()).expect("Lost Widget in invalidate");
        }
        q.validate.clear();
    }
}