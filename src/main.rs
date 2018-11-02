extern crate sdl2;

pub struct SdlVars{
    sdl: sdl2::Sdl,
    video_subsystem: sdl2::VideoSubsystem,
    window: sdl2::video::Window,
    event_pump: sdl2::EventPump,
}

pub struct Globals {
    sdl: SdlVars,
}

pub mod utils;
pub mod init;

fn main() {
    let mut globals= init::init();
    loop {
        utils::events::poll_events(&mut globals)
    }
}