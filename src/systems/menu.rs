use bevy::prelude::*;

// Modules
use crate::{
    components::{
        button::{ButtonIndex, IngameMenuButton, StartMenuButton},
        menu::{IngameMenu, Menu, MenuSelection, StartMenu},
    },
    AppState, PauseState,
};

/// Handle the pause toggle
pub fn pause_toggle(
    keys: Res<ButtonInput<KeyCode>>,
    pause: Res<State<PauseState>>,
    mut next_pause: ResMut<NextState<PauseState>>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        match pause.get() {
            PauseState::Running => next_pause.set(PauseState::Paused),
            PauseState::Paused => next_pause.set(PauseState::Running),
        }
    }
}

impl Menu for StartMenu {
    type MenuEntity = StartMenu;
    type MenuButton = StartMenuButton;
    type State = AppState;

    /// Handle keyboard input for the main menu
    fn keyboard_input(
        keyboard_input: Res<ButtonInput<KeyCode>>,
        mut selection: ResMut<MenuSelection>,
        mut next_state: ResMut<NextState<Self::State>>,
        mut exit: EventWriter<AppExit>,
    ) {
        match keyboard_input.get_pressed().next() {
            // Move up
            Some(KeyCode::KeyW) | Some(KeyCode::ArrowUp) => {
                let index: usize = selection.index().saturating_sub(1);
                selection.set_index(index);
            }
            // Move down
            Some(KeyCode::KeyS) | Some(KeyCode::ArrowDown) => {
                let index: usize = (selection.index() + 1).min(1);
                selection.set_index(index);
            }
            // Select
            Some(KeyCode::Enter) | Some(KeyCode::NumpadEnter) => {
                // Determine the selected button
                selection
                    .try_into()
                    .map(|selected_button| match selected_button {
                        // Start game
                        StartMenuButton::Start => next_state.set(Self::State::InGame),
                        // Quit game
                        StartMenuButton::Quit => {
                            let _event_id = exit.send(AppExit::Success);
                        }
                    })
                    // Errors logged automatically within try_into
                    .ok();
            }
            _ => {
                debug!(
                    "Unhandled keyboard input: {:?}",
                    keyboard_input.get_pressed().next()
                );
            }
        }
    }

    /// Handle mouse input for the main menu
    fn mouse_input(
        mut interactions: Query<(&Interaction, &StartMenuButton), Changed<Interaction>>,
        mut selection: ResMut<MenuSelection>,
        mut next_state: ResMut<NextState<Self::State>>,
        mut exit: EventWriter<AppExit>,
    ) {
        for (interaction, button) in &mut interactions {
            // Check the mouse interaction type
            match *interaction {
                Interaction::Pressed => match button {
                    // Start game
                    StartMenuButton::Start => next_state.set(AppState::InGame),
                    // Quit game
                    StartMenuButton::Quit => {
                        let _event_id = exit.send(AppExit::Success);
                    }
                },
                Interaction::Hovered => {
                    // Set the selection index
                    selection.set_index(button.index());
                }
                Interaction::None => {}
            }
        }
    }
}

impl Menu for IngameMenu {
    type MenuEntity = IngameMenu;
    type MenuButton = IngameMenuButton;
    type State = PauseState;

    /// Handle keyboard input for the in-game menu
    fn keyboard_input(
        keys: Res<ButtonInput<KeyCode>>,
        mut selection: ResMut<MenuSelection>,
        mut next_pause: ResMut<NextState<PauseState>>,
        mut exit: EventWriter<AppExit>,
    ) {
        match keys.get_pressed().next() {
            // Move up
            Some(KeyCode::KeyW) | Some(KeyCode::ArrowUp) => {
                let index: usize = selection.index().saturating_sub(1);
                selection.set_index(index);
            }
            // Move down
            Some(KeyCode::KeyS) | Some(KeyCode::ArrowDown) => {
                let index: usize = (selection.index() + 1).min(1);
                selection.set_index(index);
            }
            // Select
            Some(KeyCode::Enter) | Some(KeyCode::NumpadEnter) => {
                // Determine the selected button
                selection
                    .try_into()
                    .map(|selected_button| match selected_button {
                        // Resume game
                        IngameMenuButton::Resume => next_pause.set(PauseState::Running),
                        // Quit game
                        IngameMenuButton::Quit => {
                            let _event_id = exit.send(AppExit::Success);
                        }
                    })
                    .ok();
            }
            _ => {
                debug!("Unhandled keyboard input: {:?}", keys.get_pressed().next());
            }
        }
    }

    /// Handle mouse input for the in-game menu
    fn mouse_input(
        mut interactions: Query<(&Interaction, &IngameMenuButton), Changed<Interaction>>,
        mut selection: ResMut<MenuSelection>,
        mut next_pause: ResMut<NextState<PauseState>>,
        mut exit: EventWriter<AppExit>,
    ) {
        for (interaction, button) in &mut interactions {
            // Check the mouse interaction type
            match *interaction {
                Interaction::Pressed => match button {
                    // Resume game
                    IngameMenuButton::Resume => next_pause.set(PauseState::Running),
                    // Quit game
                    IngameMenuButton::Quit => {
                        let _event_id = exit.send(AppExit::Success);
                    }
                },
                Interaction::Hovered => {
                    // Set the selection index
                    selection.set_index(button.index());
                }
                Interaction::None => {}
            }
        }
    }
}
