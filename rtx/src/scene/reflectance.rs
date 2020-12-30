pub mod lambertian;
pub mod oren_nayar;
pub mod reflection;
pub mod refraction;

use crate::core::vec3;

pub enum ReflectanceType {
    Lambertian = 1 << 0,
    Reflection = 1 << 1,
    Refraction = 1 << 2,
    Microfacet = 1 << 3,
}

impl ReflectanceType {
    pub fn contain(flags: u32, flag_to_check: u32) -> bool {
        return (flags & flag_to_check) != 0;
    }
}

pub trait Reflectance {
    fn has_types(&self, flags: u32) -> bool;

    fn brdf(&self, normal: &vec3::Vec3, wo: &vec3::Vec3, wi: &vec3::Vec3) -> vec3::Vec3;

    fn sample_brdf(&self, normal: &vec3::Vec3, wo: &vec3::Vec3, wi: &mut vec3::Vec3) -> vec3::Vec3;
}
