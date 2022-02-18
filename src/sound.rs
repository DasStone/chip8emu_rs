pub enum SoundMode {
    On,
    Off,
    Print,
}

pub fn get_sound_handler(mode: SoundMode) -> fn() {
    let implementation = match mode {
        SoundMode::On => sound,
        SoundMode::Off => no_sound,
        SoundMode::Print => print_sound,
    };

    implementation
}

fn print_sound() {
    println!("Beep");
}

fn no_sound() {}

fn sound() {
    todo!();
}