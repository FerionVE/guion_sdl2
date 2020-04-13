use guion::layout::Size;
use crate::style::Style;
use crate::event::{key::Key, destination::StdDest};
use crate::render::Render;
use guion::backend::Backend;
use guion::{env::{EnvFlexStyleVariant, Env}, style::variant::standard::StdStyleVariant, event::dyn_evt::DynEvent};
use super::*;
use sdl2::video::Window;
use stor::SimpleStor;
use valid::SimpleValidState;
use ctx::SimpleCtx;

#[derive(Clone,PartialEq)]
pub struct SimpleEnv;
pub struct SimpleBackend;

impl Env for SimpleEnv {
    type Backend = SimpleBackend;
    type Context = SimpleCtx;
    type Storage = SimpleStor;
    type WidgetID = StdID;
    type WidgetPath = StandardPath;
    type ValidState = SimpleValidState;
}
impl EnvFlexStyleVariant for SimpleEnv {
    type StyleVariant = StdStyleVariant;
}

impl Backend<SimpleEnv> for SimpleBackend {
    type Renderer = Render<Window>;
    type Event = DynEvent<SimpleEnv,Key,StdDest<SimpleDest>>; //TODO ditch Consuming
    type Style = Style;
    type Size = Size;
}

//TODO move this to guion
#[derive(Clone)]
pub struct SimpleDest {
    pub v: usize,
}

impl GuionDestination for SimpleDest {
    const ROOT: Self = Self{v: 0};
    const FOCUSED: Self = Self{v: 1};
    const HOVERED: Self = Self{v: 2};
    const INVALID: Self = Self{v: std::usize::MAX};
}
