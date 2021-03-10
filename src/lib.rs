pub mod event;
pub mod render;
pub mod style;
pub mod handler;
pub mod core;
pub mod simple;
//pub mod util;

pub use qwutils;
pub use guion;
pub use sdl2;

//TODO refactor this import hell

use sdl2::EventPump;
use sdl2::EventSubsystem;
use sdl2::TimerSubsystem;
use sdl2::VideoSubsystem;
use sdl2::Sdl;

//use qwutils::*;
use qwutils::refc::RefClonable;

use guion::env::Env as Env;
use guion::ctx::queue::{Queue as GQueue};
use guion::event::Destination as GDestination;

use guion::aliases::*;
use guion::util::bounds::Offset as Offset;

use guion::util::bounds::Bounds as Bounds;
use guion::util::error::GuionError;
use guion::event::key::Key as GPressedKey;
use guion::event::key::Key as GKey;
use guion::util::border::Border;
use guion::path::{WidgetPath as GPath,SubPath as GSubPath};

use sdl2::keyboard::Keycode as SDLKeycode;
use sdl2::keyboard::Scancode as SDLScancode;
use sdl2::mouse::MouseButton as SDLMouseButton;
use sdl2::mouse::Cursor as SDLCursor;

use guion::event::Event as GEvent;
use sdl2::event::Event as SDLEvent;

use std::marker::PhantomData;
use std::ops::DerefMut;
use std::ops::Deref;

use guion::backend::Backend as GBackend;

use guion::widget::root::Widgets as GWidgets;
use guion::widget::link::Link;

use guion::util::AsRefMut;

use guion::handler::Handler as GHandler;
use guion::ctx::Context as GContext;

use guion::render::Render as GRender;

use sdl2::pixels::Color as SDLColor;
use guion::style::color::Color as GColor;

use guion::style::Style as GStyle;

use crate::core::*;
