use crate::core::vec3;
use crate::scene::ray;

pub enum SurfaceInfo {
    None,
    Hit {
        ray_time: f32,
        position: vec3::Vec3,
        normal: vec3::Vec3,
    },
}

impl SurfaceInfo {
    pub fn is_none(&self) -> bool {
        return match self {
            SurfaceInfo::None => true,
            _ => false
        };
    }
}

pub trait Renderable {
    fn intersect_ray(&self, ray: &ray::Ray, surface_info: &mut SurfaceInfo);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_none() {
        let mut surface_info = SurfaceInfo::None;
        assert!(surface_info.is_none());

        surface_info = SurfaceInfo::Hit {
            ray_time: 0.4,
            position: vec3::Vec3::from(1.0),
            normal: vec3::Vec3::new(0.0, 0.0, 1.0)
        };

        assert!(!surface_info.is_none());
    }
}