use std::fmt::Debug;
use std::marker::PhantomData;
use std::ops::AddAssign;

use guion::aliases::ESColor;
use guion::env::Env;
use guion::style::selector::{StyleSelector, StyleSelectorAppend};
use guion::style::selectag::standard::StdSelectag;
use guion::util::border::Border;
use guion::style::selectag::StyleSelectag;

#[non_exhaustive]
#[derive(Clone,Default)]
pub struct Selector<E> where E: Env {
    pub obj: Option<Obj>,
    pub design: Option<Design>,
    pub accent: Option<u32>,
    pub variance: Option<Variance>,
    pub hovered: Option<bool>,
    pub focused: Option<bool>,
    pub pressed: Option<bool>,
    pub locked: Option<bool>,
    //pub cursor: Option<StdCursor>, TODO
    pub border: Option<BorderPtr>,
    pub _p: PhantomData<E>, //TODO fix lazyness
}

pub struct SelectorFilled<E> where E: Env {
    pub obj: Obj,
    pub design: Design,
    pub accent: u32,
    pub variance: Variance,
    pub hovered: bool,
    pub focused: bool,
    pub pressed: bool,
    pub locked: bool,
    //pub cursor: StdCursor, TODO
    pub border: BorderPtr,
    pub _p: PhantomData<E>, //TODO fix lazyness
}

#[non_exhaustive]
#[derive(Clone,Copy,PartialEq)]
pub enum Obj {
    Default,
    Background,
    Foreground,
    Text,
    Box,
    Active,
    Border,
    Button,
    List,
    TextBox,
    Label,
    Scroll,
}

#[non_exhaustive]
#[derive(Clone,Copy,PartialEq)]
pub enum Design {
    Default,
    Normal,
    Flat,
}

#[non_exhaustive]
#[derive(Clone,Copy,PartialEq)]
pub enum Variance {
    Default,
    Normal,
    OK,
    Caution,
    Secondary,
}

#[non_exhaustive]
#[derive(Clone,Copy,PartialEq)]
pub enum StdCursor {
    Default,
    Arrow,
    IBeam,
    Wait,
    Crosshair,
    WaitArrow,
    SizeNWSE,
    SizeNESW,
    SizeWE,
    SizeNS,
    SizeAll,
    No,
    Hand,
}

#[non_exhaustive]
#[derive(Clone,Copy,PartialEq)]
pub enum BorderPtr {
    Default,
    Outer,
    Visual,
}

/*impl<E> Default for Selector<E> where E: Env {
    #[inline]
    fn default() -> Self {
        Self{
            obj: Obj::Default,
            design: Design::Default,
            accent: 0,
            variance: Variance::Default,
            hovered: false,
            focused: false,
            pressed: false,
            locked: false,
            //cursor: StdCursor::Default,
            border: BorderPtr::Default,
            _p: PhantomData
        }
    }
}*/

impl<E> Selector<E> where E: Env {
    pub fn filled(&self) -> SelectorFilled<E> {
        let s = self.clone();
        SelectorFilled {
            obj: s.obj.unwrap_or(Obj::Default),
            design: s.design.unwrap_or(Design::Default),
            accent: s.accent.unwrap_or(0),
            variance: s.variance.unwrap_or(Variance::Default),
            hovered: s.hovered.unwrap_or(false),
            focused: s.focused.unwrap_or(false),
            pressed: s.pressed.unwrap_or(false),
            locked: s.locked.unwrap_or(false),
            border: s.border.unwrap_or(BorderPtr::Default),
            _p: PhantomData,
        }
    }
}

impl<E> StyleSelector<E> for Selector<E> where E: Env {
    fn and(&self, s: &Self) -> Self {
        Self{
            obj: s.obj.or(self.obj).clone(),
            design: s.design.or(self.design).clone(),
            accent: s.accent.or(self.accent).clone(),
            variance: s.variance.or(self.variance).clone(),
            hovered: s.hovered.or(self.hovered).clone(),
            focused: s.focused.or(self.focused).clone(),
            pressed: s.pressed.or(self.pressed).clone(),
            locked: s.locked.or(self.locked).clone(),
            border: s.border.or(self.border).clone(),
            _p: PhantomData,
        }
    }
}

impl<E> StyleSelectorAppend<StdSelectag<E>,E> for Selector<E> where E: Env {
    #[inline]
    fn append(&mut self, v: StdSelectag<E>) {
        match v {
            StdSelectag::ObjDefault => self.obj = Some(Obj::Default),
            StdSelectag::ObjBackground => self.obj = Some(Obj::Background),
            StdSelectag::ObjForeground => self.obj = Some(Obj::Foreground),
            StdSelectag::ObjText => self.obj = Some(Obj::Text),
            StdSelectag::ObjBox => self.obj = Some(Obj::Box),
            StdSelectag::ObjBorder => self.obj = Some(Obj::Border),
            StdSelectag::ObjActive => self.obj = Some(Obj::Active),
            StdSelectag::ObjButton => self.obj = Some(Obj::Button),
            StdSelectag::ObjList => self.obj = Some(Obj::List),
            StdSelectag::ObjTextBox => self.obj = Some(Obj::TextBox),
            StdSelectag::ObjLabel => self.obj = Some(Obj::Label),
            StdSelectag::ObjScroll => self.obj = Some(Obj::Scroll),
            StdSelectag::DesignDefault => self.design = Some(Design::Default),
            StdSelectag::DesignNormal => self.design = Some(Design::Normal),
            StdSelectag::DesignFlat => self.design = Some(Design::Flat),
            StdSelectag::Accent(v) => self.accent = Some(v),
            StdSelectag::VariantDefault => self.variance = Some(Variance::Default),
            StdSelectag::VariantNormal => self.variance = Some(Variance::Normal),
            StdSelectag::VariantOK => self.variance = Some(Variance::OK),
            StdSelectag::VariantCaution => self.variance = Some(Variance::Caution),
            StdSelectag::VariantSecondary => self.variance = Some(Variance::Secondary),
            StdSelectag::Hovered(v) => self.hovered = Some(v),
            StdSelectag::Focused(v) => self.focused = Some(v),
            StdSelectag::Pressed(v) => self.pressed = Some(v),
            StdSelectag::Locked(v) => self.locked = Some(v),
            StdSelectag::BorderDefault => self.border = Some(BorderPtr::Default),
            StdSelectag::BorderOuter => self.border = Some(BorderPtr::Outer),
            StdSelectag::BorderVisual => self.border = Some(BorderPtr::Visual),
            _ => {},
        }        
    }
}

impl<E,T> AddAssign<T> for Selector<E> where Self: StyleSelectorAppend<T,E>, T: StyleSelectag<E>, E: Env {
    #[inline]
    fn add_assign(&mut self, v: T) {
        self.append(v)
    }
}

/*impl<E,T> StyleSelectorAppend<&[T]> for Selector<E> where Self: StyleSelectorAppend<T>, T: Clone, E: Env {
    #[inline]
    fn attach(&mut self, selectors: &[T]) {
        for t in selectors {
            self.attach(t.clone());
        }
    }
}

impl<E> StyleSelectorAppend<()> for Selector<E> where E: Env {
    #[inline]
    fn attach(&mut self, _: ()) {}
}
impl<E> StyleSelectorAppend<&()> for Selector<E> where E: Env {
    #[inline]
    fn attach(&mut self, _: &()) {}
}

impl<E> StyleSelectorGetStdCursor for Selector<E> where E: Env {
    #[inline]
    fn cursor(&self) -> StdCursor {
        self.cursor
    }
}*/

/*impl<E> Clone for Selector<E> where E: Env {
    fn clone(&self) -> Self {
        Self{
            obj: self.obj.clone(),
            design: self.design.clone(),
            accent: self.accent.clone(),
            variance: self.variance.clone(),
            hovered: self.hovered.clone(),
            focused: self.focused.clone(),
            pressed: self.pressed.clone(),
            locked: self.locked.clone(),
            cursor: self.cursor.clone(),
            border_ptr: self.border_ptr.clone(),
            border_mul: self.border_mul.clone(),
            color_specific: self.color_specific.clone(),
        }
    }
}*/
