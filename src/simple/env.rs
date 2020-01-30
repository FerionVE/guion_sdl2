use crate::style::Style;
use crate::event::Event;
use crate::render::Render;
use guion::core::backend::Backend;
use crate::simple::ctx::SimpleContext;
use guion::core::widget::Widget;
use guion::core::env::Env;
use guion::macro_prelude::id::WidgetID;

pub struct SimpleEnv;
pub struct SimpleBackend;

impl Env for SimpleEnv {
    type Backend = SimpleBackend;
    type Context = SimpleContext;
    ///regularly just dyn Widget
    type DynWidget = dyn Widget<Self>;
    type WidgetID = SimpleID;
    type Commit = usize;
}

impl Backend<SimpleEnv> for SimpleBackend {
    type Renderer = Render;
    type Event = Event;
    type Style = Style;
}

#[derive(Clone,PartialEq,Hash)]
pub struct SimpleID {
    pub v: usize,
}

impl WidgetID for SimpleID {
    
}

