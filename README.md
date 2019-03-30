Keybind
=======

Wrapper around [device_query](https://github.com/ostrosco/device_query) providing a nicer API, allowing you to trigger
your logic on specific keybind.

Full Documentation can be read [here](https://docs.rs/keybind/*/keybind/).

```rust
use keybind::{Keybind, Keycode};

fn main() {
    let mut keybind = Keybind::new(&[Keycode::LControl, Keycode::G]);

    keybind.on_trigger(|| {
        println!("This will be printed when you press CTRL+G");
    });

    keybind.wait();
}
```
