use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct CuboidParams {
    pub(crate) width: f32,
    pub(crate) height: f32,
    pub(crate) depth: f32,
}

#[derive(Debug, Deserialize)]
pub(crate) struct SphereParams {
    pub(crate) radius: f32,
}

#[derive(Debug, Deserialize)]
pub(crate) struct CylinderParams {
    pub(crate) radius: f32,
    pub(crate) height: f32,
}

#[derive(Debug, Deserialize)]
pub(crate) struct ConeParams {
    pub(crate) radius: f32,
    pub(crate) height: f32,
}

#[derive(Debug, Deserialize)]
pub(crate) struct CapsuleParams {
    pub(crate) radius: f32,
    pub(crate) length: f32,
}
