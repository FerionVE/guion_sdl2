use sdl2::EventPump;
use sdl2::EventSubsystem;
use sdl2::TimerSubsystem;
use sdl2::VideoSubsystem;
use sdl2::Sdl;
use super::*;

pub mod queue;
//pub mod imp;

//TODO make fields private
pub struct Context {
    pub sdl: Sdl,
    pub video: VideoSubsystem,
    pub timer: TimerSubsystem,
    pub event: EventSubsystem,
    pub pump: EventPump,
}