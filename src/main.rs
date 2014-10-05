extern crate sdl2;
extern crate native;

use sdl2::video::{Window, PosCentered, OpenGL};
use sdl2::event::{QuitEvent, NoEvent, poll_event};
use sdl2::timer::{delay};

fn main() {
    sdl2::init(sdl2::InitEverything);

    let window = match Window::new("yolo", PosCentered, PosCentered, 640, 480, OpenGL) {
        Ok(window) => window,
        Err(err) => fail!("failed to create window: {}", err)
    };

    window.show();

    'event : loop {
        match poll_event() {
            QuitEvent(_) => break 'event,
            NoEvent => continue,
            event => println!("event: {}", event),
        }
    }

    sdl2::quit();
}
