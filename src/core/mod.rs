use super::*;
use guion::{ctx::queue::{invalidate, validate, StdEnqueueable, StdOrder}, render::link::RenderLink};
use sdl2::EventPump;
use sdl2::EventSubsystem;
use sdl2::TimerSubsystem;
use sdl2::VideoSubsystem;
use sdl2::{Sdl, clipboard::ClipboardUtil};
use std::collections::{HashMap, VecDeque};
use rusttype::Font;
use render::font::load_font;

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
    pub clipboard: ClipboardUtil,
    pub pump: EventPump,
    pub queue: Queue<E>,
    pub default_border: Border,
    pub default_style: EStyle<E>,
    pub font: Font<'static>,
}

pub struct Queue<E>
where
    E: Env,
{
    pub event: EventSubsystem,
    pub timer: TimerSubsystem,
    pub queues: HashMap<StdOrder,Vec<(StdEnqueueable<E>,i64)>>,
    pub force_render: bool,
}

impl<E> Core<E>
where
    E: Env + Sync,
{
    pub fn from_sdl2(sdl: Sdl) -> Result<Self, String> {
        let event = sdl.event()?;
        let pump = sdl.event_pump()?;
        let timer = sdl.timer()?;
        let queue = Queue {
            event: event.clone(),
            timer: timer.clone(),
            queues: HashMap::new(),
            force_render: true,
        };
        let video = sdl.video()?;
        let clipboard = video.clipboard();

        let default_border = Border::new(4, 4, 4, 4);
        let default_style = EStyle::<E>::static_default();

        Ok(Self {
            event,
            pump,
            timer,
            queue,
            video,
            clipboard,
            sdl,
            default_border,
            default_style,
            font: load_font(),
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

pub fn process_events<E>(stor: &mut E::Storage, c: &mut E::Context, pass: StdOrder)
where
    E: Env,
    ECQueue<E>: AsRefMut<Queue<E>>,
{
    if let Some(mut queue) = c.queue_mut().as_mut().queues.remove(&pass) {
        queue.sort_by_key(|(_,p)| *p );

        for (e,_) in queue {
            match e {
                StdEnqueueable::InvalidateWidget { path } => {
                    invalidate::<E>(stor, path.clone()).expect("Lost Widget in invalidate");
                },
                StdEnqueueable::ValidateWidgetRender { path } => {
                    validate::<E>(stor, path.clone()).expect("Lost Widget in invalidate");
                },
                StdEnqueueable::ValidateWidgetSize { path, size } => todo!(),
                StdEnqueueable::Render { force } => {
                    c.queue_mut().as_mut().force_render |= force;
                },
                StdEnqueueable::Event { event, ts } => todo!(),
                StdEnqueueable::MutateWidget { path, f } => {
                    let w = stor.widget_mut(path.clone()).expect("TODO");
                    f(w.wref,c,path);
                },
                StdEnqueueable::MutateWidgetClosure { path, f } => {
                    let w = stor.widget_mut(path.clone()).expect("TODO");
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
    }
}
/// render widget and process validation
pub fn render_and_events<E>(r: &mut RenderLink<E>, w: E::WidgetPath, stor: &mut E::Storage, c: &mut E::Context)
where
    E: Env,
    ECQueue<E>: AsRefMut<Queue<E>>,
{
    process_events::<E>(stor, c, StdOrder::PreRender);
    r.force |= c.queue().as_ref().force_render;
    let w = c.link(stor.widget(w).expect("Lost Widget in render"));
    r.render_widget(w);
    c.queue_mut().as_mut().force_render = false;
    process_events::<E>(stor, c, StdOrder::RenderValidation);
    process_events::<E>(stor, c, StdOrder::PostCurrent);
    process_events::<E>(stor, c, StdOrder::PostRender);
}
