//! The press state for a button or axis. Also useful methods for checking the elapsed time.
use bevy::input::ElementState;
use bevy::utils::{Duration, Instant};
use std::ops::Add;

/// The press state for a button or axis.
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug, strum_macros::Display)]
pub enum PressState {
    /// The button or axis is pressed, along with the initial instant for the press.
    /// This need to be set as none if is the moment the button is just pressed, since it will
    /// let the input view know that the button is just pressed. The pressing instant is set
    /// in the next tick to allow users to know the pressing duration.
    Pressed {
        started_pressing_instant: Option<Instant>,
    },

    /// The button or axis is released.
    Released,
}

/// Main implementation for `PressState`.
/// This `impl` strives to make the API the simplest and cleaner possible, maintaing code reability.
impl PressState {
    /// Check if the current press state is released or not.
    pub fn released(&self) -> bool {
        return self == &PressState::Released;
    }

    /// Check if the current press state is pressed for more than a specific duration.
    pub fn pressed_for(&self, duration: Duration) -> bool {
        match self {
            PressState::Pressed {
                started_pressing_instant,
            } => {
                started_pressing_instant.is_some()
                    && started_pressing_instant.unwrap().elapsed() >= duration
            }
            _ => false,
        }
    }

    /// Check if the current press state was just pressed or not.
    pub fn just_pressed(&self) -> bool {
        match self {
            PressState::Pressed {
                started_pressing_instant,
            } => started_pressing_instant.is_none(),
            _ => false,
        }
    }

    /// Return the elapsed time since the action was pressed
    pub fn elapsed(&self) -> Option<Duration> {
        match self {
            PressState::Pressed {
                started_pressing_instant,
            } => {
                if let Some(started_pressing_instant) = started_pressing_instant {
                    Some(started_pressing_instant.elapsed())
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}

/// Implement partial comparision between press states.
impl PartialOrd for PressState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self {
            PressState::Pressed {
                started_pressing_instant: a,
            } => match other {
                PressState::Pressed {
                    started_pressing_instant: b,
                } => Some(a.cmp(b)),
                PressState::Released => Some(std::cmp::Ordering::Greater),
            },
            PressState::Released => match other {
                PressState::Pressed { .. } => Some(std::cmp::Ordering::Greater),
                PressState::Released => Some(std::cmp::Ordering::Equal),
            },
        }
    }
}

// Test to compare if `PartialOrd` is implemented correctly.
#[test]
fn partial_ord_press_state_test() {
    let a = PressState::Pressed {
        started_pressing_instant: Some(Instant::now()),
    };
    let b = PressState::Pressed {
        started_pressing_instant: Some(Instant::now().add(Duration::from_secs(342534))),
    };
    let value = a.cmp(&b);
    assert_eq!(value, std::cmp::Ordering::Less);
}

/// Implement comparison between press states.
impl Ord for PressState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

/// Implementation responsible for translating Bevy element states to EZInput press states.
/// By default, the default pressing instant is the None.
impl Into<PressState> for ElementState {
    fn into(self) -> PressState {
        match self {
            ElementState::Pressed => PressState::Pressed {
                started_pressing_instant: None,
            },
            ElementState::Released => PressState::Released,
        }
    }
}
