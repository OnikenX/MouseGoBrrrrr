use enigo::{Enigo, MouseButton, MouseControllable};
use std::{thread, time::Duration};
fn main() {
    let mut enigo = Enigo::new();

    let mut status: f64 = 0.0;
    let static_x = 40 as f64;
    let static_y = 40 as f64;
    loop {
        status += 0.95;
        thread::sleep(Duration::from_millis(200));
        let x = static_x * 0.5 + static_x * 0.4 * status.sin();
        let y = static_y * 0.5 + static_y * 0.4 * status.cos();
        enigo.mouse_move_relative(x as i32, y as i32);
        if status == 2.0 * std::f64::consts::PI {
            status = 0.0;
        }
    }
}
