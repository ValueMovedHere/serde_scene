use std::fs::File;
use std::io::BufReader;

use avian3d::{
    math::Quaternion, //
    prelude::Collider,
};
use bevy::prelude::Vec3;
use serde::Deserialize;
use serde_json::from_reader;

mod params;
mod shapes;

use params::{
    CapsuleParams, //
    ConeParams,
    CuboidParams,
    CylinderParams,
    SphereParams,
};
use shapes::ShapeType;

pub fn from_json(path: &str) -> Vec<(Vec3, Quaternion, Collider)> {
    let file = File::open(path).expect("Failed to read colliders from json");
    let reader = BufReader::new(file);
    let colliders_raw: Vec<Data> = from_reader(reader).expect("Failed to parse json");
    let mut colliders_vec: Vec<(Vec3, Quaternion, Collider)> = Vec::new();
    for collider in colliders_raw.into_iter() {
        let collider_tuple = ();
        colliders_vec.push(collider_tuple);
    }
    colliders_vec
}

#[derive(Deserialize, Debug)]
struct Data {
    shape_type: ShapeType,
    position: (f32, f32, f32),
    rotation: (f32, f32, f32, f32),
    scale: (f32, f32, f32),
}
