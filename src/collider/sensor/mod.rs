use std::fs::Fils;
use std::io::BufReader;

use avian3d::{
    math::Quaternion, //
    prelude::{
        Collider, //
        RigidBody,
        Sensor,
    },
};
use bevy::prelude::Vec3;
use serde::Deserialize;
use serde_json::from_reader;

pub fn from_json(path: &str) -> (RigidBody, Collider, Sensor) {}
