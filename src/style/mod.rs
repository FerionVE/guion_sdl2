use crate::style::color::Color;
use crate::style::font::Font;
use crate::style::font::Glyphs;
use guion::{env::EnvFlexStyleVariant, style::{variant::{StyleVariantSupport, StdTag, StyleVariantGetStdCursor}, variant::standard::{StdCursor, StdStyleVariant, Obj, BorderPtr}}};
use super::*;

pub mod font;
pub mod cursor;
pub mod default;
pub mod color;

#[derive(Clone)]
pub struct Style {
    font: Option<Font>,
    cursor: StdCursor,
}

impl<E> GuionStyleProvider<E> for Style where
    E: Env + EnvFlexStyleVariant + Sync,
    E::Backend: GuionBackend<E,Style=Self>,
    E::StyleVariant: Into<StdStyleVariant<E>>,
    E::Context: AsRefMut<Core<E>>
{
    type Font = Font;
    type Cursor = StdCursor;
    type Color = Color;
    type Glyphs = Glyphs;
    type Variant = E::StyleVariant;

    #[inline]
    fn font(&self, v: &Self::Variant) -> Option<&Self::Font> {
        todo!()
    }
    #[inline]
    fn cursor(&self, v: &Self::Variant) -> Self::Cursor {
        StyleVariantGetStdCursor::cursor(&v.clone().into())
    }
    #[inline]
    fn color(&self, v: &Self::Variant) -> Self::Color {
        Color::from_rgba8(stupid_colors(v.clone().into()))
    }
    #[inline]
    fn border(&self, v: &Self::Variant) -> Border {
        let v: StdStyleVariant<E> = v.clone().into();
        let thicc = match v.border_ptr {
            BorderPtr::Default => 2,
            BorderPtr::Outer => 2,
            BorderPtr::Visual => 1,
            BorderPtr::Specific(v) => return v,
            _ => 2,
        };
        Border::uniform(thicc * v.border_mul)
    }

    #[inline]
    fn preprocess_text(&self, s: &str, c: &mut E::Context) -> Self::Glyphs {
        todo!()
    }

    #[inline]
    fn is_cached_valid(&self, s: &Self::Glyphs, _c: &mut E::Context) -> bool {
        todo!()
    }

    #[inline]
    fn static_default() -> Self {
        Default::default()
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

pub fn stupid_colors<E>(i: StdStyleVariant<E>) -> [u8;4] where E: Env {
    match i {
        StdStyleVariant{obj: Obj::Text,..} => [255,255,255,255],
        StdStyleVariant{obj: Obj::Foreground,pressed: true,..} => [0,192,0,255],
        StdStyleVariant{obj: Obj::Foreground,hovered: true,..} => [64,128,64,255],
        StdStyleVariant{obj: Obj::Foreground,..} => [64,64,64,255],
        StdStyleVariant{obj: Obj::Active,..} => [0,128,0,255],
        StdStyleVariant{obj: Obj::Border,pressed: true,..} => [0,0,0,255],
        StdStyleVariant{obj: Obj::Border,focused: true,..} => [255,127,0,255],
        StdStyleVariant{obj: Obj::Border,..} => [0,255,0,255],
        StdStyleVariant{obj: Obj::Background,..} => [32,32,32,255],
        StdStyleVariant{obj: Obj::Default,..} => [32,32,32,255],
        _ => [127,0,0,255],
    }
}
