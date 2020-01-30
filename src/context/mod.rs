use sdl2::EventPump;
use sdl2::EventSubsystem;
use sdl2::TimerSubsystem;
use sdl2::VideoSubsystem;
use sdl2::Sdl;
use super::*;

pub mod queue;
//pub mod imp;

pub struct Context {
    sdl: Sdl,
    video: VideoSubsystem,
    timer: TimerSubsystem,
    event: EventSubsystem,
    pump: EventPump,
}