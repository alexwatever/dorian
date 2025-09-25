use bevy::{prelude::*, state::state::FreelyMutableState};

// Modules
use crate::components::button::{ButtonIndex, ButtonValues};

/// Current selection index for keyboard nav (0/1)
#[derive(Resource, Default, Debug)]
pub struct MenuSelection(usize);

impl MenuSelection {
    /// Get the current selection index
    pub fn index(&self) -> usize {
        self.0
    }

    /// Set the current selection index
    pub fn set_index(&mut self, index: usize) {
        self.0 = index;
    }
}

/// Start menu
#[derive(Component, Debug, Default)]
pub struct StartMenu;

/// Pause menu
#[derive(Component, Debug, Default)]
pub struct IngameMenu;

/// Trait for menu components
pub trait Menu {
    type MenuEntity: Component + Default;
    type MenuButton: Component + ButtonIndex + ButtonValues + std::fmt::Display + Clone;
    type State: States;

    /// Button colours
    const BUTTON_COLOUR: Color = Color::srgb(0.15, 0.15, 0.2);
    const BUTTON_SELECTED_COLOUR: Color = Color::srgb(0.30, 0.30, 0.45);

    /// Setup the menu
    fn setup(mut commands: Commands, mut selection: ResMut<MenuSelection>) {
        // Set the selection to the first button
        selection.set_index(0);

        // Spawn the menu node
        let mut menu: Entity = Self::spawn_menu(&mut commands);

        // Spawn the buttons
        for button in <Self as Menu>::MenuButton::values() {
            Self::spawn_button(&mut commands, &mut menu, button);
        }
    }

    /// Cleanup the menu
    fn cleanup(mut commands: Commands, nodes: Query<Entity, With<Self::MenuEntity>>) {
        // Despawn the menu nodes
        for node in &nodes {
            commands.entity(node).despawn_recursive();
        }
    }

    /// Spawn a menu node
    fn spawn_menu(commands: &mut Commands) -> Entity {
        // Menu attributes
        const MENU_WIDTH: Val = Val::Percent(100.0);
        const MENU_HEIGHT: Val = Val::Percent(100.0);
        const MENU_ALIGN_ITEMS: AlignItems = AlignItems::Center;
        const MENU_JUSTIFY_CONTENT: JustifyContent = JustifyContent::Center;
        const MENU_FLEX_DIRECTION: FlexDirection = FlexDirection::Column;

        commands
            .spawn((
                <Self as Menu>::MenuEntity::default(),
                Node {
                    width: MENU_WIDTH,
                    height: MENU_HEIGHT,
                    justify_content: MENU_JUSTIFY_CONTENT,
                    align_items: MENU_ALIGN_ITEMS,
                    flex_direction: MENU_FLEX_DIRECTION,
                    ..default()
                },
            ))
            .id()
    }

    /// Spawn a button node
    fn spawn_button(commands: &mut Commands, menu: &mut Entity, button_type: Self::MenuButton) {
        // Button attributes
        const BUTTON_WIDTH: Val = Val::Px(240.0);
        const BUTTON_HEIGHT: Val = Val::Px(44.0);
        const BUTTON_MARGIN: Val = Val::Px(8.0);
        const BUTTON_FONT_SIZE: f32 = 28.0;

        // Create the button node
        let button_node = Node {
            width: BUTTON_WIDTH,
            height: BUTTON_HEIGHT,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            margin: UiRect::all(BUTTON_MARGIN),
            ..default()
        };

        // Build the button
        let button: (Button, Self::MenuButton, Node, BackgroundColor) = (
            Button,
            button_type.clone(),
            button_node,
            BackgroundColor(Self::BUTTON_COLOUR),
        );

        // Build the button contents
        let button_contents: (Text, TextFont) = (
            Text::new(button_type.to_string()),
            TextFont {
                font_size: BUTTON_FONT_SIZE,
                ..default()
            },
        );

        commands.entity(*menu).with_children(|parent| {
            parent.spawn(button).with_children(|parent| {
                parent.spawn(button_contents);
            });
        });
    }

    /// Handle keyboard input
    fn keyboard_input(
        keys: Res<ButtonInput<KeyCode>>,
        selection: ResMut<MenuSelection>,
        next_state: ResMut<NextState<Self::State>>,
        exit: EventWriter<AppExit>,
    ) where
        Self::State: FreelyMutableState;

    /// Handle mouse input
    fn mouse_input(
        interactions: Query<(&Interaction, &Self::MenuButton), Changed<Interaction>>,
        selection: ResMut<MenuSelection>,
        next_state: ResMut<NextState<Self::State>>,
        exit: EventWriter<AppExit>,
    ) where
        Self::State: FreelyMutableState;

    /// Update the visuals for the menu buttons
    fn update_visuals(
        selection: Res<MenuSelection>,
        mut nodes: Query<(&Self::MenuButton, &mut BackgroundColor)>,
    ) {
        for (button, mut background) in &mut nodes {
            // Check if the button is selected
            let is_selected: bool = selection.index() == button.index();

            // Set the background color
            background.0 = if is_selected {
                Self::BUTTON_SELECTED_COLOUR
            } else {
                Self::BUTTON_COLOUR
            };
        }
    }
}
