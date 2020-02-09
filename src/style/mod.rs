use crate::handler::HandlerInner;
use crate::style::color::Color;
use crate::style::font::Font;
use crate::style::cursor::Cursor;
use crate::style::font::PPChar;
use crate::style::font::PPText;
use guion::core::style::StyleVerb;
use super::*;

pub mod font;
pub mod cursor;
pub mod default;
pub mod color;

pub struct Style {
    pub inner: StyleInner,
}

pub struct StyleInner {
    font: Option<Font>,
    cursor: Cursor,
}

impl AsRefMut<StyleInner> for Style {
    #[inline]
    fn as_ref(&self) -> &StyleInner {
        &self.inner
    }
    #[inline]
    fn as_mut(&mut self) -> &mut StyleInner {
        &mut self.inner
    }
}

impl<E> GuionStyle<E> for Style where E: Env, E::Backend: GuionBackend<E,Style=Self>, ECHandler<E>: AsRefMut<HandlerInner> {
    type Font = Font;
    type Cursor = Cursor;
    type Color = Color;
    type PreprocessedText = PPText;
    type PreprocessedChar = PPChar;

    #[inline]
    fn _with(&mut self, v: StyleVerb) {
        todo!()
    }
    #[inline]
    fn font(&self) -> Option<&Self::Font> {
        self.inner.font.as_ref()
    }
    #[inline]
    fn cursor(&self) -> Self::Cursor {
        self.inner.cursor.clone()
    }
    #[inline]
    fn preprocess_text(&self, s: &str, c: &mut E::Context) -> Self::PreprocessedText {
        todo!()
    }
}

impl PartialEq for Style {
    fn eq(&self, o: &Style) -> bool {
        todo!()
    }
}

impl Clone for Style {
    fn clone(&self) -> Self {
        todo!()
    }
}