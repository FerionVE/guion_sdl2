use crate::simple::env::SimpleEnv;
use super::*;
use guion::{widget::{Widget, WidgetMut, cast::Statize}, widget::{resolved::{ResolvedMut, Resolved}, root::{resolve_in_root, resolve_in_root_mut}}, widget::resolvable::{ResolvableMut, Resolvable}, util::bounds::Dims, layout::Size};
use ctx::SimpleCtx;
use guion::{EventResp, widget::WBaseMut};

pub struct SimpleStor {
    pub roots: Vec<(WidgetRefMut<'static,SimpleEnv>,Dims)>,
    pub _id: StdID,
}

impl SimpleStor {
    pub fn new() -> Self {
        Self{
            roots: Vec::new(),
            _id: StdID::new(),
        }
    }

    pub fn path_for_root(&self, i: usize) -> StandardPath {
        StandardPath::new(&[self.roots[i].0.id()])
    }

    pub fn resolved(&self) -> Resolved<SimpleEnv> {
        Resolved{
            wref: Box::new(self as &dyn Widget<SimpleEnv>),
            path: StandardPath::empty(),
            stor: self,
        }
    }
    pub fn resolved_mut(&mut self) -> ResolvedMut<SimpleEnv> {
        ResolvedMut{
            wref: Box::new(self as &mut dyn WidgetMut<SimpleEnv>),
            path: StandardPath::empty(),
        }
    }
}

impl GuionWidgets<SimpleEnv> for SimpleStor {
    fn widget<'a>(&'a self, i: StandardPath) -> Result<Resolved<'a,SimpleEnv>,()> {
        let (wref,path) = resolve_in_root(self, i)?;
        Ok(Resolved{
            wref,
            path,
            stor: self,
        })
    }
    fn widget_mut<'a>(&'a mut self, i: StandardPath) -> Result<ResolvedMut<'a,SimpleEnv>,()> {
        let (wref,path) = resolve_in_root_mut(self, i)?;
        Ok(ResolvedMut{
            wref,
            path,
        })
    }
    fn trace_bounds(&self, ctx: &mut SimpleCtx, i: StandardPath, b: &Bounds, e: &ESVariant<SimpleEnv>, force: bool) -> Result<Bounds,()> {
        let l = ctx.link(Resolved{
            wref: Box::new(self.base()),
            path: StandardPath::new(&[]),
            stor: self,
        });
        Widget::trace_bounds(self,l,i,b,e,force)
    }
}

//TODO protecc the fake-widget
/// fake-widget impl for real windows mapping
impl<'w> Widget<'w,SimpleEnv> for SimpleStor {
    fn id(&self) -> StdID {
        self._id
    }
    fn _render(&self, l: Link<SimpleEnv>, r: &mut RenderLink<SimpleEnv>) {
        todo!()
    }
    fn _event_direct(&self, mut l: Link<SimpleEnv>, e: &EventCompound<SimpleEnv>) -> EventResp {
        let mut passed = false;

        for i in 0..self.childs() {
            let mut l = l.for_child(i).expect("Dead Path Inside Pane");
            let sliced = e.with_bounds( Bounds::from_size(self.roots[i].1) );
            if let Some(ee) = sliced.filter(&l) {
                passed |= l.event_direct(&ee);
            }
        }
        //eprintln!("e{}",passed);
        passed
    }
    fn _size(&self, _: Link<SimpleEnv>, _: &ESVariant<SimpleEnv>) -> ESize<SimpleEnv> {
        Size::empty()
    }
    fn childs(&self) -> usize {
        self.roots.len()
    }
    fn child<'s>(&'s self, i: usize) -> Result<Resolvable<'s,SimpleEnv>,()> where 'w: 's {
        self.roots.get(i)
            .map(|w| Resolvable::Widget(Box::new((*(w.0)).base())) )
            .ok_or(())
    }
    fn into_child(self: Box<Self>, i: usize) -> Result<Resolvable<'w,SimpleEnv>,()> {
        todo!()
    }
    fn into_childs(self: Box<Self>) -> Vec<Resolvable<'w,SimpleEnv>> {
        todo!()
    }
    fn child_bounds(&self, _: Link<SimpleEnv>, _: &Bounds, _: &ESVariant<SimpleEnv>, _: bool) -> Result<Vec<Bounds>,()> {
        Ok(self.roots.iter()
            .map(|r| Bounds::from_size(r.1) )
            .collect::<Vec<_>>())
    }
    fn focusable(&self) -> bool {
        false
    }
}

impl<'w> WidgetMut<'w,SimpleEnv> for SimpleStor {
    fn child_mut<'s>(&'s mut self, i: usize) -> Result<guion::widget::resolvable::ResolvableMut<'s,SimpleEnv>,()> where 'w: 's {
        self.roots.get_mut(i)
            .map(|w| ResolvableMut::Widget(Box::new(&mut *(w.0))) )
            .ok_or(())
    }
    fn into_child_mut(self: Box<Self>, i: usize) -> Result<guion::widget::resolvable::ResolvableMut<'w,SimpleEnv>,()> {
        todo!()
    }
    fn childs_mut<'s>(&'s mut self) -> Vec<guion::widget::resolvable::ResolvableMut<'s,SimpleEnv>> where 'w: 's {
        todo!()
    }
    fn into_childs_mut(self: Box<Self>) -> Vec<guion::widget::resolvable::ResolvableMut<'w,SimpleEnv>> {
        todo!()
    }
}

unsafe impl Statize<SimpleEnv> for SimpleStor {
    type Statur = Self;
}
