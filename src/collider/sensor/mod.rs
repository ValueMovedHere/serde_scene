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

use crate::collider::{self, ColliderData, a_collider_from};

pub fn from_json(path: &str) -> (RigidBody, Collider, Sensor) {
    let collider = a_collider_from(path);
    (RigidBody::Static, collider, Sensor)
}
