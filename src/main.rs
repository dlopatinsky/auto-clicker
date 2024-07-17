use clap::{Parser, ValueEnum};
use inputbot::{get_keybd_key, MouseButton};
use std::thread;
use std::time::Duration;

fn main() {
    let args = Args::parse();
    if let Some(key_bind) = get_keybd_key(args.key_bind) {
        let mouse_button = match args.mouse_button {
            MouseButtons::LeftButton => MouseButton::LeftButton,
            MouseButtons::RightButton => MouseButton::RightButton,
            MouseButtons::MiddleButton => MouseButton::MiddleButton,
            MouseButtons::X1Button => MouseButton::X1Button,
            MouseButtons::X2Button => MouseButton::X2Button,
        };
        loop {
            if key_bind.is_pressed() {
                mouse_button.press();
                thread::sleep(Duration::from_millis(args.release_pause));
                mouse_button.release();
                thread::sleep(Duration::from_millis(args.pause));
            }
        }
    } else {
        eprintln!("Invalid key bind character");
    }
}

#[derive(Parser, Debug)]
struct Args {
    /// Key to bind the clicking to (as a latin character).
    #[arg(long, short, default_value_t = 'c')]
    key_bind: char,
    /// Pause between clicks in milliseconds.
    #[arg(long, short, default_value_t = 200)]
    pause: u64,
    /// Mouse click release pause in milliseconds.
    #[arg(long, short, default_value_t = 50)]
    release_pause: u64,
    /// Mouse button to be clicked.
    #[arg(long, short, default_value_t, value_enum)]
    mouse_button: MouseButtons,
}

#[derive(ValueEnum, Clone, Default, Debug)]
enum MouseButtons {
    #[default]
    LeftButton,
    RightButton,
    MiddleButton,
    X1Button,
    X2Button,
}
