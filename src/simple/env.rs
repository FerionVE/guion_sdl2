use std::sync::atomic::AtomicUsize;
use guion::core::lazout::Size;
use crate::style::Style;
use crate::event::{key::Key, Event, destination::StdDest};
use crate::render::Render;
use guion::core::backend::Backend;
use guion::core::widget::Widget;
use guion::core::{id::WidgetID, env::{EnvFlexStyleVariant, Env}, style::standard::StdStyleVariant};
use super::*;
use sdl2::video::Window;
use event::consuming::StdConsuming;
use stor::SimpleStor;
use handler::Handler;
use valid::SimpleValidState;
use std::sync::atomic::Ordering;
use ctx::SimpleCtx;

pub struct SimpleEnv;
pub struct SimpleBackend;

impl Env for SimpleEnv {
    type Backend = SimpleBackend;
    type Context = SimpleCtx;
    type Storage = SimpleStor;
    ///regularly just dyn Widget
    type DynWidget = dyn Widget<Self>;
    type WidgetID = SimpleID;
    type WidgetPath = Vec<SimpleID>;
    type ValidState = SimpleValidState;
}
impl EnvFlexStyleVariant for SimpleEnv {
    type StyleVariant = StdStyleVariant;
}

impl Backend<SimpleEnv> for SimpleBackend {
    type Renderer = Render<Window>;
    type Event = Event<Key,StdDest<SimpleDest>,StdConsuming>; //TODO ditch Consuming
    type Style = Style;
    type Size = Size;
}

type EEEE = Handler<(),SimpleEnv>;

static ID_ITER: AtomicUsize = AtomicUsize::new(0);

#[derive(Clone,PartialEq,Hash,Debug)]
pub struct SimpleID {
    pub v: usize, //TODO protect
}

impl SimpleID {
    pub fn new() -> Self {
        Self{
            v: ID_ITER.fetch_add(1,Ordering::Relaxed)
        }
    }
}

impl WidgetID for SimpleID {
    
}

//TODO move this to guion
#[derive(Clone)]
pub struct SimpleDest {
    pub v: usize,
}

impl GuionDestination for SimpleDest {
    const ROOT: Self = Self{v: 0};
    const SELECTED: Self = Self{v: 1};
}