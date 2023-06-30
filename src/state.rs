use bevy::prelude::*;

#[derive(States, PartialEq, Eq, Debug, Clone, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    InGame,
}

pub fn check_state(state: Res<State<AppState>>) {
    info!("We are in the {:?} state", state.0);
}

pub fn start_game(
    keyboard_input: Res<Input<KeyCode>>,
    app_state: Res<State<AppState>>,
    mut app_state_next_state: ResMut<NextState<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::G) {
        if app_state.0 != AppState::InGame {
            app_state_next_state.set(AppState::InGame);
            println!("Entered AppState::Game");
        }
    }
}
