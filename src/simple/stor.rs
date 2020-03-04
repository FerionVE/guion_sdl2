use crate::simple::env::SimpleEnv;
use super::*;
use guion::{core::{path::WPSlice, widget::{immediate::{WidgetRefMut, WidgetRef}, Widget}, ctx::{ResolvedMut, Resolved, resolve_in_root, resolve_in_root_mut}}};
use std::rc::Rc;
use ctx::SimpleCtx;
use path::RcSimplePath;

pub struct SimpleStor {
    pub root: Box<dyn Widget<SimpleEnv>>,
}

impl SimpleStor {
    pub fn new(root: Box<dyn Widget<SimpleEnv>>) -> Self {
        Self{
            root
        }
    }
}

impl GuionWidgets<SimpleEnv> for SimpleStor {
    fn widget<'a>(&'a self, i: WPSlice<SimpleEnv>) -> Result<Resolved<'a,SimpleEnv>,()> {
        //eprintln!("{:?}",i.slice);
        resolve_in_root(&*self.root, i)
        .ok_or(())
        .map(|o: (Rc<WidgetRef<'a,SimpleEnv>>,EWPRc<SimpleEnv>)| Resolved{
            wref: o.0,
            path: o.1,
            stor: self,
        })
    }
    fn _widget_mut<'a>(&'a mut self, i: WPSlice<SimpleEnv>, invalidate: bool) -> Result<ResolvedMut<'a,SimpleEnv>,()> {
        resolve_in_root_mut(&mut *self.root, i, invalidate)
        .ok_or(())
        .map(|o: (WidgetRefMut<'a,SimpleEnv>,EWPRc<SimpleEnv>)| ResolvedMut{
            wref: o.0,
            path: o.1,
        })
    }
    fn trace_bounds(&self, ctx: &mut SimpleCtx, i: WPSlice<SimpleEnv>, b: &Bounds, force: bool) -> Result<Bounds,()> {
        let l = ctx.link(Resolved{
            wref: Rc::new(self.root.as_immediate()),
            path: RcSimplePath(Rc::new(vec![])),
            stor: self,
        });
        self.root.trace_bounds(l,i,b,force)
    }
}