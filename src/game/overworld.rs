use bevy::{
    core::FixedTimestep,
    prelude::*,
    render::pass::ClearColor,
    sprite::collide_aabb::{collide, Collision},
};
use crate::game::{AppState,MusicTrack, TIME_STEP,Person,Position};
use crate::game::audio::music_setup;

pub struct OverworldPlugin;

impl Plugin for OverworldPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_enter(AppState::Overworld)
                .with_system(setup.system())
                )
        .add_system_set(
            SystemSet::on_update(AppState::Overworld)
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(movement_person.system())
                .with_system(move_camera.system()),
                );
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    audio: Res<Audio>, 
    mut state: ResMut<State<AppState>>,
    mut track: ResMut<State<MusicTrack>>,

) {
    // Spawns in the camera
    commands
        .spawn()
        .insert_bundle(OrthographicCameraBundle::new_2d())
        .insert(Position(Transform::from_translation(Vec3::new(
            0.0, 0.0, 1000.0,
        ))));
    // Spawns in the overworld image
    let asset_handler = materials.add(asset_server.load("Overworld.png").into());
    commands.spawn().insert_bundle(SpriteBundle {
        material: asset_handler.clone(),
        ..Default::default()
    });
    // Spawns in wes
    let sprite_handler = materials.add(asset_server.load("wes.png").into());
    commands.spawn().insert_bundle(SpriteBundle {
        material: sprite_handler.clone(),
        sprite: Sprite::new(Vec2::new(80.0, 134.0)),
        ..Default::default()
    })
    .insert( Person {speed: 50.0, mc: true});
    track.set(MusicTrack::Overworld);
    
    music_setup(asset_server, audio, track);
}

fn movement<'a>(
    keyboard_input: Res<'a, Input<KeyCode>>,
    transform: &mut Transform,
    init_speed: f32
) -> Res<'a, Input<KeyCode>> {
    // Direction and speed init
    let mut vel = Vec3::new(0.0,0.0,0.0);
    let speed;

    // If speedup is held, double the inital speed
    if keyboard_input.pressed(KeyCode::LShift){
        speed = 2.0 * init_speed;
    }
    else{
        speed = init_speed;
    }
    
    // Move in the specified direction
    if keyboard_input.pressed(KeyCode::W) {
        vel.y = 1.0;
    }
    else if keyboard_input.pressed(KeyCode::S) {
        vel.y = -1.0;
    }
    if keyboard_input.pressed(KeyCode::A) {
        vel.x = -1.0;
    }
    else if keyboard_input.pressed(KeyCode::D) {
        vel.x = 1.0;
    }

    // Apply the changes
    let translation = &mut transform.translation;
    translation.x += vel.x * speed * TIME_STEP;
    translation.y += vel.y * speed * TIME_STEP;
    keyboard_input

}
fn movement_person(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Person, &mut Transform)>,
) {
    // Finds the person to move and moves them
    if let Ok((mut _person, mut transform)) = query.single_mut() {
        movement(keyboard_input, &mut transform, 50.0);
    }
}

fn move_camera(
    mut keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &mut Position)>,
) {
    // Moves all cameras
    for (mut transform, mut _position) in query.iter_mut() {
        keyboard_input = movement(keyboard_input, &mut transform, 50.0);
    }
}
