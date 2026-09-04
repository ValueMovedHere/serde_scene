use serde::Deserialize;

use crate::params::{
    CapsuleParams,
    ConeParams,
    CuboidParams, //
    CylinderParams,
    SphereParams,
};

#[derive(Debug, Deserialize)]
enum ShapeType {
    Cuboid(CuboidParams),
    Sphere(SphereParams),
    Cylinder(CylinderParams),
    Cone(ConeParams),
    Capsule(CapsuleParams),
}
