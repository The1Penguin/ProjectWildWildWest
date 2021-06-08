use bevy::{
    core::FixedTimestep,
    prelude::*,
    render::pass::ClearColor,
    sprite::collide_aabb::{collide, Collision},
};
const TIME_STEP: f32 = 1.0 / 60.0;

fn main() {
    let window = WindowDescriptor {
        title: "Forrest".to_string(),
        width: 1920.0,
        height: 1080.0,
        vsync: true,
        resizable: false,
        ..Default::default()
    };



    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_system(bevy::input::system::exit_on_esc_system.system())
        .insert_resource(window)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(movement_person.system()),
                )
        .run();
}

struct Person{
    speed: f32,
}

struct World{
    x: f32,
    y: f32,
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    let asset_handler = materials.add(asset_server.load("Overworld.png").into());
    commands.spawn().insert_bundle(SpriteBundle {
        material: asset_handler.clone(),
        ..Default::default()
    })
    .insert( World {x: 50.0, y: 50.0});
    let sprite_handler = materials.add(asset_server.load("wes.png").into());
    commands.spawn().insert_bundle(SpriteBundle {
        material: sprite_handler.clone(),
        sprite: Sprite::new(Vec2::new(80.0, 134.0)),
        ..Default::default()
    })
    .insert( Person {speed: 50.0});
}

fn movement_person(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Person, &mut Transform)>,
) {
    if let Ok((mut Person, mut transform)) = query.single_mut() {
        let mut vel = Vec3::new(0.0,0.0,0.0);
        Person.speed = 50.0;
        if keyboard_input.pressed(KeyCode::LShift){
            Person.speed = 100.0;
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
        translation.x += vel.x * Person.speed * TIME_STEP;
        translation.y += vel.y * Person.speed * TIME_STEP;
        
    }

}
