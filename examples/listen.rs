fn main() {
    // This will block.
    std::env::set_var("KEYBOARD_ONLY", "y");

    if let Err(error) = rdev::listen(|event| {
        println!("{:?}", event);
    }) {
        // rdev::listen
        dbg!("{:?}", error);
    }
}
