pub mod area_light;
pub mod point_light;

use crate::core::vec2;
use crate::core::vec3;
use crate::scene::world;

pub trait Light {
    fn num_samples(&self) -> u32;

    fn sample_li(
        &self,
        sample: &vec2::Vec2,
        world: &world::World,
        surface_point: &vec3::Vec3,
        surface_normal: &vec3::Vec3,
        wi: &mut vec3::Vec3,
    ) -> vec3::Vec3;
}
