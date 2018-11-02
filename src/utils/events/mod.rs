
pub fn poll_events(g: &mut super::super::Globals) {
    for event in g.sdl_event_pump.poll_iter() {
        match event {
            sdl2::event::Event::Quit { .. } => super::exit(),
            _ => {println!("{:?}", event)},
        }
    }
}