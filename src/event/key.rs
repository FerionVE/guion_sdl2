use super::*;
#[derive(Clone)]
pub enum Key {
    Kbd(SDLKeycode),
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
    const ENTER: Self = Self::Kbd(SDLKeycode::Return);
    const TAB: Self = Self::Kbd(SDLKeycode::Tab);

    fn origin(&self) -> Self::Origin {
        match self {
            Key::Kbd(_) => KeyOrigin::Kbd(),
            Key::Mouse(_,o) => KeyOrigin::Mouse(*o),
        }
    }
}

impl PartialEq for Key {
    fn eq(&self, o: &Self) -> bool {
        unimplemented!()
        //if one or both origins are none -> true
    }
}