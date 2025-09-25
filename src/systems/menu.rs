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

    /// Setup the main menu
    fn setup(mut commands: Commands, mut selection: ResMut<MenuSelection>) {
        // Set the selection to the start button
        selection.set_index(0);

        // Spawn the menu node
        let mut menu: Entity = Self::spawn_menu(&mut commands, StartMenu);

        // Spawn the start button
        Self::spawn_button(&mut commands, &mut menu, StartMenuButton::Start);

        // Spawn the quit button
        Self::spawn_button(&mut commands, &mut menu, StartMenuButton::Quit);
    }

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
                let index = selection.index().saturating_sub(1);
                selection.set_index(index);
            }
            // Move down
            Some(KeyCode::KeyS) | Some(KeyCode::ArrowDown) => {
                let index = (selection.index() + 1).min(1);
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
                    StartMenuButton::Start => next_state.set(Self::State::InGame),
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

    /// Setup the in-game menu
    fn setup(mut commands: Commands, mut selection: ResMut<MenuSelection>) {
        // Set the selection to the resume button
        selection.set_index(0);

        // Spawn the menu node
        let mut menu: Entity = Self::spawn_menu(&mut commands, IngameMenu);

        // Spawn the resume button
        Self::spawn_button(&mut commands, &mut menu, IngameMenuButton::Resume);

        // Spawn the quit button
        Self::spawn_button(&mut commands, &mut menu, IngameMenuButton::Quit);
    }

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
                let index = selection.index().saturating_sub(1);
                selection.set_index(index);
            }
            // Move down
            Some(KeyCode::KeyS) | Some(KeyCode::ArrowDown) => {
                let index = (selection.index() + 1).min(1);
                selection.set_index(index);
            }
            // Select
            Some(KeyCode::Enter) | Some(KeyCode::NumpadEnter) => {
                selection
                    .try_into()
                    .map(|selected_button| match selected_button {
                        IngameMenuButton::Resume => next_pause.set(PauseState::Running),
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
