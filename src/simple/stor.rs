use crate::simple::env::SimpleEnv;
use super::*;
use guion::{core::{path::WPSlice, widget::{immediate::{WidgetRefMut, WidgetRef}, Widget}, ctx::{ResolvedMut, Resolved, resolve_in_root, resolve_in_root_mut}}};
use std::rc::Rc;

pub struct SimpleStor {
    pub root: Box<dyn Widget<SimpleEnv>>,
}

impl GuionWidgets<SimpleEnv> for SimpleStor {
    fn widget<'a>(&'a self, i: WPSlice<SimpleEnv>) -> Result<Resolved<'a,SimpleEnv>,()> {
        resolve_in_root(&*self.root, i)
        .ok_or(())
        .map(|o: (Rc<WidgetRef<'a,SimpleEnv>>,EWPRc<SimpleEnv>)| Resolved{
            wref: o.0,
            path: o.1,
            stor: self,
        })
    }
    fn widget_mut<'a>(&'a mut self, i: WPSlice<SimpleEnv>) -> Result<ResolvedMut<'a,SimpleEnv>,()> {
        resolve_in_root_mut(&mut *self.root, i)
        .ok_or(())
        .map(|o: (WidgetRefMut<'a,SimpleEnv>,EWPRc<SimpleEnv>)| ResolvedMut{
            wref: o.0,
            path: o.1,
        })
    }
}