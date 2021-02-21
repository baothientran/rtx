pub mod lambertian;
pub mod microfacet_reflection;
pub mod microfacet_refraction;
pub mod oren_nayar;
pub mod reflection;
pub mod refraction;

use crate::core::vec3;

pub struct ReflectanceRadiance {
    pub wi: vec3::Vec3,
    pub bxdf: vec3::Vec3,
}

pub struct ShadingReflectanceRadiance {
    pub shading_wi: vec3::Vec3,
    pub bxdf: vec3::Vec3,
}

pub enum ReflectanceType {
    Diffuse = 1 << 0,
    Microfacet = 1 << 1,
    Specular = 1 << 2,
    Reflection = 1 << 3,
    Refraction = 1 << 4,
}

impl ReflectanceType {
    pub fn contain(flags: u32, flag_to_check: u32) -> bool {
        return (flags & flag_to_check) == flag_to_check;
    }
}

pub trait Reflectance {
    fn has_types(&self, flags: u32) -> bool;

    fn bxdf(&self, shading_wo: &vec3::Vec3, shading_wi: &vec3::Vec3) -> vec3::Vec3;

    fn sample_bxdf(&self, shading_wo: &vec3::Vec3) -> Option<ShadingReflectanceRadiance>;
}

pub struct ReflectanceCollection {
    reflectances: Vec<Box<dyn Reflectance>>,
}

impl ReflectanceCollection {
    pub fn new() -> ReflectanceCollection {
        return ReflectanceCollection {
            reflectances: Vec::with_capacity(8),
        };
    }

    pub fn add(&mut self, reflectance: Box<dyn Reflectance>) {
        self.reflectances.push(reflectance);
    }

    pub fn clear(&mut self) {
        self.reflectances.clear();
    }

    pub fn bxdf(
        &self,
        normal: &vec3::Vec3,
        dpdu: &vec3::Vec3,
        wo: &vec3::Vec3,
        wi: &vec3::Vec3,
    ) -> vec3::Vec3 {
        let shading_x = vec3::Vec3::normalize(&dpdu).unwrap();
        let shading_y = vec3::Vec3::cross(&normal, &shading_x);
        let shading_wo = self.world_to_shading(&shading_x, &shading_y, normal, wo);
        if shading_wo.z == 0.0 {
            return vec3::Vec3::from(0.0);
        }

        let reflect = wo.dot(normal) * wi.dot(normal) > 0.0;
        let shading_wi = self.world_to_shading(&shading_x, &shading_y, normal, wi);
        let mut total_bxdf = vec3::Vec3::from(0.0);
        for reflectance in self.reflectances.iter() {
            if (reflect && reflectance.has_types(ReflectanceType::Reflection as u32))
                || (!reflect && reflectance.has_types(ReflectanceType::Refraction as u32))
            {
                total_bxdf += reflectance.bxdf(&shading_wo, &shading_wi);
            }
        }

        return total_bxdf;
    }

    pub fn sample_bxdf(
        &self,
        normal: &vec3::Vec3,
        dpdu: &vec3::Vec3,
        wo: &vec3::Vec3,
        flags: u32,
    ) -> Option<ReflectanceRadiance> {
        let mut bxdf_id = -1;
        for i in 0..self.reflectances.len() {
            if self.reflectances[i].has_types(flags) {
                bxdf_id = i as i32;
                break;
            }
        }

        if bxdf_id == -1 {
            return None;
        }

        let shading_x = vec3::Vec3::normalize(&dpdu).unwrap();
        let shading_y = vec3::Vec3::cross(&normal, &shading_x);
        let shading_wo = self.world_to_shading(&shading_x, &shading_y, normal, wo);
        if shading_wo.z == 0.0 {
            return None;
        }

        let maybe_radiance = self.reflectances[bxdf_id as usize].sample_bxdf(&shading_wo);
        if maybe_radiance.is_none() {
            return None;
        }

        let radiance = maybe_radiance.unwrap();
        let shading_wi = radiance.shading_wi;
        let mut bxdf = radiance.bxdf;
        let wi = self.shading_to_world(&shading_x, &shading_y, normal, &shading_wi);
        let reflect = wo.dot(normal) * wi.dot(normal) > 0.0;
        for i in 0..self.reflectances.len() {
            let reflectance = self.reflectances[i].as_ref();
            if (i != bxdf_id as usize) && (reflectance.has_types(flags)) {
                if (reflect && reflectance.has_types(ReflectanceType::Reflection as u32))
                    || (!reflect && reflectance.has_types(ReflectanceType::Refraction as u32))
                {
                    bxdf += reflectance.bxdf(&shading_wo, &shading_wi);
                }
            }
        }

        return Some(ReflectanceRadiance { wi, bxdf });
    }

    fn world_to_shading(
        &self,
        shading_x: &vec3::Vec3,
        shading_y: &vec3::Vec3,
        shading_z: &vec3::Vec3,
        v: &vec3::Vec3,
    ) -> vec3::Vec3 {
        return vec3::Vec3::new(
            vec3::Vec3::dot(shading_x, v),
            vec3::Vec3::dot(shading_y, v),
            vec3::Vec3::dot(shading_z, v),
        );
    }

    fn shading_to_world(
        &self,
        shading_x: &vec3::Vec3,
        shading_y: &vec3::Vec3,
        shading_z: &vec3::Vec3,
        v: &vec3::Vec3,
    ) -> vec3::Vec3 {
        return vec3::Vec3::new(
            shading_x.x * v.x + shading_y.x * v.y + shading_z.x * v.z,
            shading_x.y * v.x + shading_y.y * v.y + shading_z.y * v.z,
            shading_x.z * v.x + shading_y.z * v.y + shading_z.z * v.z,
        );
    }
}
