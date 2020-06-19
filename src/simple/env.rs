use guion::layout::Size;
use crate::style::Style;
use crate::event::{key::Key, destination::StdDest};
use crate::render::Render;
use guion::backend::Backend;
use guion::{env::{EnvFlexStyleVariant, Env}, style::variant::standard::StdStyleVariant, event::{filter::StdFilter, dyn_evt::DynEvent}, widget::{resolvable::{ResolvableMut, Resolvable}, as_widget::{AsWidgetMut, AsWidget}}};
use super::*;
use stor::SimpleStor;
use valid::SimpleValidState;
use ctx::SimpleCtx;
use std::fmt::Debug;

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
    type Renderer = Render;
    type Event = DynEvent<SimpleEnv,Key,StdDest<SimpleDest>>; //TODO ditch Consuming
    type EventFilter = StdFilter<SimpleEnv>;
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

guion::impl_env_stds!(SimpleEnv);
//guion::impl_remote_state!(u8,SimpleEnv);

impl Debug for SimpleEnv {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
