use serde::Deserialize;

use super::params::{
    CapsuleParams,
    ConeParams,
    CuboidParams, //
    CylinderParams,
    SphereParams,
};

#[derive(Debug, Deserialize, Clone)]
pub(crate) enum ShapeType {
    Cuboid(CuboidParams),
    Sphere(SphereParams),
    Cylinder(CylinderParams),
    Cone(ConeParams),
    Capsule(CapsuleParams),
}
