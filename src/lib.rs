//! # Keybind
//!
//! Wrapper around [device_query](https://github.com/ostrosco/device_query) providing a nicer API, allowing you to trigger
//! your logic on specific keybind.
//!
//! # Example
//!
//! ```ignore
//! use keybind::{Keybind, Keycode};
//!
//!fn main() {
//!    let mut keybind = Keybind::new(&[Keycode::LControl, Keycode::G]);
//!
//!    keybind.on_trigger(|| {
//!        println!("This will be printed when you press CTRL+G");
//!    });
//!
//!    keybind.wait();
//!}
//! ```
//!

use device_query::{DeviceQuery, DeviceState};
use std::mem;

pub use device_query::Keycode;

pub struct Keybind {
    device_state: DeviceState,
    pressed_keys: Vec<Keycode>,
    key_binds: Vec<Keycode>,
    on_trigger: Box<Fn()>,
}

impl Keybind {
    /// Constructs a new `Keybind`.
    ///
    /// # Example
    ///
    /// ```ignore
    /// use keybind::{Keybind, Keycode};
    ///
    /// let mut keybind = Keybind::new(&[Keycode::LControl, Keycode::G]);
    /// ```
    pub fn new(keys: &[Keycode]) -> Keybind {
        Keybind {
            device_state: DeviceState::new(),
            pressed_keys: Vec::new(),
            key_binds: keys.to_vec(),
            on_trigger: Box::new(||{})
        }
    }

    /// Returns bool if the specific keybind has been triggered
    ///
    /// # Example
    ///
    /// ```ignore
    /// use keybind::{Keybind, Keycode};
    ///
    /// let mut keybind = Keybind::new(&[Keycode::LControl, Keycode::G]);
    ///
    /// loop {
    ///     if self.triggered() {
    ///         println!("triggered");
    ///     }
    /// }
    /// ```
    pub fn triggered(&mut self) -> bool {
        let previous_pressed_keys = mem::replace(
            &mut self.pressed_keys,
            self.device_state.get_keys()
        );

        self.pressed_keys.len() == self.key_binds.len()
            && previous_pressed_keys != self.pressed_keys
            && self.pressed_keys == self.key_binds
    }

    /// Sets provided callback that will be executed on trigger.
    ///
    /// # Example
    ///
    /// ```ignore
    /// use keybind::{Keybind, Keycode};
    ///
    /// let mut keybind = Keybind::new(&[Keycode::LControl, Keycode::G]);
    ///
    /// keybind.on_trigger(|| {
    ///     println!("This will be printed when you press CTRL+G");
    /// });
    /// ```
    pub fn on_trigger<C: Fn() + 'static>(&mut self, callback: C) {
        self.on_trigger = Box::new(callback);
    }

    /// Starts an infinite loop and calls provided callback when the keybind is triggered.
    ///
    /// # Example
    ///
    /// ```ignore
    /// use keybind::{Keybind, Keycode};
    ///
    ///fn main() {
    ///    let mut keybind = Keybind::new(&[Keycode::LControl, Keycode::G]);
    ///
    ///    keybind.on_trigger(|| {
    ///        println!("This will be printed when you press CTRL+G");
    ///    });
    ///
    ///    keybind.wait();
    ///}
    /// ```
    pub fn wait(&mut self) {
        loop {
            if self.triggered() {
                (self.on_trigger)();
            }
        }
    }
}
