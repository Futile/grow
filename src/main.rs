extern crate sdl2;
extern crate native;
extern crate time;
extern crate num;

use sdl2::video::{Window, PosCentered, OpenGL};
use sdl2::event::{QuitEvent, poll_event, KeyDownEvent};
use sdl2::keycode::{EscapeKey};
use sdl2::rect::{Rect, Point};
use time::{get_time, Timespec};
use std::io::timer::{sleep};
use std::time::Duration;

fn main() {
    let renderer = Renderer::init(get_time());

    let mut last_frame = get_time();
    let time_per_frame = Duration::microseconds(1000000 / 60);
        
    // loop until we receive a QuitEvent
    'gameloop : loop {
        //println!("{}", 1000.0 / ((get_time() - last_frame).num_milliseconds() as f64));
        
        //let elapsed = get_time() - last_frame;
        sleep( last_frame + time_per_frame - get_time() );
        last_frame = get_time();
        
        renderer.draw(last_frame);
        
        match poll_event() {
              QuitEvent(_)
            | KeyDownEvent(_, _, EscapeKey, _, _) => break 'gameloop,
            _                                     => continue,
        }
    }

    sdl2::quit();
}

struct Renderer {
    renderer : sdl2::render::Renderer<Window>,
    start_time : Timespec,
}

impl Renderer {

    fn init(start_time : Timespec) -> Renderer {
        // start sdl2 with everything
        sdl2::init(sdl2::InitEverything);

        // Create a window
        let window  = match Window::new("grow", PosCentered, PosCentered, 640, 480, OpenGL) {
            Ok(window) => window,
            Err(err)   => fail!("failed to create window: {}", err)
        };

        // Create a rendering context
        let renderer = match sdl2::render::Renderer::from_window(window, sdl2::render::DriverAuto, sdl2::render::Accelerated) {
            Ok(renderer) => renderer,
            Err(err) => fail!("failed to create renderer: {}", err)
        };

        Renderer{renderer:renderer, start_time: start_time}
    }
    
    fn elapsed(&self, last_frame : Timespec) -> Duration {last_frame - self.start_time}

    fn draw(&self, last_frame : Timespec) {
            // Set the drawing color to a light blue.
            let _ = self.renderer.set_draw_color(sdl2::pixels::RGB(101, 208, 246));

            // Clear the buffer, using the light blue color set above.
            let _ = self.renderer.clear();

            // Set the drawing color to a darker blue.
            let _ = self.renderer.set_draw_color(sdl2::pixels::RGB(0, 153, 204));

            let millis = self.elapsed(last_frame).num_milliseconds() as f64;
            let angle = millis / 1000f64;
            let r = 200f64;
            let a = Point{ x: ((angle*0.5).cos()*r+r) as i32, y: ((angle*0.5).sin()*r+r) as i32};
            let b = Point{ x: (-angle.cos()*r+r) as i32, y: (-angle.sin()*r+r) as i32};
            
            let _ = match self.renderer.draw_line(a,b) {
                Ok(_) => {},
                Err(err) => fail!("failed to draw line: {}", err)
            };
            
            // Create centered Rect, draw the outline of the Rect in our dark blue color.
            let border_rect = Rect::new(320-64, 240-64, 128, 128);
            let _ = match self.renderer.draw_rect(&border_rect) {
                Ok(_)    => {},
                Err(err) => fail!("failed to draw rect: {}", err)
            };
            
            // Swap our buffer for the present buffer, displaying it.
            let _ = self.renderer.present();
    }
}

