use super::*;
use qwutils::imp::option::OptionExt;
#[derive(Clone,Debug)]
pub enum Key {
    Kbd(SDLScancode),
    Mouse(SDLMouseButton,Option<u32>),
}
#[derive(Clone,PartialEq)]
pub struct PressedKey<E> where E: Env {
    pub v: Key,
    pub ts: u32,
    pub window: u32,
    pub widget: E::WidgetID,
}

pub enum KeyOrigin {
    Kbd(),
    Mouse(Option<u32>),
}

impl GuionKey for Key {
    type Origin = KeyOrigin;
    const MOUSE_LEFT: Self = Self::Mouse(SDLMouseButton::Left,None);
    const ENTER: Self = Self::Kbd(SDLScancode::Return);
    const SPACE: Self = Self::Kbd(SDLScancode::Space);
    const TAB: Self = Self::Kbd(SDLScancode::Tab);
    const BACKSPACE: Self = Self::Kbd(SDLScancode::Backspace);

    fn origin(&self) -> Self::Origin {
        match self {
            Key::Kbd(_) => KeyOrigin::Kbd(),
            Key::Mouse(_,o) => KeyOrigin::Mouse(*o),
        }
    }
}

impl PartialEq for Key {
    fn eq(&self, o: &Self) -> bool {
        match self {
            Key::Kbd(key) => match o {
                Key::Kbd(okey) => key == okey,
                _ => false,
            }
            Key::Mouse(key,origin) => match o {
                Key::Mouse(okey,oorigin) => 
                    key == okey && origin.with_if(oorigin,|a,b| a==b ).unwrap_or(true),
                _ => false,
            }
        }
        //todo!()
        //if one or both origins are none -> true
    }
}