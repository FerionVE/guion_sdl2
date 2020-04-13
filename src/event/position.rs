use guion::util::bounds::Offset;
use super::*;

/// extract the absolute position out of a SDL event
pub fn pos_of(e: &SDLEvent, wx: f32, wy: f32) -> Option<Offset> {
    match e {
        SDLEvent::Quit{..} => None,
        SDLEvent::AppTerminating{..} => None,
        SDLEvent::AppLowMemory{..} => None,
        SDLEvent::AppWillEnterBackground{..} => None,
        SDLEvent::AppDidEnterBackground{..} => None,
        SDLEvent::AppWillEnterForeground{..} => None,
        SDLEvent::AppDidEnterForeground{..} => None,
        
        SDLEvent::Window{..} => None,
        
        SDLEvent::KeyDown{..} => None,
        SDLEvent::KeyUp{..} => None,
        
        SDLEvent::TextEditing{..} => None,
        
        SDLEvent::TextInput{..} => None,
        
        SDLEvent::MouseMotion{x,y,..} => Some(Offset{x:*x,y:*y}),
        
        SDLEvent::MouseButtonDown{x,y,..} => Some(Offset{x:*x,y:*y}),
        SDLEvent::MouseButtonUp{x,y,..} => Some(Offset{x:*x,y:*y}),
        
        SDLEvent::MouseWheel{x,y,..} => Some(Offset{x:*x,y:*y}),
        
        SDLEvent::JoyAxisMotion{..} => None,
        
        SDLEvent::JoyBallMotion{..} => None,
        
        SDLEvent::JoyHatMotion{..} => None,
        
        SDLEvent::JoyButtonDown{..} => None,
        SDLEvent::JoyButtonUp{..} => None,
        
        SDLEvent::JoyDeviceAdded{..} => None,
        SDLEvent::JoyDeviceRemoved{..} => None,
        
        SDLEvent::ControllerAxisMotion{..} => None,
        
        SDLEvent::ControllerButtonDown{..} => None,
        SDLEvent::ControllerButtonUp{..} => None,
        
        SDLEvent::ControllerDeviceAdded{..} => None,
        SDLEvent::ControllerDeviceRemoved{..} => None,
        SDLEvent::ControllerDeviceRemapped{..} => None,
        
        SDLEvent::FingerDown{x,y,..} => Some(Offset{x: (x*wx) as i32, y: (y*wy) as i32}),
        SDLEvent::FingerUp{x,y,..} => Some(Offset{x: (x*wx) as i32, y: (y*wy) as i32}),
        SDLEvent::FingerMotion{x,y,..} => Some(Offset{x: (x*wx) as i32, y: (y*wy) as i32}),
        
        SDLEvent::DollarGesture{..} => None,
        SDLEvent::DollarRecord{..} => None,
        
        SDLEvent::MultiGesture{..} => None,
        
        SDLEvent::ClipboardUpdate{..} => None,
        
        SDLEvent::DropFile{..} => None,
        SDLEvent::DropText{..} => None,
        SDLEvent::DropBegin{..} => None,
        SDLEvent::DropComplete{..} => None,

        SDLEvent::AudioDeviceAdded{..} => None,
        SDLEvent::AudioDeviceRemoved{..} => None,
        SDLEvent::RenderTargetsReset{..} => None,
        SDLEvent::RenderDeviceReset{..} => None,
        
        SDLEvent::User{..} => None,
        
        SDLEvent::Unknown{..} => None,
    }
}