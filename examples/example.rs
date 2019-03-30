use keybind::{Keybind, Keycode};

fn main() {
    let mut keybind = Keybind::new(&[Keycode::LControl, Keycode::G]);

    keybind.on_trigger(|| {
        println!("This will be printed when you press CTRL+G");
    });

    keybind.wait();
}
