use sdl2::pixels::Color;
use sdl2::ttf::FontStyle;
use std::path::Path;
use guion::core::util::bounds::Offset;
use guion::core::util::bounds::Dims;
use guion::core::style::font::PreprocessedChar;
use guion::core::style::font::PreprocessedText;
use guion::core::backend::Backend;
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

impl<E> PreprocessedText<E> for PPText where
    E: Env + EnvFlexStyleVariant + Sync,
    EStyle<E>: GuionStyle<E,PreprocessedText=Self,PreprocessedChar=PPChar>,
    E::Context: AsRefMut<Core<E>>
{
    fn size(&self) -> Dims {
        todo!()
    }
    fn style(&self) -> &EStyle<E> {
        todo!()
    }
    fn chars(&self) -> &[PPChar] {
        todo!()
    }
    fn back(&self) -> String {
        todo!()
    }
}

impl PreprocessedChar for PPChar {
    fn offset(&self) -> Offset {
        todo!()
    }
    fn char(&self) -> char {
        todo!()
    }
}