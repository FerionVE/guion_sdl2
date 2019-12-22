use guion::core::event::Destination;
use super::*;

pub trait SDLDestination: Destination {
    const JOYPAD: Self;
    const CONTROLLER: Self;
    const GESTURE: Self;
    const FINGER: Self;
    const APP: Self;
    const WINDOW: Self;
    const KEYBOARD: Self;
    const TEXT_OP: Self;
    const MOUSE: Self;
    const WHEEL: Self;
    const CLIPBOARD_UPDATE: Self;
    const DROP_FILE: Self;
    const USER: Self;
    const UNKNOWN: Self;

    #[inline]
    fn destination_of(e: &SDLEvent) -> Self {
        match e {
            SDLEvent::Quit{..} => Self::APP,
            SDLEvent::AppTerminating{..} => Self::APP,
            SDLEvent::AppLowMemory{..} => Self::APP,
            SDLEvent::AppWillEnterBackground{..} => Self::APP,
            SDLEvent::AppDidEnterBackground{..} => Self::APP,
            SDLEvent::AppWillEnterForeground{..} => Self::APP,
            SDLEvent::AppDidEnterForeground{..} => Self::APP,

            SDLEvent::Window{..} => Self::WINDOW,

            SDLEvent::KeyDown{..} => Self::KEYBOARD,
            SDLEvent::KeyUp{..} => Self::KEYBOARD,

            SDLEvent::TextEditing{..} => Self::TEXT_OP,

            SDLEvent::TextInput{..} => Self::TEXT_OP,

            SDLEvent::MouseMotion{..} => Self::MOUSE,

            SDLEvent::MouseButtonDown{..} => Self::MOUSE,
            SDLEvent::MouseButtonUp{..} => Self::MOUSE,

            SDLEvent::MouseWheel{..} => Self::WHEEL,

            SDLEvent::JoyAxisMotion{..} => Self::JOYPAD,

            SDLEvent::JoyBallMotion{..} => Self::JOYPAD,

            SDLEvent::JoyHatMotion{..} => Self::JOYPAD,

            SDLEvent::JoyButtonDown{..} => Self::JOYPAD,
            SDLEvent::JoyButtonUp{..} => Self::JOYPAD,

            SDLEvent::JoyDeviceAdded{..} => Self::JOYPAD,
            SDLEvent::JoyDeviceRemoved{..} => Self::JOYPAD,

            SDLEvent::ControllerAxisMotion{..} => Self::CONTROLLER,

            SDLEvent::ControllerButtonDown{..} => Self::CONTROLLER,
            SDLEvent::ControllerButtonUp{..} => Self::CONTROLLER,

            SDLEvent::ControllerDeviceAdded{..} => Self::CONTROLLER,
            SDLEvent::ControllerDeviceRemoved{..} => Self::CONTROLLER,
            SDLEvent::ControllerDeviceRemapped{..} => Self::CONTROLLER,

            SDLEvent::FingerDown{..} => Self::FINGER,
            SDLEvent::FingerUp{..} => Self::FINGER,
            SDLEvent::FingerMotion{..} => Self::FINGER,

            SDLEvent::DollarGesture{..} => Self::GESTURE,
            SDLEvent::DollarRecord{..} => Self::GESTURE,

            SDLEvent::MultiGesture{..} => Self::GESTURE,

            SDLEvent::ClipboardUpdate{..} => Self::CLIPBOARD_UPDATE,

            SDLEvent::DropFile{..} => Self::DROP_FILE,

            SDLEvent::User{..} => Self::USER,

            SDLEvent::Unknown{..} => Self::UNKNOWN,
        }
    }
}
#[derive(Clone)]
pub struct StdDest<D> where D: Destination {
    pub v: D,
}

impl<D> Destination for StdDest<D> where D: Destination {
    const ROOT: Self = Self{v: D::ROOT};
    const SELECTED: Self = Self{v: D::SELECTED};
}

impl<D> SDLDestination for StdDest<D> where D: Destination {
    const JOYPAD: Self = Self{v: D::SELECTED};
    const CONTROLLER: Self = Self{v: D::SELECTED};
    const GESTURE: Self = Self{v: D::ROOT};
    const FINGER: Self = Self{v: D::ROOT};
    const APP: Self = Self{v: D::ROOT};
    const WINDOW: Self = Self{v: D::ROOT};
    const KEYBOARD: Self = Self{v: D::SELECTED};
    const TEXT_OP: Self = Self{v: D::SELECTED};
    const MOUSE: Self = Self{v: D::ROOT};
    const WHEEL: Self = Self{v: D::ROOT};
    const CLIPBOARD_UPDATE: Self = Self{v: D::ROOT};
    const DROP_FILE: Self = Self{v: D::ROOT};
    const USER: Self = Self{v: D::ROOT};
    const UNKNOWN: Self = Self{v: D::ROOT};
}