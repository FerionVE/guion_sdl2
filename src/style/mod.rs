use crate::style::color::Color;
use crate::style::font::Font;
use crate::style::font::Glyphs;
use guion::style::standard::cursor::StdCursor;
use self::selector::*;

use super::*;
use std::ops::Mul;

pub mod font;
pub mod cursor;
pub mod default;
pub mod color;
pub mod selector;

#[derive(Clone)]
pub struct Style {
    font: Option<Font>,
    cursor: StdCursor,
}

impl<E> GStyle<E> for Style where
    E: Env + Default + Sync,
    //E::Backend: GBackend<E,Style=Self>,
    //E::StyleSelector: Into<Selector<E>>,
    E::Context: AsRefMut<Core<E>>
{
    type Font = Font;
    type Cursor = StdCursor;
    type Color = Color;
    type Glyphs = Glyphs;
    type Selector = Selector<E>;

    #[inline]
    fn font(&self, v: &Self::Selector, _: &mut E::Context) -> Option<&Self::Font> {
        todo!()
    }
    #[inline]
    fn cursor(&self, v: &Self::Selector, _: &mut E::Context) -> Self::Cursor {
        self.cursor.clone()
    }
    #[inline]
    fn color(&self, v: &Self::Selector, _: &mut E::Context) -> Self::Color {
        Color::from_rgba8(stupid_colors(v.clone().filled()))
    }
    #[inline]
    fn border(&self, v: &Self::Selector, _: &mut E::Context) -> Border {
        stupid_border(v.clone().filled())
    }

    #[inline]
    fn preprocess_text(&self, s: &str, c: &mut E::Context) -> Self::Glyphs {
        todo!()
    }

    #[inline]
    fn is_cached_valid(&self, s: &Self::Glyphs, _c: &mut E::Context) -> bool {
        todo!()
    }

    fn and(&self, s: &Self) -> Self {
        self.clone() //TODO
    }
}

impl AsRefMut<Self> for Style {
    #[inline]
    fn as_ref(&self) -> &Self {
        self
    }
    #[inline]
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

pub fn stupid_border<E>(v: SelectorFilled<E>) -> Border where E: Env {
    match v {
        SelectorFilled{border: BorderPtr::Outer,design: Design::Flat,..} => Border::empty(),
        SelectorFilled{border: BorderPtr::Outer,..} => Border::uniform(2),
        SelectorFilled{border: BorderPtr::Visual,..} => Border::uniform(1),
        _ => Border::uniform(2),
    }
}

pub fn stupid_colors<E>(v: SelectorFilled<E>) -> [u8;4] where E: Env {
    match v {
        SelectorFilled{obj: Obj::Text,..} => [255,255,255,255],
        SelectorFilled{obj: Obj::Foreground,pressed: true,..} => [0,192,0,255],
        SelectorFilled{obj: Obj::Foreground,hovered: true,..} => [64,128,64,255],
        SelectorFilled{obj: Obj::Foreground,..} => [64,64,64,255],
        SelectorFilled{obj: Obj::Active,..} => [0,128,0,255],
        SelectorFilled{obj: Obj::Border,pressed: true,..} => [0,0,0,255],
        SelectorFilled{obj: Obj::Border,focused: true,..} => [255,127,0,255],
        SelectorFilled{obj: Obj::Border,..} => [0,255,0,255],
        SelectorFilled{obj: Obj::Background,..} => [32,32,32,255],
        SelectorFilled{obj: Obj::Default,..} => [32,32,32,255],
        _ => [127,0,0,255],
    }
}
