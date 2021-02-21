pub mod area_light;
pub mod point_light;

use crate::core::vec2;
use crate::core::vec3;
use crate::scene::world;

pub struct SampleLightRadiance {
    pub wi: vec3::Vec3,
    pub li: vec3::Vec3,
}

impl SampleLightRadiance {
    pub fn new(wi: vec3::Vec3, li: vec3::Vec3) -> SampleLightRadiance {
        return SampleLightRadiance { wi, li };
    }
}

pub trait Light {
    fn num_samples(&self) -> u32;

    fn sample_li(
        &self,
        sample: &vec2::Vec2,
        world: &world::World,
        surface_point: &vec3::Vec3,
        surface_normal: &vec3::Vec3,
    ) -> Option<SampleLightRadiance>;

    fn sample_li_no_shadow_check(
        &self,
        sample: &vec2::Vec2,
        world: &world::World,
        surface_point: &vec3::Vec3,
        surface_normal: &vec3::Vec3,
    ) -> Option<SampleLightRadiance>;
}
