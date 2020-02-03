use crate::style::Style;
use crate::event::{key::Key, Event, destination::StdDest};
use crate::render::Render;
use guion::core::backend::Backend;
use crate::simple::ctx::SimpleContext;
use guion::core::widget::Widget;
use guion::core::env::Env;
use guion::macro_prelude::id::WidgetID;
use super::*;
use sdl2::video::Window;
use event::consuming::StdConsuming;

pub struct SimpleEnv;
pub struct SimpleBackend;

impl Env for SimpleEnv {
    type Backend = SimpleBackend;
    type Context = SimpleContext;
    ///regularly just dyn Widget
    type DynWidget = dyn Widget<Self>;
    type WidgetID = SimpleID;
}

impl Backend<SimpleEnv> for SimpleBackend {
    type Renderer = Render<Window>;
    type Event = Event<Key,StdDest<StandardDest>,StdConsuming>; //TODO ditch Consuming
    type Style = Style<SimpleEnv>;
}

#[derive(Clone,PartialEq,Hash)]
pub struct SimpleID {
    pub v: usize,
}

impl SimpleID {
    pub fn new() {
        todo!()
    }
}

impl WidgetID for SimpleID {
    
}

pub struct SimplePath {
    pub v: Vec<SimpleID>,
}

//TODO move this to guion
#[derive(Clone)]
pub struct StandardDest {
    pub v: usize,
}

impl GuionDestination for StandardDest {
    const ROOT: Self = Self{v: 0};
    const SELECTED: Self = Self{v: 1};
}
