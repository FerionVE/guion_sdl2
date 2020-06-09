use super::*;
use guion::{util::bounds::Dims, event::{imp::StdVarSup, variant::VariantSupport, variants::{TextInput, RootEvent, MouseScroll}, variant::Variant}};
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
        } => ret(process_kbd_evt(scancode,*repeat,false),timestamp),
        SDLEvent::KeyUp {
            timestamp,
            window_id,
            keycode,
            scancode,
            keymod,
            repeat,
        } => ret(process_kbd_evt(scancode,*repeat,true),timestamp),
        SDLEvent::TextEditing {
            timestamp,
            window_id,
            text,
            start,
            length,
        } => ret(is,timestamp),
        SDLEvent::TextInput {
            timestamp,
            window_id,
            text,
        } => ret(RootEvent::TextInput{text: text.to_owned()},timestamp),
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
        },timestamp),
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
        },timestamp),
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
        },timestamp),
        SDLEvent::MouseWheel {
            timestamp,
            window_id,
            which,
            x,
            y,
            direction,
        } => ret(RootEvent::MouseScroll{x: (-*x)*8, y: (-*y)*32},timestamp),
        SDLEvent::JoyAxisMotion {
            timestamp,
            which,
            axis_idx,
            value,
        } => ret(is,timestamp),
        SDLEvent::JoyBallMotion {
            timestamp,
            which,
            ball_idx,
            xrel,
            yrel,
        } => ret(is,timestamp),
        SDLEvent::JoyHatMotion {
            timestamp,
            which,
            hat_idx,
            state,
        } => ret(is,timestamp),
        SDLEvent::JoyButtonDown {
            timestamp,
            which,
            button_idx,
        } => ret(is,timestamp),
        SDLEvent::JoyButtonUp {
            timestamp,
            which,
            button_idx,
        } => ret(is,timestamp),
        SDLEvent::JoyDeviceAdded { timestamp, which } => ret(is,timestamp),
        SDLEvent::JoyDeviceRemoved { timestamp, which } => ret(is,timestamp),
        SDLEvent::ControllerAxisMotion {
            timestamp,
            which,
            axis,
            value,
        } => ret(is,timestamp),
        SDLEvent::ControllerButtonDown {
            timestamp,
            which,
            button,
        } => ret(is,timestamp),
        SDLEvent::ControllerButtonUp {
            timestamp,
            which,
            button,
        } => ret(is,timestamp),
        SDLEvent::ControllerDeviceAdded { timestamp, which } => ret(is,timestamp),
        SDLEvent::ControllerDeviceRemoved { timestamp, which } => ret(is,timestamp),
        SDLEvent::ControllerDeviceRemapped { timestamp, which } => ret(is,timestamp),
        SDLEvent::FingerDown {
            timestamp,
            touch_id,
            finger_id,
            x,
            y,
            dx,
            dy,
            pressure,
        } => ret(is,timestamp),
        SDLEvent::FingerUp {
            timestamp,
            touch_id,
            finger_id,
            x,
            y,
            dx,
            dy,
            pressure,
        } => ret(is,timestamp),
        SDLEvent::FingerMotion {
            timestamp,
            touch_id,
            finger_id,
            x,
            y,
            dx,
            dy,
            pressure,
        } => ret(is,timestamp),
        SDLEvent::DollarGesture {
            timestamp,
            touch_id,
            gesture_id,
            num_fingers,
            error,
            x,
            y,
        } => ret(is,timestamp),
        SDLEvent::DollarRecord {
            timestamp,
            touch_id,
            gesture_id,
            num_fingers,
            error,
            x,
            y,
        } => ret(is,timestamp),
        SDLEvent::MultiGesture {
            timestamp,
            touch_id,
            d_theta,
            d_dist,
            x,
            y,
            num_fingers,
        } => ret(is,timestamp),
        SDLEvent::ClipboardUpdate { timestamp } => ret(is,timestamp),
        SDLEvent::DropFile {
            timestamp,
            window_id,
            filename,
        } => ret(is,timestamp),
        SDLEvent::DropText {
            timestamp,
            window_id,
            filename,
        } => ret(is,timestamp),
        SDLEvent::DropBegin {
            timestamp,
            window_id,
        } => ret(is,timestamp),
        SDLEvent::DropComplete {
            timestamp,
            window_id,
        } => ret(is,timestamp),
        SDLEvent::AudioDeviceAdded {
            timestamp,
            which,
            iscapture,
        } => ret(is,timestamp),
        SDLEvent::AudioDeviceRemoved {
            timestamp,
            which,
            iscapture,
        } => ret(is,timestamp),
        SDLEvent::RenderTargetsReset { timestamp } => ret(is,timestamp),
        SDLEvent::RenderDeviceReset { timestamp } => ret(is,timestamp),
        SDLEvent::User {
            timestamp,
            window_id,
            type_,
            code,
            data1,
            data2,
        } => ret(is,timestamp),
        SDLEvent::Unknown { timestamp, type_ } => ret(is,timestamp),
        SDLEvent::Quit { timestamp } => ret(is,timestamp),
        SDLEvent::AppTerminating { timestamp } => ret(is,timestamp),
        SDLEvent::AppLowMemory { timestamp } => ret(is,timestamp),
        SDLEvent::AppWillEnterBackground { timestamp } => ret(is,timestamp),
        SDLEvent::AppDidEnterBackground { timestamp } => ret(is,timestamp),
        SDLEvent::AppWillEnterForeground { timestamp } => ret(is,timestamp),
        SDLEvent::AppDidEnterForeground { timestamp } => ret(is,timestamp),
        SDLEvent::Window {
            timestamp,
            window_id,
            win_event,
        } => match win_event {
            sdl2::event::WindowEvent::None => ret(is,timestamp),
            sdl2::event::WindowEvent::Shown => ret(is,timestamp),
            sdl2::event::WindowEvent::Hidden => ret(is,timestamp),
            sdl2::event::WindowEvent::Exposed => ret(is,timestamp),
            sdl2::event::WindowEvent::Moved(x,y) => ret(RootEvent::WindowMove{
                pos: Offset{x: *x, y: *y},
                size: Dims::from(window_size),
            },timestamp),
            sdl2::event::WindowEvent::Resized(w,h) => ret(RootEvent::WindowResize{
                size: Dims{w: *w as u32, h: *h as u32},
            },timestamp),
            sdl2::event::WindowEvent::SizeChanged(_, _) => ret(is,timestamp),
            sdl2::event::WindowEvent::Minimized => ret(is,timestamp),
            sdl2::event::WindowEvent::Maximized => ret(is,timestamp),
            sdl2::event::WindowEvent::Restored => ret(is,timestamp),
            sdl2::event::WindowEvent::Enter => ret(is,timestamp),
            sdl2::event::WindowEvent::Leave => ret(RootEvent::MouseLeaveWindow{},timestamp),
            sdl2::event::WindowEvent::FocusGained => ret(is,timestamp),
            sdl2::event::WindowEvent::FocusLost => ret(is,timestamp),
            sdl2::event::WindowEvent::Close => ret(is,timestamp),
            sdl2::event::WindowEvent::TakeFocus => ret(is,timestamp),
            sdl2::event::WindowEvent::HitTest => ret(is,timestamp),
        },
    }
}

fn process_kbd_evt<E>(key: &Option<SDLScancode>, repeat: bool, up: bool) -> RootEvent<E> where E: Env, EEKey<E>: From<Key>, {
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
    pub ts: u32,
}

fn ret<E,T>(event: T, ts: &u32) -> ParsedEvent<E> where E: Env, EEvent<E>: VariantSupport<T,E>, T: Variant<E> {
    ParsedEvent{
        event: GuionEvent::from(event),
        ts: *ts,
    }
}

#[allow(unused)]
pub fn event_dest_window(s: &SDLEvent) -> Option<u32> {
    match s {
        SDLEvent::KeyDown {window_id,..} => Some(*window_id),
        SDLEvent::KeyUp {window_id,..} => Some(*window_id),
        SDLEvent::TextEditing {window_id,..} => Some(*window_id),
        SDLEvent::TextInput {window_id,..} => Some(*window_id),
        SDLEvent::MouseMotion {window_id,..} => Some(*window_id),
        SDLEvent::MouseButtonDown {window_id,..} => Some(*window_id),
        SDLEvent::MouseButtonUp {window_id,..} => Some(*window_id),
        SDLEvent::MouseWheel {window_id,..} => Some(*window_id),
        SDLEvent::JoyAxisMotion {..} => None,
        SDLEvent::JoyBallMotion {..} => None,
        SDLEvent::JoyHatMotion {..} => None,
        SDLEvent::JoyButtonDown {..} => None,
        SDLEvent::JoyButtonUp {..} => None,
        SDLEvent::JoyDeviceAdded {..} => None,
        SDLEvent::JoyDeviceRemoved {..} => None,
        SDLEvent::ControllerAxisMotion {..} => None,
        SDLEvent::ControllerButtonDown {..} => None,
        SDLEvent::ControllerButtonUp {..} => None,
        SDLEvent::ControllerDeviceAdded {..} => None,
        SDLEvent::ControllerDeviceRemoved {..} => None,
        SDLEvent::ControllerDeviceRemapped {..} => None,
        SDLEvent::FingerDown {..} => None,
        SDLEvent::FingerUp {..} => None,
        SDLEvent::FingerMotion {..} => None,
        SDLEvent::DollarGesture {..} => None,
        SDLEvent::DollarRecord {..} => None,
        SDLEvent::MultiGesture {..} => None,
        SDLEvent::ClipboardUpdate {..} => None,
        SDLEvent::DropFile {window_id,..} => Some(*window_id),
        SDLEvent::DropText {window_id,..} => Some(*window_id),
        SDLEvent::DropBegin {window_id,..} => Some(*window_id),
        SDLEvent::DropComplete {window_id,..} => Some(*window_id),
        SDLEvent::AudioDeviceAdded {..} => None,
        SDLEvent::AudioDeviceRemoved {..} => None,
        SDLEvent::RenderTargetsReset {..} => None,
        SDLEvent::RenderDeviceReset {..} => None,
        SDLEvent::User {window_id,..} => Some(*window_id),
        SDLEvent::Unknown {..} => None,
        SDLEvent::Quit {..} => None,
        SDLEvent::AppTerminating {..} => None,
        SDLEvent::AppLowMemory {..} => None,
        SDLEvent::AppWillEnterBackground {..} => None,
        SDLEvent::AppDidEnterBackground {..} => None,
        SDLEvent::AppWillEnterForeground {..} => None,
        SDLEvent::AppDidEnterForeground {..} => None,
        SDLEvent::Window {window_id,..} => Some(*window_id),
    }
}