//! A view is a object where all input states are stored. It also has useful methods such checking
//! if a key or axis for a [`BindingTypeView`] is pressed or released by proving the [`PressState`].
use std::collections::HashMap;

use bevy::prelude::Component;

use crate::prelude::*;

/// Agnostic type for representing a input source (e.g. keyboard, mouse, gamepad).
#[derive(PartialEq, Eq, Hash, Debug, Clone, Component, Copy)]
pub enum InputSource {
    Gamepad,
    Keyboard,
    Mouse,
}

impl InputSource {
    /// Returns whether this input source is referent to a gamepad.
    fn is_gamepad(&self) -> bool {
        return self == &InputSource::Gamepad;
    }

    /// Returns whether this input source is referent to a keyboard.
    fn is_keyboard(&self) -> bool{
        return self == &InputSource::Keyboard;
    }

    /// Returns whether this input source is referent to a mouse.
    fn is_mouse(&self) -> bool {
        return self == &InputSource::Mouse;
    }
}

/// The current axis state. In other words, the strength (how much the axis is moved) and press state.
#[derive(PartialEq, Clone, Copy, Debug)]
pub struct AxisState(pub f32, pub PressState);

/// A view is a object where all input states are stored. It also has useful methods such checking
/// if a key or axis for a [`BindingTypeView`] is pressed or released by proving the [`PressState`].
#[derive(PartialEq, Clone, Debug, Component, Default)]
pub struct InputView<Keys>
where
    Keys: BindingTypeView,
{
    pub last_input_source: Option<InputSource>,
    pub bindings: HashMap<Keys, ActionBinding<Keys>>,
    pub key_receiver_states: HashMap<BindingInputReceiver, PressState>,
    pub axis_receiver_states: HashMap<BindingInputReceiver, AxisState>,
    pub receiver_default_axis_values: HashMap<BindingInputReceiver, f32>,
}

impl<Keys> InputView<Keys>
where
    Keys: BindingTypeView,
{
    /// Create an empty input view.
    pub fn empty() -> Self {
        Self {
            last_input_source: None,
            bindings: HashMap::new(),
            key_receiver_states: HashMap::new(),
            axis_receiver_states: HashMap::new(),
            receiver_default_axis_values: HashMap::new(),
        }
    }

    /// Insert a new binding into the storage.
    pub fn add_binding(&mut self, binding: &ActionBinding<Keys>) -> &mut Self {
        binding.apply_default_axis_to_all_receivers(self);
        self.bindings.insert(binding.key, binding.clone());
        self
    }

    /// Add a default axis value to a receiver.
    pub fn add_receiver_default_axis_values(
        &mut self,
        receiver: BindingInputReceiver,
        value: f32,
    ) -> &mut Self {
        self.receiver_default_axis_values.insert(receiver, value);
        self
    }

    /// Returns the default axis value for a receiver, or `1` if not found.
    pub fn get_receiver_default_axis_value(&self, receiver: BindingInputReceiver) -> f32 {
        self.receiver_default_axis_values
            .get(&receiver)
            .unwrap_or(&1.)
            .clone()
    }

    /// Set the button state for a specific key receiver.
    pub fn set_key_receiver_state(&mut self, key: BindingInputReceiver, state: PressState) {
        self.key_receiver_states.insert(key, state);
    }

    /// Set the axis state for a specific axis receiver.
    pub fn set_axis_value(
        &mut self,
        receiver: BindingInputReceiver,
        value: f32,
        element_state: PressState,
    ) {
        self.axis_receiver_states
            .insert(receiver, AxisState(value, element_state));
    }

    /// Return the current press state for a specific binding matching with the given BindingTypeView.
    pub fn key(&self, kind: &Keys) -> PressState {
        let binding = self.bindings.get(kind);
        if let Some(binding) = binding {
            'initial: for r in binding.input_receivers.iter() {
                let states: Vec<PressState> =
                    r.0.iter()
                        .map(|x| {
                            self.key_receiver_states
                                .get(x)
                                .unwrap_or(&PressState::Released)
                                .clone()
                        })
                        .collect();
                if states.is_empty() {
                    continue 'initial;
                }
                for state in states.iter() {
                    if state.released() {
                        continue 'initial;
                    }
                }
                return states.iter().min().unwrap().clone();
            }
        }
        PressState::Released
    }

    /// Return the current axis state for a specific binding matching with the given BindingTypeView.
    pub fn axis(&self, kind: &Keys) -> Vec<AxisState> {
        let binding = self.bindings.get(kind);
        if let Some(binding) = binding {
            'initial: for r in binding.input_receivers.iter() {
                let states: Vec<AxisState> =
                    r.0.iter()
                        .map(|x| {
                            self.axis_receiver_states
                                .get(x)
                                .unwrap_or(&AxisState(0., PressState::Released))
                                .clone()
                        })
                        .collect();
                if states.is_empty() {
                    continue 'initial;
                }
                for state in states.iter() {
                    if state.1.released() {
                        continue 'initial;
                    }
                }
                return states;
            }
        }
        Vec::new()
    }

    /// Change the value of the last active input source.
    pub fn set_last_input_source(&mut self, input_source: Option<InputSource>) {
        self.last_input_source = input_source;
    }
}
