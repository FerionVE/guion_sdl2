use super::*;
use guion::{util::bounds::Dims, event::{imp::StdVarSup, variant::VariantSupport, variants::RootEvent, variant::Variant}};
use SDLKeycode;

#[allow(unused)]
pub fn parse_event<E>(s: &SDLEvent, window_size: (u32,u32)) -> ParsedEvent<E>
where
    E: Env,
    EEDest<E>: SDLDestination,
    EEKey<E>: From<Key>,
    EEvent<E>: StdVarSup<E> + VariantSupport<Event, E>,
{
    let is = Event { e: s.clone(), ws: (window_size.0 as f32, window_size.1 as f32) };

    match s {
        SDLEvent::KeyDown {
            timestamp,
            window_id,
            keycode,
            scancode,
            keymod,
            repeat,
        } => ret(process_kbd_evt(keycode,*repeat,false),Some(window_id),timestamp),
        SDLEvent::KeyUp {
            timestamp,
            window_id,
            keycode,
            scancode,
            keymod,
            repeat,
        } => ret(process_kbd_evt(keycode,*repeat,true),Some(window_id),timestamp),
        SDLEvent::TextEditing {
            timestamp,
            window_id,
            text,
            start,
            length,
        } => ret(is,Some(window_id),timestamp),
        SDLEvent::TextInput {
            timestamp,
            window_id,
            text,
        } => ret(is,Some(window_id),timestamp),
        SDLEvent::MouseMotion {
            timestamp,
            window_id,
            which,
            mousestate,
            x,
            y,
            xrel,
            yrel,
        } => ret(RootEvent::MouseMove{
            pos: Offset{x: *x, y: *y},
        },Some(window_id),timestamp),
        SDLEvent::MouseButtonDown {
            timestamp,
            window_id,
            which,
            mouse_btn,
            clicks,
            x,
            y,
        } => ret(RootEvent::MouseDown{
            key: Key::Mouse(*mouse_btn,Some(*which)).into(),
        },Some(window_id),timestamp),
        SDLEvent::MouseButtonUp {
            timestamp,
            window_id,
            which,
            mouse_btn,
            clicks,
            x,
            y,
        } => ret(RootEvent::MouseUp{
            key: Key::Mouse(*mouse_btn,Some(*which)).into(),
        },Some(window_id),timestamp),
        SDLEvent::MouseWheel {
            timestamp,
            window_id,
            which,
            x,
            y,
            direction,
        } => ret(is,Some(window_id),timestamp),
        SDLEvent::JoyAxisMotion {
            timestamp,
            which,
            axis_idx,
            value,
        } => ret(is,None,timestamp),
        SDLEvent::JoyBallMotion {
            timestamp,
            which,
            ball_idx,
            xrel,
            yrel,
        } => ret(is,None,timestamp),
        SDLEvent::JoyHatMotion {
            timestamp,
            which,
            hat_idx,
            state,
        } => ret(is,None,timestamp),
        SDLEvent::JoyButtonDown {
            timestamp,
            which,
            button_idx,
        } => ret(is,None,timestamp),
        SDLEvent::JoyButtonUp {
            timestamp,
            which,
            button_idx,
        } => ret(is,None,timestamp),
        SDLEvent::JoyDeviceAdded { timestamp, which } => ret(is,None,timestamp),
        SDLEvent::JoyDeviceRemoved { timestamp, which } => ret(is,None,timestamp),
        SDLEvent::ControllerAxisMotion {
            timestamp,
            which,
            axis,
            value,
        } => ret(is,None,timestamp),
        SDLEvent::ControllerButtonDown {
            timestamp,
            which,
            button,
        } => ret(is,None,timestamp),
        SDLEvent::ControllerButtonUp {
            timestamp,
            which,
            button,
        } => ret(is,None,timestamp),
        SDLEvent::ControllerDeviceAdded { timestamp, which } => ret(is,None,timestamp),
        SDLEvent::ControllerDeviceRemoved { timestamp, which } => ret(is,None,timestamp),
        SDLEvent::ControllerDeviceRemapped { timestamp, which } => ret(is,None,timestamp),
        SDLEvent::FingerDown {
            timestamp,
            touch_id,
            finger_id,
            x,
            y,
            dx,
            dy,
            pressure,
        } => ret(is,None,timestamp),
        SDLEvent::FingerUp {
            timestamp,
            touch_id,
            finger_id,
            x,
            y,
            dx,
            dy,
            pressure,
        } => ret(is,None,timestamp),
        SDLEvent::FingerMotion {
            timestamp,
            touch_id,
            finger_id,
            x,
            y,
            dx,
            dy,
            pressure,
        } => ret(is,None,timestamp),
        SDLEvent::DollarGesture {
            timestamp,
            touch_id,
            gesture_id,
            num_fingers,
            error,
            x,
            y,
        } => ret(is,None,timestamp),
        SDLEvent::DollarRecord {
            timestamp,
            touch_id,
            gesture_id,
            num_fingers,
            error,
            x,
            y,
        } => ret(is,None,timestamp),
        SDLEvent::MultiGesture {
            timestamp,
            touch_id,
            d_theta,
            d_dist,
            x,
            y,
            num_fingers,
        } => ret(is,None,timestamp),
        SDLEvent::ClipboardUpdate { timestamp } => ret(is,None,timestamp),
        SDLEvent::DropFile {
            timestamp,
            window_id,
            filename,
        } => ret(is,Some(window_id),timestamp),
        SDLEvent::DropText {
            timestamp,
            window_id,
            filename,
        } => ret(is,Some(window_id),timestamp),
        SDLEvent::DropBegin {
            timestamp,
            window_id,
        } => ret(is,Some(window_id),timestamp),
        SDLEvent::DropComplete {
            timestamp,
            window_id,
        } => ret(is,Some(window_id),timestamp),
        SDLEvent::AudioDeviceAdded {
            timestamp,
            which,
            iscapture,
        } => ret(is,None,timestamp),
        SDLEvent::AudioDeviceRemoved {
            timestamp,
            which,
            iscapture,
        } => ret(is,None,timestamp),
        SDLEvent::RenderTargetsReset { timestamp } => ret(is,None,timestamp),
        SDLEvent::RenderDeviceReset { timestamp } => ret(is,None,timestamp),
        SDLEvent::User {
            timestamp,
            window_id,
            type_,
            code,
            data1,
            data2,
        } => ret(is,Some(window_id),timestamp),
        SDLEvent::Unknown { timestamp, type_ } => ret(is,None,timestamp),
        SDLEvent::Quit { timestamp } => ret(is,None,timestamp),
        SDLEvent::AppTerminating { timestamp } => ret(is,None,timestamp),
        SDLEvent::AppLowMemory { timestamp } => ret(is,None,timestamp),
        SDLEvent::AppWillEnterBackground { timestamp } => ret(is,None,timestamp),
        SDLEvent::AppDidEnterBackground { timestamp } => ret(is,None,timestamp),
        SDLEvent::AppWillEnterForeground { timestamp } => ret(is,None,timestamp),
        SDLEvent::AppDidEnterForeground { timestamp } => ret(is,None,timestamp),
        SDLEvent::Window {
            timestamp,
            window_id,
            win_event,
        } => match win_event {
            sdl2::event::WindowEvent::None => ret(is,Some(window_id),timestamp),
            sdl2::event::WindowEvent::Shown => ret(is,Some(window_id),timestamp),
            sdl2::event::WindowEvent::Hidden => ret(is,Some(window_id),timestamp),
            sdl2::event::WindowEvent::Exposed => ret(is,Some(window_id),timestamp),
            sdl2::event::WindowEvent::Moved(x,y) => ret(RootEvent::WindowMove{
                pos: Offset{x: *x, y: *y},
                size: Dims::from(window_size),
            },Some(window_id),timestamp),
            sdl2::event::WindowEvent::Resized(w,h) => ret(RootEvent::WindowResize{
                size: Dims{w: *w as u32, h: *h as u32},
            },Some(window_id),timestamp),
            sdl2::event::WindowEvent::SizeChanged(_, _) => ret(is,Some(window_id),timestamp),
            sdl2::event::WindowEvent::Minimized => ret(is,Some(window_id),timestamp),
            sdl2::event::WindowEvent::Maximized => ret(is,Some(window_id),timestamp),
            sdl2::event::WindowEvent::Restored => ret(is,Some(window_id),timestamp),
            sdl2::event::WindowEvent::Enter => ret(is,Some(window_id),timestamp),
            sdl2::event::WindowEvent::Leave => ret(RootEvent::MouseLeaveWindow{},Some(window_id),timestamp),
            sdl2::event::WindowEvent::FocusGained => ret(is,Some(window_id),timestamp),
            sdl2::event::WindowEvent::FocusLost => ret(is,Some(window_id),timestamp),
            sdl2::event::WindowEvent::Close => ret(is,Some(window_id),timestamp),
            sdl2::event::WindowEvent::TakeFocus => ret(is,Some(window_id),timestamp),
            sdl2::event::WindowEvent::HitTest => ret(is,Some(window_id),timestamp),
        },
    }
}

fn process_kbd_evt<E>(key: &Option<SDLKeycode>, repeat: bool, up: bool) -> RootEvent<E> where E: Env, EEKey<E>: From<Key>, {
    if up {
        RootEvent::KbdUp{
            key: Key::Kbd(key.expect("TODO")).into()
        }
    }else if repeat{
        RootEvent::KbdPress{
            key: Key::Kbd(key.expect("TODO")).into()
        }
    }else{
        RootEvent::KbdDown{
            key: Key::Kbd(key.expect("TODO")).into()
        }
    }
}

pub struct ParsedEvent<E> where E: Env {
    pub event: EEvent<E>,
    pub dest_window: Option<u32>,
    pub ts: u32,
}

fn ret<E,T>(event: T, dest_window: Option<&u32>, ts: &u32) -> ParsedEvent<E> where E: Env, EEvent<E>: VariantSupport<T,E>, T: Variant<E> {
    ParsedEvent{
        event: GuionEvent::from(event),
        dest_window: dest_window.cloned(),
        ts: *ts,
    }
} 