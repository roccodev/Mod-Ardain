use std::ops::{Add, BitOr};

#[derive(Clone, Copy, Debug)]
pub struct PadData {
    buttons: u64,
}

/// Monolith Soft's controller-agnostic button IDs
#[derive(Clone, Copy, Debug)]
#[allow(unused)]
pub enum PadButton {
    A = 4,                          // nn bit: 0x00
    B = 2,                          // nn: 0x01
    X = 8,                          // nn: 0x02
    Y = 1,                          // nn: 0x03
    L = 0x10,                       // nn: 0x06
    R = 0x20,                       // nn: 0x07
    ZL = 0x40,                      // nn: 0x08
    ZR = 0x80,                      // nn: 0x09
    DpadRight = 0x2000,             // nn: 0x0e
    DpadLeft = 0x8000,              // nn: 0x0c
    DpadUp = 0x1000,                // nn: 0x0d
    DpadDown = 0x4000,              // nn: 0x0f
    LeftStickClick = 0x400,         // nn: 0x04
    RightStickClick = 0x800,        // nn: 0x05
    Plus = 0x200,                   // nn: 0x0a
    Minus = 0x100,                  // nn: 0x0b
    LeftSL = 0x80000,               // nn: 0x18
    LeftSR = 0x100000,              // nn: 0x19
    RightSL = 0x200000,             // nn: 0x1a
    RightSR = 0x400000,             // nn: 0x1b
    LeftStickRight = 0x1_000_000,   // nn: 0x12
    LeftStickUp = 0x800_000,        // nn: 0x11
    LeftStickLeft = 0x4_000_000,    // nn: 0x10
    LeftStickDown = 0x2_000_000,    // nn: 0x13
    RightStickRight = 0x10_000_000, // nn: 0x16
    RightStickUp = 0x8_000_000,     // nn: 0x15
    RightStickLeft = 0x40_000_000,  // nn: 0x14
    RightStickDown = 0x20_000_000,  // nn: 0x17
}

impl PadData {
    pub fn contains<P: Into<PadData>>(&self, other: P) -> bool {
        let other = other.into();
        self.buttons & other.buttons == other.buttons
    }

    pub fn is_empty(&self) -> bool {
        self.buttons == 0
    }
}

impl PadButton {
    /// Returns the button combination required to execute an "emergency escape", i.e. an instant
    /// death for the party.
    ///
    /// See: <https://tcrf.net/Xenoblade_Chronicles_2#Insta-kill_Button_Combination>
    /// Based on function `gf::GfGamePad::checkEmergencyEscape` (0x003267c8 in the base executable)
    pub fn emergency_escape() -> PadData {
        use PadButton::*;
        X + DpadDown + ZL + ZR + L + R
    }
}

impl From<u64> for PadData {
    fn from(buttons: u64) -> Self {
        Self { buttons }
    }
}

impl Add<PadButton> for PadData {
    type Output = Self;

    fn add(self, rhs: PadButton) -> Self::Output {
        Self {
            buttons: self.buttons + (rhs as u64),
        }
    }
}

impl Add for PadButton {
    type Output = PadData;

    fn add(self, rhs: Self) -> Self::Output {
        PadData {
            buttons: self as u64 + rhs as u64,
        }
    }
}

impl BitOr<PadButton> for PadData {
    type Output = Self;

    fn bitor(self, rhs: PadButton) -> Self::Output {
        self.add(rhs)
    }
}

impl BitOr for PadButton {
    type Output = PadData;

    fn bitor(self, rhs: Self) -> Self::Output {
        self.add(rhs)
    }
}

impl From<PadButton> for PadData {
    fn from(button: PadButton) -> Self {
        Self {
            buttons: button as u64,
        }
    }
}

impl Default for PadData {
    fn default() -> Self {
        Self {
            buttons: Default::default(),
        }
    }
}
