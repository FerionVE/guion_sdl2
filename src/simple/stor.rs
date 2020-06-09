use crate::simple::env::SimpleEnv;
use super::*;
use guion::{widget::{Widget, WidgetMut}, widget::{resolved::{ResolvedMut, Resolved}, root::{resolve_in_root, resolve_in_root_mut}}, widget::resolvable::Resolvable};
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
    fn widion_mut<'a>(&'a mut self, i: &StdID) -> Result<&mut dyn WidgetMut<'static,SimpleEnv>,()> {
        for w in &mut self.roots {
            if w.base().resolves_by(i) {
                return Ok(w);
            }
        }
        Err(())
    }
    pub fn path_for_root(&self, i: usize) -> StandardPath {
        StandardPath::new(&[self.roots[i].id()])
    }
}

impl GuionWidgets<SimpleEnv> for SimpleStor {
    fn widget<'a>(&'a self, i: StandardPath) -> Result<Resolved<'a,SimpleEnv>,()> {
        if i.parent().is_none() {return Err(());}
        let w = self.widion(i.index(0))?;
        let r = w.resolve(i.slice(1..))?;
        match r {
            Resolvable::Widget(w) => 
                Ok(Resolved{
                    wref: w,
                    path: i,
                    stor: self,
                }),
            Resolvable::Path(p) => self.widget(p),
        }
    }
    fn widget_mut<'a>(&'a mut self, i: StandardPath) -> Result<ResolvedMut<'a,SimpleEnv>,()> {
        let final_path = self.widget(i).map(|r| r.path )?;

        let w = self.widion_mut(final_path.index(0))?;
        let o = resolve_in_root_mut(w, final_path.slice(1..))?;
        Ok(ResolvedMut{
            wref: o.0,
            path: o.1,
        })
    }
    fn trace_bounds(&self, ctx: &mut SimpleCtx, i: StandardPath, b: &Bounds, force: bool) -> Result<Bounds,()> {
        let w = self.widget(i.slice(..1))?;
        let mut w = ctx.link(w);
        w._trace_bounds(i.slice(1..),b,force)
    }
}
