use std::sync::atomic::AtomicUsize;
use guion::core::layout::Size;
use crate::style::Style;
use crate::event::{key::Key, destination::StdDest};
use crate::render::Render;
use guion::core::backend::Backend;
use guion::core::{id::WidgetID, env::{EnvFlexStyleVariant, Env}, style::standard::StdStyleVariant, event::dyn_evt::DynEvent};
use super::*;
use sdl2::video::Window;
use stor::SimpleStor;
use valid::SimpleValidState;
use std::{any::TypeId, sync::atomic::Ordering};
use ctx::SimpleCtx;
use path::SimplePath;

#[derive(Clone)]
pub struct SimpleEnv;
pub struct SimpleBackend;

impl Env for SimpleEnv {
    type Backend = SimpleBackend;
    type Context = SimpleCtx;
    type Storage = SimpleStor;
    type WidgetID = SimpleID;
    type WidgetPath = SimplePath;
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

static ID_ITER: AtomicUsize = AtomicUsize::new(0);

#[derive(Clone,PartialEq,Hash,Debug)]
pub enum SimpleID {
    Dyn(usize),
    Const(TypeId),
}

impl SimpleID {
    pub fn new() -> Self {
        SimpleID::Dyn(ID_ITER.fetch_add(1,Ordering::Relaxed))
    }
}

#[macro_export]
macro_rules! const_id {
    () => {
        {
            struct Ident;
            $crate::simple::env::SimpleID::Const(std::any::TypeId::of::<Ident>())
        }
    };
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
    const FOCUSED: Self = Self{v: 1};
    const HOVERED: Self = Self{v: 2};
    const INVALID: Self = Self{v: std::usize::MAX};
}
