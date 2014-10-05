extern crate sdl2;
extern crate native;

use sdl2::video::{Window, PosCentered, OpenGL};
use sdl2::event::{QuitEvent, poll_event, NoEvent, KeyDownEvent};
use sdl2::keycode::{EscapeKey};
use sdl2::rect::{Rect};

fn main() {
    // start sdl2 with everything
    sdl2::init(sdl2::InitEverything);

    // Create a window
    let window  = match Window::new("eg03", PosCentered, PosCentered, 640, 480, OpenGL) {
        Ok(window) => window,
        Err(err)   => fail!("failed to create window: {}", err)
    };

    // Create a rendering context
    let renderer = match sdl2::render::Renderer::from_window(window, sdl2::render::DriverAuto, sdl2::render::Accelerated) {
        Ok(renderer) => renderer,
        Err(err) => fail!("failed to create renderer: {}", err)
    };

    // Set the drawing color to a light blue.
    let _ = renderer.set_draw_color(sdl2::pixels::RGB(101, 208, 246));

    // Clear the buffer, using the light blue color set above.
    let _ = renderer.clear();

    // Set the drawing color to a darker blue.
    let _ = renderer.set_draw_color(sdl2::pixels::RGB(0, 153, 204));

    // Create centered Rect, draw the outline of the Rect in our dark blue color.
    let border_rect = Rect::new(320-64, 240-64, 128, 128);
    let _ = match renderer.draw_rect(&border_rect) {
        Ok(_)    => {},
        Err(err) => fail!("failed to draw rect: {}", err) 
    };

    // Create a smaller centered Rect, filling it in the same dark blue.
    let inner_rect = Rect::new(320-60, 240-60, 120, 120);
    let _ = match renderer.fill_rect(&inner_rect) {
        Ok(_)    => {},
        Err(err) => fail!("failed to draw rect: {}", err) 
    };

    // loop until we receive a QuitEvent
    'event : loop {
        // Swap our buffer for the present buffer, displaying it.
        let _ = renderer.present();

        match poll_event() {
              QuitEvent(_)
            | KeyDownEvent(_, _, EscapeKey, _, _) => break 'event,
            NoEvent                               => continue,
            // m                                     => println!("{}", m),
            _ => continue,
        }
    }

    sdl2::quit();
}

