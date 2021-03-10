use crate::style::Style;
use crate::event::{key::Key, destination::StdDest};
use crate::render::Render;
use guion::{backend::Backend, layout::StdGonstraints};
use guion::{env::{Env}, event::{filter::StdFilter, standard::dyn_evt::DynEvent}, widget::{resolvable::{ResolvableMut, Resolvable}, as_widget::{AsWidgetMut, AsWidget}}};
use super::*;
use stor::SimpleStor;
use valid::SimpleValidState;
use ctx::SimpleCtx;
use std::{any::Any, fmt::Debug};

#[derive(Clone,PartialEq,Default)]
pub struct SimpleEnv;
#[derive(Clone,PartialEq,Default)]
pub struct SimpleBackend;

impl Env for SimpleEnv {
    type Backend = SimpleBackend;
    type Context = SimpleCtx;
    type Storage = SimpleStor;
    type WidgetID = StdID;
    type WidgetPath = StandardPath;
    type ValidState = SimpleValidState;
    type Message = Box<dyn Any>;
}

impl Backend<SimpleEnv> for SimpleBackend {
    type Renderer = Render<SimpleEnv>;
    type Event = DynEvent<SimpleEnv,Key,StdDest<SimpleDest>>; //TODO ditch Consuming
    type EventFilter = StdFilter<SimpleEnv>;
    type Style = Style;
    type Size = StdGonstraints;
}

//TODO move this to guion
#[derive(Clone)]
pub struct SimpleDest {
    pub v: usize,
}

impl GDestination for SimpleDest {
    const ROOT: Self = Self{v: 0};
    const FOCUSED: Self = Self{v: 1};
    const HOVERED: Self = Self{v: 2};
    const INVALID: Self = Self{v: std::usize::MAX};
}

//guion::impl_env_stds!(SimpleEnv);
//guion::impl_remote_state!(u8,SimpleEnv);

impl AsWidget<SimpleEnv> for StandardPath {
    #[inline]
    fn as_ref(&self) -> guion::widget::resolvable::Resolvable<SimpleEnv> {
        guion::widget::resolvable::Resolvable::Path(self.clone().into())
    }
    #[inline]
    fn into_ref<'w>(self) -> guion::widget::resolvable::Resolvable<'w,SimpleEnv> where Self: 'w {
        guion::widget::resolvable::Resolvable::Path(self.clone().into())
    }
}
impl AsWidgetMut<SimpleEnv> for StandardPath {
    #[inline]
    fn as_mut(&mut self) -> guion::widget::resolvable::ResolvableMut<SimpleEnv> {
        guion::widget::resolvable::ResolvableMut::Path(self.clone().into())
    }
    #[inline]
    fn into_mut<'w>(self) -> guion::widget::resolvable::ResolvableMut<'w,SimpleEnv> where Self: 'w {
        guion::widget::resolvable::ResolvableMut::Path(self.clone().into())
    }
}

/*impl AsWidget<SimpleEnv> for <SimpleEnv as Env>::WidgetPath {
    #[inline]
    fn as_ref(&self) -> guion::widget::resolvable::Resolvable<SimpleEnv> {
        guion::widget::resolvable::Resolvable::Path(self.clone().into())
    }
    #[inline]
    fn into_ref<'w>(self) -> guion::widget::resolvable::Resolvable<'w,SimpleEnv> where Self: 'w {
        guion::widget::resolvable::Resolvable::Path(self.clone().into())
    }
}
impl AsWidgetMut<SimpleEnv> for <SimpleEnv as Env>::WidgetPath {
    #[inline]
    fn as_mut(&mut self) -> guion::widget::resolvable::ResolvableMut<SimpleEnv> {
        guion::widget::resolvable::ResolvableMut::Path(self.clone().into())
    }
    #[inline]
    fn into_mut<'w>(self) -> guion::widget::resolvable::ResolvableMut<'w,SimpleEnv> where Self: 'w {
        guion::widget::resolvable::ResolvableMut::Path(self.clone().into())
    }
}*/

impl Debug for SimpleEnv {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
