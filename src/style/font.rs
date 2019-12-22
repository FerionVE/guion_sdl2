use sdl2::pixels::Color;
use sdl2::ttf::FontStyle;
use std::path::Path;
use guion::core::util::bounds::Offset;
use guion::core::util::bounds::Dims;
use guion::core::style::font::PreprocessedChar;
use guion::core::style::font::PreprocessedText;
use super::*;
#[derive(Clone,PartialEq)]
pub struct Font {
    pub source: FontSource,
    pub index: u32,
    pub size: u16,
    pub style: FontStyle,
    pub render: FontRender,
}
#[derive(Clone,PartialEq)]
pub enum FontSource {
    File(&'static Path),
    Memory(&'static [u8]),
}
#[derive(Clone,PartialEq)]
pub enum FontRender {
    Solid(),
    Shaded(),
    Blended(),
    BlendedWrapped(Color,u32),
}

pub struct PPText {

}

pub struct PPChar {

}

impl<E,S> PreprocessedText<Style<S>,E> for PPText where E: Env<Style=Style<S>>, ECHLink<E>: AsSDLHandler<E::Context>, S: StyleDefaults {
    fn size(&self) -> Dims {
        unimplemented!()
    }
    fn style(&self) -> &Style<S> {
        unimplemented!()
    }
    fn chars(&self) -> &[PPChar] {
        unimplemented!()
    }
    fn back(&self) -> String {
        unimplemented!()
    }
}

impl PreprocessedChar for PPChar {
    fn offset(&self) -> Offset {
        unimplemented!()
    }
    fn char(&self) -> char {
        unimplemented!()
    }
}