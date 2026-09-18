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

use crate::collider::{self, ColliderData};

pub fn from_json(path: &str) -> (RigidBody, Collider, Sensor) {}
