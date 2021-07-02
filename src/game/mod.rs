use bevy::prelude::*;
use overworld::OverworldPlugin;
use menu::MenuPlugin;

mod overworld;
mod menu;
mod audio;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(OverworldPlugin)
            .add_state(AppState::Menu)
            .add_state(MusicTrack::Overworld)
            .add_plugin(MenuPlugin);
    }
}

const TIME_STEP: f32 = 1.0 / 60.0;
pub struct Position(Transform);

// The different states the game can be in
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState{
    Load,
    Overworld,
    Combat,
    Chat,
    Menu,
}


#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum MusicTrack{
    Overworld,
}

// Struct to indicate different characters
struct Person{
    speed: f32,
    mc: bool,
}
