use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct CuboidParams {
    width: f32,
    height: f32,
    depth: f32,
}

#[derive(Debug, Deserialize)]
pub(crate) struct SphereParams {
    radius: f32,
}

#[derive(Debug, Deserialize)]
pub(crate) struct CylinderParams {
    radius: f32,
    height: f32,
}

#[derive(Debug, Deserialize)]
pub(crate) struct ConeParams {
    radius: f32,
    height: f32,
}

#[derive(Debug, Deserialize)]
pub(crate) struct CapsuleParams {
    radius: f32,
    length: f32,
}
