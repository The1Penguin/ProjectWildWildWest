use bevy::{
    core::FixedTimestep,
    prelude::*,
    render::pass::ClearColor,
    sprite::collide_aabb::{collide, Collision},
};
use crate::game::{AppState,MusicTrack};

pub fn music_setup(
    mut asset_server: Res<AssetServer>, 
    audio: Res<Audio>, 
    mut track: ResMut<State<MusicTrack>>, 
    ) {
    let music;
    
    match *track.current() {
        MusicTrack::Overworld => music = asset_server.load("First Steps.mp3"),
        _ => return,
    }
    println!("{:?}", music);
    audio.play(music);
}

