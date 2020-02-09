extern crate guion_sdl2;

use guion_sdl2::*;
use sdl2::*;
use context::Context;
use guion::widgets::null::Null;
use simple::env::{SimpleID, SimpleEnv};

fn main() {
    let sdl = sdl2::init().unwrap();
    
    let c = Context::from_sdl2(sdl,h);

    let g: Null<SimpleEnv> = Null::new(
        SimpleID::new(),
        None,
        Vec::new(),
    );

    //TODO Widget resolve impl
}