use crate::style::color::Color;
use crate::style::font::Font;
use crate::style::font::PPChar;
use crate::style::font::PPText;
use guion::core::{env::EnvFlexStyleVariant, style::{StyleVariantSupport, StdVerb, standard::StdStyleVariant, cursor::StdCursor, StyleVariantGetStdCursor}};
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

impl<E> GuionStyle<E> for Style where
    E: Env + EnvFlexStyleVariant + Sync,
    E::Backend: GuionBackend<E,Style=Self>,
    E::StyleVariant: StyleVariantGetStdCursor,
    E::Context: AsRefMut<Core<E>>
{
    type Font = Font;
    type Cursor = StdCursor;
    type Color = Color;
    type PreprocessedText = PPText;
    type PreprocessedChar = PPChar;
    type Variant = E::StyleVariant;

    fn font(&self, v: &Self::Variant) -> Option<&Self::Font> {
        todo!()
    }
    fn cursor(&self, v: &Self::Variant) -> Self::Cursor {
        v.cursor()
    }
    fn color(&self, v: &Self::Variant) -> Self::Color {
        Color::from_rgba8([127,0,0,255])
    }
    fn preprocess_text(&self, s: &str, c: &mut E::Context) -> Self::PreprocessedText {
        todo!()
    }

    #[inline]
    fn is_cached_valid(&self, s: &Self::PreprocessedText, _c: &mut E::Context) -> bool {
        todo!()
    }

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