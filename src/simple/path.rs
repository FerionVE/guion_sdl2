use super::*;
use env::{SimpleEnv, SimpleID};
use std::{any::{TypeId, Any}, rc::Rc};
use guion::core::path::WPSlice;
use guion::core::path::AsWPSlice;

impl GuionPath<SimpleEnv> for Vec<SimpleID> {
    type SubPath = SimpleID;
    type RcPath = RcSimplePath;
    fn attach(&mut self, sub: Self::SubPath) {
        self.push(sub)
    }
    fn attached(mut self, sub: Self::SubPath) -> Self {
        self.push(sub);
        self
    }
    fn id(&self) -> &SimpleID {
        self.tip()
    }
    fn tip(&self) -> &Self::SubPath {
        &self[self.len()-1]
    }
    fn parent(&self) -> Option<WPSlice<SimpleEnv>> {
        Self::parent_of_slice(self.slice())
    }
    fn id_of_slice(s: WPSlice<SimpleEnv>) -> &SimpleID {
        &s.slice[s.slice.len()-1]
    }
    fn parent_of_slice(s: WPSlice<SimpleEnv>) -> Option<WPSlice<SimpleEnv>> {
        if s.slice.len() <= 1 {return None;}
        Some(s.slice(0..s.slice.len()-1))
    }
    fn from_slice(s: WPSlice<SimpleEnv>) -> Self {
        s.slice.to_owned()
    }
}

impl AsWPSlice<SimpleEnv> for Vec<SimpleID> { //TODO this trait shouldn't exist anymore
    fn slice(&self) -> WPSlice<SimpleEnv> {
        WPSlice{slice: &self[..]}
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

#[repr(transparent)]
pub struct RcSimplePath(Rc<Vec<SimpleID>>);

impl RefClonable for RcSimplePath {
    fn refc(&self) -> Self {
        Self(self.0.refc())
    }
}

impl From<Vec<SimpleID>> for RcSimplePath {
    fn from(s: Vec<SimpleID>) -> Self {
        Self(Rc::new(s))
    }
}

impl Into<Vec<SimpleID>> for RcSimplePath {
    fn into(self) -> Vec<SimpleID> {
        (*self.0).clone() //TODO optimize
    }
}

impl Deref for RcSimplePath {
    type Target = Vec<SimpleID>;
    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}