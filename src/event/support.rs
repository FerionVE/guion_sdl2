use crate::event::key::Key;
use guion::core::event::variants::*;
use guion::core::event::{Variant, VariantSupport};
use super::*;

/*macro_rules! variant {
    ($type:ty,$recv:pat,$cast:block,$back:block) => {
        impl<K,D,C,E> VariantSupport<$type,E> for Event<K,D,C> where K: GuionKey + FromInto<Key> + 'static, D: SDLDestination, C: SDLConsuming, E: Env, E::Backend: GuionBackend<E,Event=Self> {
            fn from_variant(v: $type) -> Self {
                $back
            }
            fn to_variant(&self) -> Option<$type> {
                match self.e {
                    $recv => Some($cast),
                    _ => None,
                }
            }
        }
    };
}

variant!(
    KbdDown<E>,
    SDLEvent::KeyDown{
        timestamp,
        window_id,
        keycode,
        scancode,
        keymod,
        repeat
    },{
        if repeat || keycode.is_none() {return None;}
        KbdDown{
            key: K::qfrom(
                Key::Kbd(keycode.unwrap()),
            )
        }
    },{
        todo!()
    }
);*/

impl<K,D,C,E,T> VariantSupport<T,E> for Event<K,D,C> where
    K: GuionKey + FromInto<Key> + 'static,
    D: SDLDestination,
    C: SDLConsuming,
    E: Env,
    E::Backend: GuionBackend<E,Event=Self>,
    T: Variant<E>
{
    fn from_variant(v: T) -> Self {
        todo!()
    }
    fn to_variant(&self) -> Option<T> {
        todo!()
    }
}