extern crate guion_sdl2;

use guion_sdl2::*;
use sdl2::*;
use context::Context;
use guion::widgets::null::Null;

fn main() {
    let sdl = sdl2::init().unwrap();
    
    let c = Context {
        video: sdl.video().unwrap(),
        event: sdl.event().unwrap(),
        pump: sdl.event_pump().unwrap(),
        timer: sdl.timer().unwrap(),
        sdl
    };

    let g = Null::new();

    //TODO Widget resolve impl
}