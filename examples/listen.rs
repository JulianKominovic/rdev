use std::thread;

fn main() {
    // This will block.
    std::env::set_var("KEYBOARD_ONLY", "y");

    thread::spawn(move || {
        if let Err(error) = rdev::listen(|event| {
            println!("{:?}", event);
        }) {
            // rdev::listen
            dbg!("{:?}", error);
        }
    });

    thread::sleep(std::time::Duration::from_secs(100000));
}
