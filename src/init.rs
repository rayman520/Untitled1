extern crate sdl2;

pub fn init() -> super::Globals {
    return super::Globals{
        sdl: init_sdl(),
    };
}

use utils::CheckResult;

fn init_sdl() -> super::SdlVars {
    let tmp_sdl = sdl2::init().check_result();
    let tmp_event_pump = tmp_sdl.event_pump().check_result();
    let tmp_video_subsystem = tmp_sdl.video().check_result();
    let tmp_window = tmp_video_subsystem
        .window("Game", 900, 700)
        .resizable().build().check_result();


    let mut sdl_vars = super::SdlVars {
        sdl: tmp_sdl,
        event_pump: tmp_event_pump,
        video_subsystem: tmp_video_subsystem,
        window: tmp_window,
    };
    return sdl_vars
}