use crate::simple::env::SimpleEnv;
use super::*;
use guion::{widget::{Widget, WidgetMut}, widget::{resolved::{ResolvedMut, Resolved}, root::{resolve_in_root, resolve_in_root_mut}}};
use std::rc::Rc;
use ctx::SimpleCtx;

pub struct SimpleStor {
    pub root: Box<dyn WidgetMut<'static,SimpleEnv>>,
}

impl SimpleStor {
    pub fn new(root: Box<dyn WidgetMut<'static,SimpleEnv>>) -> Self {
        Self{
            root
        }
    }
}

impl GuionWidgets<SimpleEnv> for SimpleStor {
    fn widget<'a>(&'a self, i: StandardPath) -> Result<Resolved<'a,SimpleEnv>,()> {
        //eprintln!("{:?}",i.slice);
       let o = resolve_in_root(self.root.base(), i)?;
        Ok(Resolved{
            wref: o.0,
            path: o.1,
            stor: self,
        })
    }
    fn widget_mut<'a>(&'a mut self, i: StandardPath) -> Result<ResolvedMut<'a,SimpleEnv>,()> {
        let o = resolve_in_root_mut(&mut *self.root, i)?;
        Ok(ResolvedMut{
            wref: o.0,
            path: o.1,
        })
    }
    fn trace_bounds(&self, ctx: &mut SimpleCtx, i: StandardPath, b: &Bounds, force: bool) -> Result<Bounds,()> {
        let l = ctx.link(Resolved{
            wref: Box::new(self.root.base()),
            path: StandardPath::new(&[]),
            stor: self,
        });
        self.root.trace_bounds(l,i,b,force)
    }
}