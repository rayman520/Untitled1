
pub fn poll_events(g: &mut super::super::Globals) {
    for event in g.sdl.event_pump.poll_iter() {
        match event {
            sdl2::event::Event::Quit { .. } => super::exit("window closed"),
            _ => {/*println!("{:?}", event)*/},
        }
    }
}