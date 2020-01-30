use guion::core::ctx::aliases::EEKey;
use guion::core::event::variants::*;
use guion::core::backend::Backend as GuionBackend;
use guion::core::event::VariantSupport;
use super::*;

macro_rules! variant {
    ($type:ty,$dest:ident,$cast:block,$back:block) => {
        impl<D,C,E> VariantSupport<$type,E> for Event<EEKey<E>,D,C> where D: SDLDestination, C: SDLConsuming, E: Env, E::Backend: GuionBackend<E,Event=Self> {
            fn from_variant(v: $type) -> Self {
                $back
            }
            fn to_variant(&self) -> Option<$type> {
                match self {
                    SDLEvent::$dest{ee} => Some($cast),
                    _ => None,
                }
            }
        }
    };
}

variant!(KbdDown<EEKey<E>>,KeyDown,{
    ee.keycode;
    unimplemented!()
},{
    unimplemented!()
});