use sdl2::pixels::Color as SDLColor;
use guion::core::style::color::Color as GuionColor;

pub struct Color {
    pub v: SDLColor,
}

impl GuionColor for Color {
    #[inline]
    fn from_rgba8(c: [u8;4]) -> Self {
        Self{
            v: SDLColor::RGBA(c[0], c[1], c[2], c[3]),
        }
    }
    #[inline]
    fn into_rgba8(&self) -> [u8;4] {
        [self.v.r,self.v.g,self.v.b,self.v.a]
    }
}