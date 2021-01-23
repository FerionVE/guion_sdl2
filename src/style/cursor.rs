use super::*;
use guion::style::standard::cursor::StdCursor;
use sdl2::mouse::SystemCursor;

pub fn to_sdl_cursor<C: Into<StdCursor>>(c: C) -> SystemCursor {
    match c.into() {
        StdCursor::Default => SystemCursor::Arrow,
        StdCursor::Arrow => SystemCursor::Arrow,
        StdCursor::IBeam => SystemCursor::IBeam,
        StdCursor::Wait => SystemCursor::Wait,
        StdCursor::Crosshair => SystemCursor::Crosshair,
        StdCursor::WaitArrow => SystemCursor::WaitArrow,
        StdCursor::SizeNWSE => SystemCursor::SizeNWSE,
        StdCursor::SizeNESW => SystemCursor::SizeNESW,
        StdCursor::SizeWE => SystemCursor::SizeWE,
        StdCursor::SizeNS => SystemCursor::SizeNS,
        StdCursor::SizeAll => SystemCursor::SizeAll,
        StdCursor::No => SystemCursor::No,
        StdCursor::Hand => SystemCursor::Hand,
        // default to arrow
        _ => SystemCursor::Arrow,
    }
}
