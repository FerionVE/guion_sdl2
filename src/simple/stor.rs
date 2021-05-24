use crate::simple::env::SimpleEnv;
use super::*;
use guion::{widget::{Widget, WidgetMut, cast::Statize}, widget::{resolved::{ResolvedMut, Resolved}, root::{resolve_in_root, resolve_in_root_mut}}, widget::resolvable::{ResolvableMut, Resolvable}, util::bounds::Dims, layout::StdGonstraints, layout::Gonstraints};
use ctx::SimpleCtx;
use guion::{EventResp, widget::WBaseMut};

pub struct SimpleStor {
    pub roots: Vec<(Box<dyn AsWidgetMut<SimpleEnv>+'static>,Dims)>,
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
        (*self.roots[i].0).as_ref().in_parent_path(SimplePath::new(&[]),true) //TODO empty default constructor for path
    }

    pub fn resolved(&self) -> Resolved<SimpleEnv> {
        Resolved{
            wref: Box::new(self as &dyn Widget<SimpleEnv>),
            path: StandardPath::empty(),
            direct_path: StandardPath::empty(),
            stor: self,
        }
    }
    pub fn resolved_mut(&mut self) -> ResolvedMut<SimpleEnv> {
        ResolvedMut{
            wref: Box::new(self as &mut dyn WidgetMut<SimpleEnv>),
            path: StandardPath::empty(),
            direct_path: StandardPath::empty(),
        }
    }
}

impl GWidgets<SimpleEnv> for SimpleStor {
    fn widget<'a>(&'a self, i: StandardPath) -> Result<Resolved<'a,SimpleEnv>,GuionError<SimpleEnv>> {
        resolve_in_root(self, i.refc(), i, self)
    }
    fn widget_mut<'a>(&'a mut self, i: StandardPath) -> Result<ResolvedMut<'a,SimpleEnv>,GuionError<SimpleEnv>> {
        resolve_in_root_mut(self, |s| s as &dyn Widget<_>, |s| s as &mut dyn WidgetMut<_>, i.refc(), i)
    }
    fn trace_bounds(&self, ctx: &mut SimpleCtx, i: StandardPath, b: &Bounds, e: &EStyle<SimpleEnv>, force: bool) -> Result<Bounds,GuionError<SimpleEnv>> {
        let l = ctx.link(Resolved{
            wref: Box::new(self.base()),
            path: StandardPath::empty(),
            direct_path: StandardPath::empty(),
            stor: self,
        });
        Widget::trace_bounds(self,l,i,b,e,force)
    }
}

//TODO protecc the fake-widget
/// fake-widget impl for real windows mapping
impl Widget<SimpleEnv> for SimpleStor {
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
    fn _size(&self, _: Link<SimpleEnv>, _: &EStyle<SimpleEnv>) -> ESize<SimpleEnv> {
        Gonstraints::empty()
    }
    fn childs(&self) -> usize {
        self.roots.len()
    }
    fn child(&self, i: usize) -> Result<Resolvable<SimpleEnv>,()> {
        self.roots.get(i)
            .map(|w| (*w.0).as_ref() )
            .ok_or(())
    }
    fn into_child<'w>(self: Box<Self>, i: usize) -> Result<Resolvable<'w,SimpleEnv>,()> where Self: 'w {
        todo!()
    }
    fn into_childs<'w>(self: Box<Self>) -> Vec<Resolvable<'w,SimpleEnv>> where Self: 'w {
        todo!()
    }
    fn child_bounds(&self, _: Link<SimpleEnv>, _: &Bounds, _: &EStyle<SimpleEnv>, _: bool) -> Result<Vec<Bounds>,()> {
        Ok(self.roots.iter()
            .map(|r| Bounds::from_size(r.1) )
            .collect::<Vec<_>>())
    }
    fn focusable(&self) -> bool {
        false
    }
}

impl WidgetMut<SimpleEnv> for SimpleStor {
    fn child_mut(&mut self, i: usize) -> Result<guion::widget::resolvable::ResolvableMut<SimpleEnv>,()> {
        self.roots.get_mut(i)
            .map(|w| (*w.0).as_mut() )
            .ok_or(())
    }
    fn into_child_mut<'w>(self: Box<Self>, i: usize) -> Result<guion::widget::resolvable::ResolvableMut<'w,SimpleEnv>,()> where Self: 'w {
        todo!()
    }
    fn childs_mut(&mut self) -> Vec<guion::widget::resolvable::ResolvableMut<SimpleEnv>> {
        todo!()
    }
    fn into_childs_mut<'w>(self: Box<Self>) -> Vec<guion::widget::resolvable::ResolvableMut<'w,SimpleEnv>> where Self: 'w {
        todo!()
    }
}

unsafe impl Statize<SimpleEnv> for SimpleStor {
    type Statur = Self;
}
