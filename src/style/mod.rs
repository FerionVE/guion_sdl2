use crate::handler::HandlerInner;
use crate::style::color::Color;
use crate::style::default::StyleDefaults;
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

pub struct Style<S> where S: StyleDefaults {
    pub inner: StyleInner,
    _d: PhantomData<S>,
}

pub struct StyleInner {
    font: Option<Font>,
    cursor: Cursor,
}

impl<S> AsRefMut<StyleInner> for Style<S> where S: StyleDefaults {
    #[inline]
    fn as_ref(&self) -> &StyleInner {
        &self.inner
    }
    #[inline]
    fn as_mut(&mut self) -> &mut StyleInner {
        &mut self.inner
    }
}

impl<E,S> GuionStyle<E> for Style<S> where E: Env, E::Backend: GuionBackend<E,Style=Self>, ECHandler<E>: AsRefMut<HandlerInner>, S: StyleDefaults {
    type Font = Font;
    type Cursor = Cursor;
    type Color = Color;
    type PreprocessedText = PPText;
    type PreprocessedChar = PPChar;

    #[inline]
    fn _with(&mut self, v: StyleVerb) {
        unimplemented!()
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
    fn default() -> &'static Self {
        S::DEFAULT
    }
    #[inline]
    fn default_border() -> &'static Border {
        S::DEFAULT_BORDER
    }
    #[inline]
    fn preprocess_text(&self, s: &str, c: &mut E::Context) -> Self::PreprocessedText {
        unimplemented!()
    }
}

impl<S> PartialEq for Style<S> where S: StyleDefaults {
    fn eq(&self, o: &Style<S>) -> bool {
        unimplemented!()
    }
}

impl<S> Clone for Style<S> where S: StyleDefaults {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}