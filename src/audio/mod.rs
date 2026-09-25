use std::fs::File;
use std::io::BufReader;

use bevy::prelude::*;
use serde_json;

pub fn from_json(
    path: &str,
    audio_player: AudioPlayer,
    playback_settings: PlaybackSettings,
) -> Vec<(Transform, AudioPlayer, PlaybackSettings)> {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let pos_data_vec: Vec<(f32, f32, f32)> = serde_json::from_reader(reader).unwrap();
    let mut sound_data_vec = Vec::with_capacity(1usize);
    // 假设这些位置对应的是同一个音频, 且具有相同的播放设置
    for item in pos_data_vec {
        sound_data_vec.push((
            Transform::from_xyz(item.0, item.1, item.2),
            audio_player.clone(),
            playback_settings,
        ));
    }
    sound_data_vec
}
