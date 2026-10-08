use std::fs::File;
use std::io::BufReader;

use bevy::prelude::*;
use serde_json;

use crate::errors::SceneError;

type SoundData = (Transform, AudioPlayer, PlaybackSettings);

pub fn from_json(
    path: &str,
    audio_player: AudioPlayer,
    playback_settings: PlaybackSettings,
) -> Result<Vec<SoundData>, SceneError> {
    let file = File::open(path).map_err(|e| SceneError::NotFound {
        path: path.to_string(),
        source: e,
    })?;
    let reader = BufReader::new(file);
    let pos_data_vec: Vec<(f32, f32, f32)> =
        serde_json::from_reader(reader).map_err(|e| SceneError::ParseError {
            file_path: path.to_string(),
            source: e,
        })?;
    let mut sound_data_vec = Vec::with_capacity(1usize);
    // 假设这些位置对应的是同一个音频, 且具有相同的播放设置
    for item in pos_data_vec {
        sound_data_vec.push((
            Transform::from_xyz(item.0, item.1, item.2),
            audio_player.clone(),
            playback_settings,
        ));
    }
    Ok(sound_data_vec)
}
