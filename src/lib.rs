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

use shapes::ShapeType;

pub fn from_json(path: &str) -> Vec<(Vec3, Quaternion, Collider)> {
    let file = File::open(path).expect("Failed to read colliders from json");
    let reader = BufReader::new(file);
    let colliders_raw: Vec<Data> = from_reader(reader).expect("Failed to parse json");
    let mut colliders_vec: Vec<(Vec3, Quaternion, Collider)> = Vec::with_capacity(1usize);
    for collider_data in colliders_raw.into_iter() {
        let collider = match collider_data.shape_type {
            ShapeType::Cuboid(params) => Collider::cuboid(
                params.width * collider_data.scale.0,
                params.height * collider_data.scale.1,
                params.depth * collider_data.scale.2,
            ),
            ShapeType::Sphere(params) => Collider::sphere(params.radius * collider_data.scale.0), // 缩放在某些情况下是不支持非均匀缩放的，所以在这些情况下直接统一使用 x 方向上的缩放
            ShapeType::Cylinder(params) => Collider::cylinder(
                params.radius * collider_data.scale.0,
                params.height * collider_data.scale.1,
            ),
            ShapeType::Cone(params) => Collider::cone(
                params.radius * collider_data.scale.0,
                params.height * collider_data.scale.1,
            ),
            ShapeType::Capsule(params) => Collider::capsule(
                params.radius * collider_data.scale.0,
                params.length * collider_data.scale.1,
            ),
        };
        let collider_tuple = (
            Vec3::new(
                collider_data.position.0,
                collider_data.position.1,
                collider_data.position.2,
            ),
            Quaternion::from_xyzw(
                collider_data.rotation.0,
                collider_data.rotation.1,
                collider_data.rotation.2,
                collider_data.rotation.3,
            ),
            collider,
        );
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
