use bevy::{
    core::FixedTimestep,
    prelude::*,
    render::pass::ClearColor,
    sprite::collide_aabb::{collide, Collision},
};
use crate::game::{AppState,TIME_STEP};

pub struct MenuPlugin;

pub struct MainMenuMarker;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_enter(AppState::Menu)
                .with_system(main_menu_setup.system())
                )
        .add_system_set(
            SystemSet::on_update(AppState::Menu)
                .with_system(go_to_world.system()),
                )
        .add_system_set(
            SystemSet::on_exit(AppState::Menu)
                .with_system(leave.system())
                );
    }
}

fn main_menu_setup (
    mut commands: Commands, 
    asset_server: Res<AssetServer>
    ) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(MainMenuMarker);
    commands.spawn_bundle(Text2dBundle {
        text: Text::with_section(
            "Would you like to play a game?",
            TextStyle {
                font: asset_server.load("fonts/tiny.ttf"),
                font_size: 100.0,
                color: Color::WHITE,
            },
            TextAlignment {
                vertical: VerticalAlign::Center,
                horizontal: HorizontalAlign::Center,
            },
        ),
        ..Default::default()
    })
        .insert(MainMenuMarker);
}

fn go_to_world(
    keyboard_input: Res<Input<KeyCode>>,
    mut state: ResMut<State<AppState>>
    ) {
    if keyboard_input.pressed(KeyCode::Return){
        state.set(AppState::Overworld).unwrap();
    }
}

fn leave(
    mut commands: Commands, 
    main_menu_query: Query<Entity, With<MainMenuMarker>>
    ) {
    for ent in main_menu_query.iter() {
        commands.entity(ent).despawn_recursive();
    }
}
