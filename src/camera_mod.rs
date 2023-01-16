use std::f32::consts::PI;

use crate::vector_math::Vector3;

pub struct Camera {
    pub pos: Vector3,
    pub rot: Vector3,
}
impl Camera {
    pub fn move_pos(&mut self, direction: Vector3, dist: f32) {
        let x = direction.x.cos() * direction.y.sin();
        let z = direction.x.cos() * direction.y.cos();
        let y = direction.x.sin();
        self.pos.x += x * dist;
        self.pos.y += y * dist;
        self.pos.z += z * dist;
    }
    pub fn control(&mut self, direction: Vector3) {
        self.pos.y += direction.y;
        self.move_pos(self.rot, direction.z);
        self.move_pos(Vector3::new(0.0, self.rot.y + PI / 2.0, 0.0), direction.x);
    }
}
