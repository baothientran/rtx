use crate::core::vec3;

pub struct Sphere {
    center: vec3::Vec3,
    radius: f32,
}

impl Sphere {
    pub fn new(center: vec3::Vec3, radius: f32) -> Sphere {
        Sphere { center, radius }
    }

    pub fn center(&self) -> &vec3::Vec3 {
        &self.center
    }

    pub fn radius(&self) -> f32 {
        self.radius
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::core::math;

    #[test]
    fn test_create_sphere() {
        let sphere = Sphere::new(vec3::Vec3::new(1.0, 2.0, 3.0), 32.0);
        let center = sphere.center();
        let radius = sphere.radius();
        assert!(math::equal_epsilon_f32(center.x, 1.0, math::EPSILON_F32_5));
        assert!(math::equal_epsilon_f32(center.y, 2.0, math::EPSILON_F32_5));
        assert!(math::equal_epsilon_f32(center.z, 3.0, math::EPSILON_F32_5));
        assert!(math::equal_epsilon_f32(radius, 32.0, math::EPSILON_F32_5));
    }
}
