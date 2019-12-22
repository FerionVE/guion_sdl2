use crate::handler::AsSDLHandler;
use std::marker::PhantomData;
use crate::style::default::StyleDefaults;
use guion::core::ctx::Context;
use crate::style::font::Font;
use crate::style::cursor::Cursor;
use crate::style::font::PPChar;
use crate::style::font::PPText;
use std::sync::Arc;
use sdl2::ttf::Sdl2TtfContext;
use guion::core::util::border::Border;
use guion::core::style::StyleVerb;
use sdl2::mouse::Cursor as SDLCursor;
use sdl2::ttf::Font as SDLFont;
use guion::core::style::Style as GuionStyle;

pub mod font;
pub mod cursor;
pub mod default;

pub struct Style<S> where S: StyleDefaults {
    font: Option<Font>,
    cursor: usize,
    _d: PhantomData<S>,
}

impl<C,S> GuionStyle<C> for Style<S> where C: Context<Style=Self>, C::Link: AsSDLHandler<C>, S: StyleDefaults {
    type Font = Font;
    type Cursor = usize;
    type PreprocessedText = PPText;
    type PreprocessedChar = PPChar;

    #[inline]
    fn _with(&mut self, v: StyleVerb) {

    }
    #[inline]
    fn font(&self) -> Option<&Self::Font> {
        self.font.as_ref()
    }
    #[inline]
    fn cursor(&self) -> Self::Cursor {
        self.cursor
    }
    #[inline]
    fn default() -> &'static Self {
        &S::DEFAULT
    }
    #[inline]
    fn default_border() -> &'static Border {
        &S::DEFAULT_BORDER
    }
    #[inline]
    fn preprocess_text(&self, s: &str, c: &mut C) -> Self::PreprocessedText {
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