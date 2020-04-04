use super::*;
use env::{SimpleEnv, SimpleID};
use std::{any::{TypeId, Any}, rc::Rc, sync::Arc, ops::{RangeBounds, Range}, slice::SliceIndex};
use guion::core::{widget::as_widget::*, ctx::*};
use qwutils::RefClonable;

#[derive(PartialEq,Clone)]
pub struct SimplePath {
    pub v: Arc<Vec<SimpleID>>,
    pub slice: Range<usize>,
}

impl GuionPath<SimpleEnv> for SimplePath {
    type SubPath = SimpleID;
    fn attach(&mut self, sub: Self::SubPath) {
        Arc::make_mut(&mut self.v).push(sub);
        self.slice.end += 1;
    }
    fn attached(mut self, sub: Self::SubPath) -> Self { //TODO can be default impl
        self.attach(sub);
        self
    }
    fn attach_subpath(&mut self, sub: &Self) {
        let senf = Arc::make_mut(&mut self.v);
        senf.extend_from_slice(sub._get());
        self.slice.end += sub.slice.len();
    }
    fn id(&self) -> SimpleID {
        self.tip().clone()
    }
    fn tip(&self) -> &Self::SubPath {
        &self.v[self.slice.end-1]
    }
    fn parent(&self) -> Option<Self> {
        if self.is_empty() {return None;}
        Some(Self{
            v: self.v.refc(),
            slice: self.slice.start .. self.slice.end-1,
        })
    }
    fn is_empty(&self) -> bool {
        self.slice.len() == 0
    }
    fn slice<T>(&self, range: T) -> Self where T: RangeBounds<usize> {
        Self{
            v: self.v.refc(),
            slice: slice_range(&self.slice,range),
        }
    }
    fn index<T>(&self, i: T) -> &Self::SubPath where T: SliceIndex<[Self::SubPath],Output=Self::SubPath> {
        &self._get()[i]
    }
}

impl SimplePath {
    pub fn new(range: &[SimpleID], root_id: SimpleID) -> Self {
        let mut dest = Vec::with_capacity(range.len()+1);
        dest.push(root_id);
        dest.extend_from_slice(range);
        Self{
            slice: 1..dest.len(),
            v: Arc::new(dest),
        }
    }
    fn _get(&self) -> &[SimpleID] {
        &self.v[self.slice.clone()]
    }
    fn _root(&self) -> &SimpleID {
        &self.v[0]
    }
}

impl RefClonable for SimplePath {
    fn refc(&self) -> Self {
        self.clone()
    }
}

impl GuionSubPath<SimpleEnv> for SimpleID {
    fn from_id(id: SimpleID) -> Self {
        id
    }
    fn eq_id(&self, id: SimpleID) -> bool {
        self == &id
    }
    fn into_id(self) -> SimpleID {
        self
    }
    fn is<T: Any>(&self) -> bool { //TODO default underlying-trait impl hack
        Any::is::<T>(self)
    }
    fn downcast_ref<T: Any>(&self) -> Option<&T> {
        Any::downcast_ref::<T>(self)
    }
    fn downcast_mut<T: Any>(&mut self) -> Option<&mut T> {
        Any::downcast_mut::<T>(self)
    }
    fn downcast_into<T: Any>(self) -> Result<T,Self> where Self: Sized + 'static {
        todo!()
    }
}

//TODO move as macro to guion
impl AsWidget<'static,SimpleEnv> for SimplePath {
    fn as_ref<'s>(&'s self) -> Resolvable<'s,SimpleEnv> where 'static: 's {
        Resolvable::Path(self.clone().into())
    }
    fn into_ref(self) -> Resolvable<'static,SimpleEnv> {
        Resolvable::Path(self.clone().into())
    }
}
impl AsWidgetMut<'static,SimpleEnv> for SimplePath {
    fn as_mut<'s>(&'s mut self) -> ResolvableMut<'s,SimpleEnv> where 'static: 's {
        ResolvableMut::Path(self.clone().into())
    }
    fn into_mut(self) -> ResolvableMut<'static,SimpleEnv> {
        ResolvableMut::Path(self.clone().into())
    }
}

fn slice_range<S>(range: &Range<usize>, slice: S) -> Range<usize> where S: RangeBounds<usize> {
    let (os,oe) = (range.start,range.end);
    let (mut s,mut e) = (os,oe);
    match range.end_bound() {
        std::ops::Bound::Included(b) => e = oe.min(b-1+os),
        std::ops::Bound::Excluded(b) => e = oe.min(b+os),
        std::ops::Bound::Unbounded => (),
    }
    match range.start_bound() {
        std::ops::Bound::Included(b) => s = os.max(b+os),
        std::ops::Bound::Excluded(b) => s = os.max(b-1+os),
        std::ops::Bound::Unbounded => (),
    }
    s..e
}