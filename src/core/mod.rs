use super::*;
use guion::core::{ctx::invalidate, render::link::RenderLink, widget::WidgetMut};
use render::font::TextProd;
use sdl2::EventPump;
use sdl2::EventSubsystem;
use sdl2::TimerSubsystem;
use sdl2::VideoSubsystem;
use sdl2::{ttf::Sdl2TtfContext, Sdl};

pub mod queue;
//pub mod imp;

//TODO make fields private
pub struct Core<E>
where
    E: Env + Sync,
{
    pub sdl: Sdl,
    pub video: VideoSubsystem,
    pub timer: TimerSubsystem,
    pub event: EventSubsystem,
    pub pump: EventPump,
    pub queue: Queue<E>,
    pub font: TextProd,
    pub default_border: Border,
    pub default_style: EStyle<E>,
}

pub struct Queue<E>
where
    E: Env,
{
    pub event: EventSubsystem,
    pub timer: TimerSubsystem,
    pub validate: Vec<E::WidgetPath>,
    pub invalidate: Vec<E::WidgetPath>,
    pub mut_fn: Vec<(E::WidgetPath, fn(&mut dyn WidgetMut<E>), bool)>,
}

impl<E> Core<E>
where
    E: Env + Sync,
{
    pub fn from_sdl2(sdl: Sdl, ttf: Sdl2TtfContext) -> Result<Self, String> {
        let event = sdl.event()?;
        let pump = sdl.event_pump()?;
        let timer = sdl.timer()?;
        let queue = Queue {
            event: event.clone(),
            timer: timer.clone(),
            validate: Vec::with_capacity(4096),
            invalidate: Vec::with_capacity(4096),
            mut_fn: Vec::with_capacity(4096),
        };
        let video = sdl.video()?;
        let font = TextProd::new(ttf)?;

        let default_border = Border::new(4, 4, 4, 4);
        let default_style = EStyle::<E>::static_default();

        Ok(Self {
            event,
            pump,
            timer,
            queue,
            video,
            sdl,
            default_border,
            default_style,
            font,
        })
    }
}

impl<E> AsRefMut<Self> for Core<E> where E: Env {
    fn as_ref(&self) -> &Self {
        self
    }
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

/// render widget and process validation
pub fn render_and_events<E>(r: &mut RenderLink<E>, w: E::WidgetPath, stor: &mut E::Storage, c: &mut E::Context)
where
    E: Env,
    ECQueue<E>: AsRefMut<Queue<E>>,
{
    pre_render_events::<E>(stor, c);
    let w = c.link(stor.widget(w).expect("Lost Widget in render"));
    r.render_widget(w);
    post_render_events::<E>(stor, c);
}

pub fn pre_render_events<E>(stor: &mut E::Storage, c: &mut E::Context)
where
    E: Env,
    ECQueue<E>: AsRefMut<Queue<E>>,
{
    let q = c.queue_mut().as_mut();

    for (p, f, i) in &q.mut_fn {
        let mut w = stor._widget_mut(p.refc(), *i).expect("TODO");
        f(w.widget());
    };
    q.mut_fn.clear();

    for p in &q.invalidate {
        invalidate::<E>(stor, p.refc()).expect("Lost Widget in invalidate");
    }
    q.invalidate.clear();
}

pub fn post_render_events<E>(stor: &mut E::Storage, c: &mut E::Context)
where
    E: Env,
    ECQueue<E>: AsRefMut<Queue<E>>,
{
    let q = c.queue_mut().as_mut();

    for p in &q.validate {
        invalidate::<E>(stor, p.refc()).expect("Lost Widget in invalidate");
    }
    q.validate.clear();
}
