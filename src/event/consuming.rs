use super::*;

pub trait SDLConsuming {
    const JOYPAD: bool;
    const CONTROLLER: bool;
    const GESTURE: bool;
    const FINGER: bool;
    const APP: bool;
    const WINDOW: bool;
    const KEYBOARD: bool;
    const TEXT_OP: bool;
    const MOUSE: bool;
    const WHEEL: bool;
    const CLIPBOARD_UPDATE: bool;
    const DROP_FILE: bool;
    const USER: bool;
    const UNKNOWN: bool;

    #[inline]
    fn consuming_of(e: &SDLEvent) -> bool {
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

pub struct StdConsuming;

impl SDLConsuming for StdConsuming {
    const JOYPAD: bool = false;
    const CONTROLLER: bool = false;
    const GESTURE: bool = false;
    const FINGER: bool = true;
    const APP: bool = false;
    const WINDOW: bool = false;
    const KEYBOARD: bool = false;
    const TEXT_OP: bool = false;
    const MOUSE: bool = true;
    const WHEEL: bool = true;
    const CLIPBOARD_UPDATE: bool = false;
    const DROP_FILE: bool = true;
    const USER: bool = false;
    const UNKNOWN: bool = false;
}