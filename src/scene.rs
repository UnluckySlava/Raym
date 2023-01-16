use crate::vector_math::Vector3;

pub struct Scene {
    objects: Vec<Object>
}
pub enum Object {
    Sphere {pos:Vector3, radius: f32},
    Plane {normal: Vector3, dist_from_origin: f32},
    Box {dimensions: Vector3}
}

pub struct Sphere {
    pos: Vector3,
    radius: f32
}
pub struct Plane {
    normal: Vector3,
    dist_from_origin: f32
}
pub struct Box {
    dimensions: Vector3,
    pos: Vector3
}
