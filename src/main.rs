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

use std::time::Duration;

fn main() {
    let mut globals= init::init();
    loop {
        utils::events::poll_events(&mut globals);
        let pressed_keys = utils::keyboard::get_keypress_map(&mut globals);
        println!("{:?}", pressed_keys);
        std::thread::sleep(Duration::from_millis(10));

    }
}