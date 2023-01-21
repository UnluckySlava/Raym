use std::collections::HashMap;
use crate::vector_math::Vector3;
macro_rules! impl_object_for {
    ($($t:ty),+) => {$(impl Object for $t {})+};
}

pub struct Scene {
    pub objects: HashMap<String, Box<dyn Object>>
}

pub trait Object {
}
pub struct Sphere {
    pub pos: Vector3,
    pub radius: f32
}
pub struct Plane {
    pub normal: Vector3,
    pub dist_from_origin: f32
}
pub struct Cuboid {
    pub pos: Vector3,
    pub dimensions: Vector3
}
impl_object_for!(Sphere, Plane, Cuboid);
