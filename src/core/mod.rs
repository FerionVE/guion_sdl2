use super::*;
use guion::{ctx::queue::{invalidate, validate, StdEnqueueable}, render::link::RenderLink};
use render::font::TextProd;
use sdl2::EventPump;
use sdl2::EventSubsystem;
use sdl2::TimerSubsystem;
use sdl2::VideoSubsystem;
use sdl2::{ttf::Sdl2TtfContext, Sdl};
use std::collections::VecDeque;

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
    pub validate_render: Vec<E::WidgetPath>,
    pub validate_size: Vec<(E::WidgetPath,ESize<E>)>,
    pub invalidate: Vec<E::WidgetPath>,
    pub mut_fn: VecDeque<StdEnqueueable<E>>,
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
            invalidate: Vec::with_capacity(4096),
            validate_render: Vec::with_capacity(4096),
            validate_size: Vec::with_capacity(4096),
            mut_fn: VecDeque::with_capacity(4096),
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
    

    /*while let Some((p, f, i)) = c.queue_mut().as_mut().mut_fn.pop_front() {
        let w = stor._widget_mut(p.refc(), i).expect("TODO");
        f(w.wref,c);
    };*/

    while let Some(e) = c.queue_mut().as_mut().mut_fn.pop_front() {
        match e {
            StdEnqueueable::InvalidateWidget { path } => unreachable!(),
            StdEnqueueable::ValidateWidgetRender { path } => unreachable!(),
            StdEnqueueable::ValidateWidgetSize { path, size } => unreachable!(),
            StdEnqueueable::Render { force } => (),
            StdEnqueueable::Event { event, ts } => todo!(),
            StdEnqueueable::MutateWidget { path, f, invalidate } => {
                let w = stor._widget_mut(path.clone(), invalidate).expect("TODO");
                f(w.wref,c,path);
            },
            StdEnqueueable::MutateWidgetClosure { path, f, invalidate } => {
                let w = stor._widget_mut(path.clone(), invalidate).expect("TODO");
                f(w.wref,c,path);
            },
            StdEnqueueable::MutateRoot { f } => {
                f(stor,c)
            },
            StdEnqueueable::MutateRootClosure { f } => {
                f(stor,c)
            },
            StdEnqueueable::AccessWidget { path, f } => todo!(),
            StdEnqueueable::AccessWidgetClosure { path, f } => todo!(),
            StdEnqueueable::AccessRoot { f } => todo!(),
            StdEnqueueable::AccessRootClosure { f } => todo!(),
        }
    }

    let q = c.queue_mut().as_mut();

    for p in &q.invalidate {
        invalidate::<E>(stor, p.clone()).expect("Lost Widget in invalidate");
    }
    q.invalidate.clear();
}

pub fn post_render_events<E>(stor: &mut E::Storage, c: &mut E::Context)
where
    E: Env,
    ECQueue<E>: AsRefMut<Queue<E>>,
{
    let q = c.queue_mut().as_mut();

    /*for p in &q.validate_size {
        validate::<E>(stor, p.refc()).expect("Lost Widget in invalidate");
    }
    q.validate_size.clear();*/
    for p in &q.validate_render {
        validate::<E>(stor, p.clone()).expect("Lost Widget in invalidate");
    }
    q.validate_render.clear();
}
