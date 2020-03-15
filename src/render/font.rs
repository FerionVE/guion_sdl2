use super::*;
use sdl2::{ttf::{Sdl2TtfContext, Font, FontError, FontResult}, surface::Surface, rwops::RWops};
use std::collections::HashMap;
use style::color::Color;

pub struct TextProd {
    font: Font<'static,'static>,
    cache: HashMap<(String,SDLColor),Surface<'static>>,
}

impl TextProd {
    pub fn new(c: Sdl2TtfContext) -> Result<Self,String> {
        let c = Box::leak(Box::new(c));
        let p: &'static [u8] = include_bytes!("../../assets/NotoSans-Regular.ttf");
        let rw = RWops::from_bytes(p)?;
        let f = c.load_font_from_rwops(rw,16)?;
        
        Ok(Self{
            font: f,
            cache: HashMap::new(),
        })
    }
    
    pub fn render<'a,C>(&'a mut self, s: &str, c: C) -> Result<&'a Surface<'static>,FontError> where C: GuionColor + Into<Color> {
        let c = c.into().v;
        let key = (s.to_owned(),c.clone());
        if !self.cache.contains_key(&key) {
            let r = self.font.render(s);
            self.cache.insert(key.clone(),r.blended(c).expect("TODO"));
        }
        Ok(self.cache.get(&key).unwrap())
    }
    
    pub fn size(&self, s: &str) -> FontResult<(u32,u32)> {
        self.font.size_of(s)
    }
}