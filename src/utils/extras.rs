use crate::*;

// terminal screen functions
pub fn clear_screen() {
    print!("\x1B[2J");
    print!("\x1b[1;1H");
}
pub fn new_buffer() {
    print!("\x1b[?1049h");
}
pub fn exit_buffer() {
    print!("\x1b[?1049l");
}

// handle SIGINT
pub fn handle_ctrlc() {
    let mut signals = Signals::new([SIGINT]).unwrap();
    
    std::thread::spawn(move || {
        for sig in signals.wait() {
            if sig == 2 {
                exit_buffer();
                std::process::exit(0)
            }
        }
    });
}

