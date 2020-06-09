use crate::simple::env::SimpleEnv;
use super::*;
use guion::{widget::{Widget, WidgetMut, WBaseMut}, widget::{resolved::{ResolvedMut, Resolved}, root::{resolve_in_root, resolve_in_root_mut}}, widget::cast::Statize, widget::resolvable::*};
use std::rc::Rc;
use ctx::SimpleCtx;
use sdl2::video::Window;

pub struct SimpleStor {
    pub roots: Vec<Box<dyn WidgetMut<'static,SimpleEnv>>>,
}

impl SimpleStor {
    pub fn new() -> Self {
        Self{
            roots: Vec::new()
        }
    }

    fn widion<'a>(&'a self, i: &StdID) -> Result<&dyn WidgetMut<'static,SimpleEnv>,()> {
        for w in &self.roots {
            if w.base().resolves_by(i) {
                return Ok(w);
            }
        }
        Err(())
    }
    /*fn widion_mut<'a>(&'a mut self, i: &StdID) -> Result<&mut dyn WidgetMut<'static,SimpleEnv>,()> {
        for w in &mut self.roots {
            if w.base().resolves_by(i) {
                return Ok(w);
            }
        }
        Err(())
    }*/
    pub fn path_for_root(&self, i: usize) -> StandardPath {
        StandardPath::new(&[self.roots[i].id()])
    }
}

impl GuionWidgets<SimpleEnv> for SimpleStor {
    fn widget<'a>(&'a self, i: StandardPath) -> Result<Resolved<'a,SimpleEnv>,()> {
        //let w = self.widion(i.index(0))?;
        if i.parent().is_none() {return Err(());}
        let w = WidgetHack::from_ref(self);
        let o = resolve_in_root(w.base(), i.slice(1..))?;
        Ok(Resolved{
            wref: o.0,
            path: o.1,
            stor: self,
        })
    }
    fn widget_mut<'a>(&'a mut self, i: StandardPath) -> Result<ResolvedMut<'a,SimpleEnv>,()> {
        //let w = self.widion_mut(i.index(0))?;
        if i.parent().is_none() {return Err(());}
        let w = WidgetHack::from_mut(self);
        let o = resolve_in_root_mut(w, i.slice(1..))?;
        Ok(ResolvedMut{
            wref: o.0,
            path: o.1,
        })
    }
    fn trace_bounds(&self, ctx: &mut SimpleCtx, i: StandardPath, b: &Bounds, force: bool) -> Result<Bounds,()> {
        let w = self.widion(i.index(0))?;
        let l = ctx.link(Resolved{
            wref: Box::new(w.base()),
            path: i.slice(..1),
            stor: self,
        });
        w.trace_bounds(l,i.slice(1..),b,force)
    }
}

#[repr(transparent)]
struct WidgetHack {
    inner: SimpleStor
}

impl WidgetHack {
    fn from_ref<'a>(s: &'a SimpleStor) -> &'a Self {
        unsafe{
            std::mem::transmute::<&'a SimpleStor,&'a Self>(s)
        }
    }
    fn from_mut<'a>(s: &'a mut SimpleStor) -> &'a mut Self {
        unsafe{
            std::mem::transmute::<&'a mut SimpleStor,&'a mut Self>(s)
        }
    }
}

impl<'w> Widget<'w,SimpleEnv> for WidgetHack {
    fn id(&self) -> StdID {
        panic!()
    }
    fn _render(&self, l: Link<SimpleEnv>, r: &mut RenderLink<SimpleEnv>) {
        panic!()
    }
    fn _event(&self, l: Link<SimpleEnv>, e: (EEvent<SimpleEnv>,&Bounds,u64)) {
        panic!()
    }
    fn _size(&self, l: Link<SimpleEnv>) -> ESize<SimpleEnv> {
        panic!()
    }
    fn childs(&self) -> usize {
        self.inner.roots.len()
    }
    fn child<'s>(&'s self, i: usize) -> Result<Resolvable<'s,SimpleEnv>,()> where 'w: 's {
        Ok(Resolvable::Widget(Box::new(self.inner.roots[i].base())))
    }
    fn into_child(self: Box<Self>, i: usize) -> Result<Resolvable<'w,SimpleEnv>,()> {
        panic!()
    }
    fn into_childs(self: Box<Self>) -> Vec<Resolvable<'w,SimpleEnv>> {
        panic!()
    }
    fn child_bounds(&self, l: Link<SimpleEnv>, b: &Bounds, force: bool) -> Result<Vec<Bounds>,()> {
        panic!()
    }
    fn focusable(&self) -> bool {
        panic!()
    }
}

impl<'w> WidgetMut<'w,SimpleEnv> for WidgetHack {
    fn child_mut<'s>(&'s mut self, i: usize) -> Result<ResolvableMut<'s,SimpleEnv>,()> where 'w: 's {
        Ok(ResolvableMut::Widget(Box::new(&mut (*self.inner.roots[i]))))
    }
    fn into_child_mut(self: Box<Self>, i: usize) -> Result<ResolvableMut<'w,SimpleEnv>,()> {
        todo!()
    }
    fn childs_mut<'s>(&'s mut self) -> Vec<ResolvableMut<'s,SimpleEnv>> where 'w: 's {
        todo!()
    }
    fn into_childs_mut(self: Box<Self>) -> Vec<ResolvableMut<'w,SimpleEnv>> {
        todo!()
    }
}

unsafe impl Statize<SimpleEnv> for WidgetHack {
    type Statur = WidgetHack;
}

