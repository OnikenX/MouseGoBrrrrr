use enigo::{Enigo, MouseControllable};
use std::{thread, time::Duration};
fn main() {
    let mut enigo = Enigo::new();
    let mut status: f64 = 0.0;
    let value = 25.0;
    loop {
        enigo.mouse_move_relative((value * status.sin()) as i32, (value * status.cos()) as i32);
        thread::sleep(Duration::from_millis(20));
        status += 0.4;
    }
}
