use std::{thread, time::Duration};
use enigo::{Enigo,  MouseControllable, };
fn main() {
    let mut enigo = Enigo::new();
    let mut status: f64 = 0.0;

    let static_x = 1920 as f64;
    let static_y = 1080 as f64;
    loop {
        status += 0.001;
        thread::sleep(Duration::from_millis(1));
        let x = static_x*0.5 + static_x*0.4 * status.sin();
        let y = static_y*0.5 + static_y*0.4 * status.cos();
        enigo.mouse_move_to(x as i32, y as i32);
    }
}
