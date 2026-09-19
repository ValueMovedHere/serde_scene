use avian3d::{
    math::Quaternion, //
    prelude::{
        Collider, //
        RigidBody,
        Sensor,
    },
};
use bevy::prelude::{Transform, Vec3};

use crate::collider::{
    a_collider_from, //
    shapes::ShapeType,
};

pub fn from_json(path: &str) -> (Collider, RigidBody, Sensor, Transform) {
    // TODO: 后续需要正确解析位置, 旋转等数据并以合适的形式返回, 现在还不能直接使用
    let collider_data = a_collider_from(path);
    // 从 ColliderData 构造 Collider
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
    let position = Vec3::from(collider_data.position);
    let rotation = Quaternion::from_xyzw(
        collider_data.rotation.0,
        collider_data.rotation.1,
        collider_data.rotation.2,
        collider_data.rotation.3,
    );
    let transform = Transform {
        translation: position,
        rotation,
        ..Default::default()
    };
    // 先支持静态刚体
    (collider, RigidBody::Static, Sensor, transform)
}
