use bevy::prelude::States;

#[derive(Default, Hash, Debug, PartialEq, Eq, Clone, States)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,
}
