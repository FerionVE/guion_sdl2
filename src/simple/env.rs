use guion::core::lazout::Size;
use crate::style::Style;
use crate::event::{key::Key, Event, destination::StdDest};
use crate::render::Render;
use guion::core::backend::Backend;
use guion::core::widget::Widget;
use guion::core::{id::WidgetID, env::Env};
use super::*;
use sdl2::video::Window;
use event::consuming::StdConsuming;
use context::Context;
use stor::SimpleStor;
use handler::Handler;
use valid::SimpleValidState;

pub struct SimpleEnv;
pub struct SimpleBackend;

impl Env for SimpleEnv {
    type Backend = SimpleBackend;
    type Context = Context<Self,EEEE>;
    type Storage = SimpleStor;
    ///regularly just dyn Widget
    type DynWidget = dyn Widget<Self>;
    type WidgetID = SimpleID;
    type WidgetPath = Vec<SimpleID>;
    type ValidState = SimpleValidState;
}

impl Backend<SimpleEnv> for SimpleBackend {
    type Renderer = Render<Window>;
    type Event = Event<Key,StdDest<SimpleDest>,StdConsuming>; //TODO ditch Consuming
    type Style = Style;
    type Size = Size;
}

type EEEE = Handler<(),SimpleEnv>;

#[derive(Clone,PartialEq,Hash)]
pub struct SimpleID {
    pub v: usize,
}

impl SimpleID {
    pub fn new() -> Self {
        todo!()
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