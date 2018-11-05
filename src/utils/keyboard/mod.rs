extern crate sdl2;

use sdl2::keyboard::Keycode;
use std::collections::HashSet;

pub fn get_keypress_map(g: &mut super::super::Globals) -> HashSet<sdl2::keyboard::Keycode, std::collections::hash_map::RandomState> {
    return g.sdl.event_pump.keyboard_state().pressed_scancodes().filter_map(Keycode::from_scancode).collect();
}