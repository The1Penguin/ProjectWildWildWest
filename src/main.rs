use bevy::{
    core::FixedTimestep,
    prelude::*,
    window::WindowMode,
    render::pass::ClearColor,
    sprite::collide_aabb::{collide, Collision},
};
use game::GamePlugin;

mod game;

fn main() {
    let window = WindowDescriptor {
        title: "Wild Wild Wes".to_string(),
        width: 1920.0,
        height: 1080.0,
        vsync: true,
        resizable: false,
        mode: bevy::window::WindowMode::BorderlessFullscreen,
        ..Default::default()
    };

    App::build()
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(window)
        .add_plugins(DefaultPlugins)
        .add_system(bevy::input::system::exit_on_esc_system.system())
        .add_plugin(GamePlugin)
        .run();
}

