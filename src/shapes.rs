use serde::Deserialize;

use crate::params::{
    CapsuleParams,
    ConeParams,
    CuboidParams, //
    CylinderParams,
    SphereParams,
};

#[derive(Debug, Deserialize)]
pub(crate) enum ShapeType {
    Cuboid(CuboidParams),
    Sphere(SphereParams),
    Cylinder(CylinderParams),
    Cone(ConeParams),
    Capsule(CapsuleParams),
}
