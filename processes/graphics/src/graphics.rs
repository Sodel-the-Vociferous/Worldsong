extern crate common;
extern crate sdl2; 

use common::data::Data;
use sdl2::pixels;
use sdl2::event;
use sdl2::keycode;

// Processes are stateless libraries that modify persistent state
// (stored in data.rs)
// Because they're stateless, they can be added, removed, and modified at runtime.

#[no_mangle]
pub fn variable_update(data: &mut Data) -> () {
    match event::poll_event() {
        event::QuitEvent(_) => data.kernel.quit = true,
        event::KeyDownEvent(_, _, key, _, _) => {
            if key == keycode::EscapeKey {
                data.kernel.quit = true;
            }
        },
        event::WindowEvent(_, _, id, _, _) => {
            if id as int == event::FocusGainedWindowEventId as int {
                if data.window.first_focus {
                    data.window.first_focus = false;
                    return;
                }
                if !data.kernel.load_processes {
                    data.kernel.load_processes = true;
                }
            }
        }
        _ => {}
    }
}

#[no_mangle]
pub fn fixed_update(data: &mut Data) -> () {

    let renderer = data.window.renderer.as_ref().unwrap();

    // For example, while the kernel is running, try modifying these values,
    // compiling this process via the local ./compile.sh, and
    // re-focusing the Worldsong window.
    data.sim.color_r += 1;
    data.sim.color_g -= 1;
    data.sim.color_b *= 2;

    // Your changes will be visible immediately. :)
    
    let _ = renderer.set_draw_color(pixels::RGB(data.sim.color_r, data.sim.color_g, data.sim.color_b));
    let _ = renderer.clear();
    renderer.present();
}
