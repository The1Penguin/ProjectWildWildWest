use bevy::{
    core::FixedTimestep,
    prelude::*,
    render::pass::ClearColor,
    sprite::collide_aabb::{collide, Collision},
};
const TIME_STEP: f32 = 1.0 / 60.0;
pub struct Position(Transform);

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum AppState{
    Load,
    Overworld,
    Combat,
    Chat,
    Menu,
}

fn main() {
    let window = WindowDescriptor {
        title: "Wild Wild Wes".to_string(),
        width: 1920.0,
        height: 1080.0,
        vsync: true,
        resizable: false,
        ..Default::default()
    };

    App::build()
        .add_plugins(DefaultPlugins)
        .add_state(AppState::Overworld)
        .add_system(bevy::input::system::exit_on_esc_system.system())
        .insert_resource(window)
        .add_system_set(
            SystemSet::on_enter(AppState::Overworld)
                .with_system(setup.system())
                )
        .add_system_set(
            SystemSet::on_update(AppState::Overworld)
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(movement_person.system())
                .with_system(move_camera.system()),
                )
        .run();
}

struct Person{
    speed: f32,
    mc: bool,
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands
        .spawn()
        .insert_bundle(OrthographicCameraBundle::new_2d())
        .insert(Position(Transform::from_translation(Vec3::new(
            0.0, 0.0, 1000.0,
        ))));

    let asset_handler = materials.add(asset_server.load("Overworld.png").into());
    commands.spawn().insert_bundle(SpriteBundle {
        material: asset_handler.clone(),
        ..Default::default()
    });
    let sprite_handler = materials.add(asset_server.load("wes.png").into());
    commands.spawn().insert_bundle(SpriteBundle {
        material: sprite_handler.clone(),
        sprite: Sprite::new(Vec2::new(80.0, 134.0)),
        ..Default::default()
    })
    .insert( Person {speed: 50.0, mc: true});
}

fn movement<'a>(
    keyboard_input: Res<'a, Input<KeyCode>>,
    transform: &mut Transform,
    init_speed: f32
) -> Res<'a, Input<KeyCode>> {
    let mut vel = Vec3::new(0.0,0.0,0.0);
    let speed;
    if keyboard_input.pressed(KeyCode::LShift){
        speed = 2.0 * init_speed;
    }
    else{
        speed = init_speed;
    }

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

    let translation = &mut transform.translation;
    translation.x += vel.x * speed * TIME_STEP;
    translation.y += vel.y * speed * TIME_STEP;
    keyboard_input

}
fn movement_person(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Person, &mut Transform)>,
) {
    if let Ok((mut _person, mut transform)) = query.single_mut() {
        movement(keyboard_input, &mut transform, 50.0);
    }
}

fn move_camera(
    mut keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &mut Position)>,
) {
    for (mut transform, mut _position) in query.iter_mut() {
        keyboard_input = movement(keyboard_input, &mut transform, 50.0);
    }
}
