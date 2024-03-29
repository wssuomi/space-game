use bevy::prelude::*;

#[derive(States, PartialEq, Eq, Debug, Clone, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,
}

pub fn start_game(
    keyboard_input: Res<Input<KeyCode>>,
    app_state: Res<State<AppState>>,
    mut next_app_state: ResMut<NextState<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Return) {
        if app_state.0 != AppState::Game {
            next_app_state.set(AppState::Game);
        }
    }
}

pub fn go_to_main_menu(
    keyboard_input: Res<Input<KeyCode>>,
    app_state: Res<State<AppState>>,
    mut next_app_state: ResMut<NextState<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Return) {
        if app_state.0 != AppState::MainMenu {
            next_app_state.set(AppState::MainMenu);
        }
    }
}
