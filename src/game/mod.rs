use bevy::prelude::*;
use overworld::OverworldPlugin;

mod overworld;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(OverworldPlugin)
            .add_state(AppState::Overworld);
    }
}

const TIME_STEP: f32 = 1.0 / 60.0;
pub struct Position(Transform);

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState{
    Load,
    Overworld,
    Combat,
    Chat,
    Menu,
}


struct Person{
    speed: f32,
    mc: bool,
}