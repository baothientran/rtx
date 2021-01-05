use crate::core::vec3;
use crate::core::math;
use crate::scene::material;
use crate::scene::reflectance;
use crate::scene::reflectance::{lambertian, oren_nayar};

pub struct Matte {
    reflectances: reflectance::ReflectanceCollection,
}

impl Matte {
    pub fn new(kd: vec3::Vec3, sigma: f32) -> Matte {
        let mut reflectances = reflectance::ReflectanceCollection::new();

        if math::equal_epsilon_f32(sigma, 0.0, math::EPSILON_F32_6) {
            reflectances.add(Box::new(lambertian::Lambertian::new(kd)));
        }
        else {
            reflectances.add(Box::new(oren_nayar::OrenNayar::new(kd, sigma)));
        }

        return Matte { reflectances };
    }
}

impl material::Material for Matte {
    fn brdf(
        &self,
        normal: &vec3::Vec3,
        dpdu: &vec3::Vec3,
        wo: &vec3::Vec3,
        wi: &vec3::Vec3,
    ) -> vec3::Vec3 {
        return self.reflectances.brdf(normal, dpdu, wo, wi);
    }

    fn sample_brdf(
        &self,
        normal: &vec3::Vec3,
        dpdu: &vec3::Vec3,
        wo: &vec3::Vec3,
        wi: &mut vec3::Vec3,
        flags: u32,
    ) -> vec3::Vec3 {
        return self.reflectances.sample_brdf(normal, dpdu, wo, wi, flags);
    }
}
