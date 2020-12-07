pub mod point_light;

use crate::core::vec3;
use crate::scene::world;

pub trait Light {
    fn is_visible(&self, surface_point: &vec3::Vec3, world: &world::World) -> bool;

    fn li(&self, surface_point: &vec3::Vec3, wi: &mut vec3::Vec3) -> vec3::Vec3;
}
