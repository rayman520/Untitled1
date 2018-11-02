
pub fn init() -> super::Globals {

    return super::Globals{
        sdl: init_sdl(),
    };
}

fn init_sdl() -> super::SdlVars {
    let mut sdl_vars = super::SdlVars {
        sdl: sdl2::init().unwrap_or_else(super::utils::exit),
        video_subsystem: nil,
        window: nil,
        event_pump: nil,
    };
    sdl_vars.video_subsystem = sdl_vars.sdl.video().unwrap_or_else(super::utils::exit);
    sdl_vars.window = sdl_vars.video_subsystem
        .window("Game", 900, 700)
        .resizable().build().unwrap_or_else(super::utils::exit);
    sdl_vars.event_pump = sdl_vars.sdl.event_pump().unwrap_or_else(super::utils::exit);
    return sdl_vars
}