use crate::core::vec3;
use crate::scene::light;

pub struct RectangleLight {
    position: vec3::Vec3,
    normal: vec3::Vec3,
    distance: vec3::Vec3,
    width: f32,
    height: f32
}

impl RectangleLight {
    pub fn new(position: vec3::Vec3, normal: vec3::Vec3, distance: vec3::Vec3, width: f32, height: f32) -> RectangleLight {
        return RectangleLight{
            position,
            normal,
            distance,
            width,
            height
        };
    }
}

impl light::Light for RectangleLight {
    fn sample_li(
        &self,
        sampler: &mut dyn crate::scene::sampler::Sampler,
        world: &crate::scene::world::World,
        surface_point: &crate::core::vec3::Vec3,
        wi: &mut crate::core::vec3::Vec3,
    ) -> vec3::Vec3 {
        todo!()
    }
}