use bevy::prelude::*;
use parse_display::Display;

// Modules
use crate::{
    components::menu::MenuSelection,
    error::{Error, ErrorLevel},
};

/// Start menu buttons
#[derive(Component, Clone, Copy, PartialEq, Eq, Display, Debug)]
pub enum StartMenuButton {
    #[display("Start")]
    Start,
    #[display("Quit")]
    Quit,
}

impl StartMenuButton {
    /// Start menu button values
    const VALUES: [Self; 2] = [Self::Start, Self::Quit];
}

impl TryFrom<ResMut<'_, MenuSelection>> for StartMenuButton {
    type Error = Error;

    /// Try to convert a menu selection to a start menu button
    fn try_from(selection: ResMut<'_, MenuSelection>) -> Result<Self, Error> {
        // Match the selection index
        match selection.index() {
            // Start game
            0 => Ok(StartMenuButton::Start),
            // Quit game
            1 => Ok(StartMenuButton::Quit),
            // Invalid selection
            _ => Err(Error::new(
                ErrorLevel::Error,
                "Invalid menu selection",
                None,
            )),
        }
    }
}

/// Pause menu buttons
#[derive(Component, Clone, Copy, PartialEq, Eq, Display, Debug)]
pub enum IngameMenuButton {
    #[display("Resume")]
    Resume,
    #[display("Quit")]
    Quit,
}

impl IngameMenuButton {
    /// Start menu button values
    const VALUES: [Self; 2] = [Self::Resume, Self::Quit];
}

impl TryFrom<ResMut<'_, MenuSelection>> for IngameMenuButton {
    type Error = Error;

    /// Try to convert a menu selection to a in-game menu button
    fn try_from(selection: ResMut<'_, MenuSelection>) -> Result<Self, Error> {
        // Match the selection index
        match selection.index() {
            // Resume game
            0 => Ok(IngameMenuButton::Resume),
            // Quit game
            1 => Ok(IngameMenuButton::Quit),
            // Invalid selection
            _ => Err(Error::new(
                ErrorLevel::Error,
                "Invalid in-game menu selection",
                None,
            )),
        }
    }
}

/// Trait for button indices
pub trait ButtonIndex {
    /// Get the index for a button
    fn index(&self) -> usize;
}

impl ButtonIndex for StartMenuButton {
    /// Get the index for a start menu button
    fn index(&self) -> usize {
        match self {
            StartMenuButton::Start => 0,
            StartMenuButton::Quit => 1,
        }
    }
}

impl ButtonIndex for IngameMenuButton {
    /// Get the index for a in-game menu button
    fn index(&self) -> usize {
        match self {
            IngameMenuButton::Resume => 0,
            IngameMenuButton::Quit => 1,
        }
    }
}

/// Trait for button values
pub trait ButtonValues {
    /// Get the values for a button
    fn values() -> Vec<Self>
    where
        Self: Sized;
}

impl ButtonValues for StartMenuButton {
    /// Get the values for a start menu button
    fn values() -> Vec<Self> {
        Self::VALUES.to_vec()
    }
}

impl ButtonValues for IngameMenuButton {
    /// Get the values for a in-game menu button
    fn values() -> Vec<Self> {
        Self::VALUES.to_vec()
    }
}
