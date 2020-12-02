use crate::core::math;
use crate::core::vec3;
use crate::scene::ray;
use crate::scene::renderable;

pub struct Plane {
    normal: vec3::Vec3,
    distance: f32,
}

impl Plane {
    pub fn new(normal: vec3::Vec3, distance: f32) -> Plane {
        Plane {
            normal: vec3::Vec3::normalize(&normal).unwrap(),
            distance,
        }
    }

    pub fn normal(&self) -> &vec3::Vec3 {
        return &self.normal;
    }

    pub fn distance(&self) -> f32 {
        return self.distance;
    }

    fn _intersect_ray(&self, ray: &ray::Ray) -> Option<f32> {
        let vd = vec3::Vec3::dot(&self.normal, ray.direction());
        if math::equal_epsilon_f32(vd, 0.0, math::EPSILON_F32_6) {
            return None;
        }

        let vo = -vec3::Vec3::dot(&self.normal, ray.origin()) - self.distance;
        let t = vo / vd;
        if t <= 0.0 {
            return None;
        }

        return Some(t);
    }
}

impl renderable::Renderable for Plane {
    fn intersect_ray(&self, ray: &ray::Ray, surface_info: &mut renderable::SurfaceInfo) {
        match self._intersect_ray(ray) {
            Some(t) => {
                match surface_info {
                    renderable::SurfaceInfo::None => {
                        // flip normal if the plane normal points away from ray
                        let intersect_normal;
                        if vec3::Vec3::dot(ray.direction(), &self.normal) < 0.0 {
                            intersect_normal = self.normal;
                        } else {
                            intersect_normal = -self.normal;
                        }

                        *surface_info = renderable::SurfaceInfo::Hit {
                            ray_time: t,
                            position: ray.calc_position(t),
                            normal: intersect_normal,
                        };
                    },
                    renderable::SurfaceInfo::Hit {
                        ray_time,
                        position,
                        normal,
                    } => {
                        if *ray_time > t {
                            // flip normal if the plane normal points away from ray
                            let intersect_normal;
                            if vec3::Vec3::dot(ray.direction(), &self.normal) < 0.0 {
                                intersect_normal = self.normal;
                            } else {
                                intersect_normal = -self.normal;
                            }

                            *ray_time = t;
                            *position = ray.calc_position(t);
                            *normal = intersect_normal;
                        }
                    }
                };
            }
            None => {}
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::scene::renderable::Renderable;

    #[test]
    fn test_create_plane() {
        let plane = Plane::new(vec3::Vec3::new(2.0, 0.0, 0.0), 1.0);        
        let normal = plane.normal();
        let distance = plane.distance();
        assert!(math::equal_epsilon_f32(vec3::Vec3::length(normal), 1.0, math::EPSILON_F32_5));
        assert!(math::equal_epsilon_f32(normal.x, 1.0, math::EPSILON_F32_5));
        assert!(math::equal_epsilon_f32(normal.y, 0.0, math::EPSILON_F32_5));
        assert!(math::equal_epsilon_f32(normal.z, 0.0, math::EPSILON_F32_5));
        assert!(math::equal_epsilon_f32(distance, 1.0, math::EPSILON_F32_5));
    }

    #[test]
    fn test_intersect_ray() {
        let plane = Plane::new(vec3::Vec3::new(0.0, 1.0, 0.0), 0.2);

        let ray_origin = vec3::Vec3::new(0.0, 1.0, 1.0);
        let mut ray_direction = vec3::Vec3::from(0.0) - ray_origin;
        ray_direction = vec3::Vec3::normalize(&ray_direction).unwrap();
        let ray = ray::Ray::new(ray_origin, ray_direction);

        let intersect = plane._intersect_ray(&ray).unwrap();
        let position = ray.calc_position(intersect);

        let plane_func = vec3::Vec3::dot(&position, plane.normal()) + plane.distance();
        assert!(math::equal_epsilon_f32(plane_func, 0.0, math::EPSILON_F32_5));
    }

    #[test]
    fn test_intersect_ray_parallel() {
        let plane = Plane::new(vec3::Vec3::new(0.0, 1.0, 0.0), 0.2);

        let ray_origin = vec3::Vec3::new(0.0, 0.2, 1.0);
        let ray_direction = vec3::Vec3::new(0.0, 0.0, -1.0);
        let ray = ray::Ray::new(ray_origin, ray_direction);

        let intersect = plane._intersect_ray(&ray);
        assert!(intersect.is_none());
    }

    #[test]
    fn test_intersect_ray_no_intersect() {
        let plane = Plane::new(vec3::Vec3::new(0.0, 1.0, 0.0), 0.2);

        let ray_origin = vec3::Vec3::new(0.0, 0.6, 1.0);
        let mut ray_direction = vec3::Vec3::new(0.0, 1.0, 1.0);
        ray_direction = vec3::Vec3::normalize(&ray_direction).unwrap();
        let ray = ray::Ray::new(ray_origin, ray_direction);

        let intersect = plane._intersect_ray(&ray);
        assert!(intersect.is_none());
    }

    #[test]
    fn test_intersect_ray_renderable_no_intersect() {
        let plane = Plane::new(vec3::Vec3::new(0.0, 1.0, 0.0), 0.2);

        let ray_origin = vec3::Vec3::new(0.0, 0.6, 1.0);
        let mut ray_direction = vec3::Vec3::new(0.0, 1.0, 1.0);
        ray_direction = vec3::Vec3::normalize(&ray_direction).unwrap();
        let ray = ray::Ray::new(ray_origin, ray_direction);

        let mut surface_info = renderable::SurfaceInfo::None;
        plane.intersect_ray(&ray, &mut surface_info);
        assert!(surface_info.is_none());
    }

    #[test]
    fn test_intersect_ray_renderable_with_plane_point_away() {
        let plane = Plane::new(vec3::Vec3::new(0.0, 1.0, 0.0), 0.2);

        let ray_origin = vec3::Vec3::new(0.0, -4.0, 1.0);
        let mut ray_direction = vec3::Vec3::new(0.0, 1.0, 1.0);
        ray_direction = vec3::Vec3::normalize(&ray_direction).unwrap();
        let ray = ray::Ray::new(ray_origin, ray_direction);

        assert!(vec3::Vec3::dot(ray.direction(), plane.normal()) > 0.0);

        let mut surface_info = renderable::SurfaceInfo::None;
        plane.intersect_ray(&ray, &mut surface_info);
        match surface_info {
            renderable::SurfaceInfo::None => {assert!(false);}
            renderable::SurfaceInfo::Hit {ray_time: _, position, normal} => {
                let plane_func = vec3::Vec3::dot(&position, plane.normal()) + plane.distance();
                assert!(math::equal_epsilon_f32(plane_func, 0.0, math::EPSILON_F32_5));

                assert!(math::equal_epsilon_f32(normal.x, -plane.normal().x, math::EPSILON_F32_5));
                assert!(math::equal_epsilon_f32(normal.y, -plane.normal().y, math::EPSILON_F32_5));
                assert!(math::equal_epsilon_f32(normal.z, -plane.normal().z, math::EPSILON_F32_5));
            }
        }
    }

    #[test]
    fn test_intersect_ray_renderable_with_plane_point_toward() {
        let plane = Plane::new(vec3::Vec3::new(0.0, 1.0, 0.0), 0.2);

        let ray_origin = vec3::Vec3::new(1.0, 1.0, 1.0);
        let mut ray_direction = vec3::Vec3::from(0.0) - ray_origin;
        ray_direction = vec3::Vec3::normalize(&ray_direction).unwrap();
        let ray = ray::Ray::new(ray_origin, ray_direction);

        assert!(vec3::Vec3::dot(ray.direction(), plane.normal()) < 0.0);

        let mut surface_info = renderable::SurfaceInfo::None;
        plane.intersect_ray(&ray, &mut surface_info);
        match surface_info {
            renderable::SurfaceInfo::None => {assert!(false);}
            renderable::SurfaceInfo::Hit {ray_time: _, position, normal} => {
                let plane_func = vec3::Vec3::dot(&position, plane.normal()) + plane.distance();
                assert!(math::equal_epsilon_f32(plane_func, 0.0, math::EPSILON_F32_5));

                assert!(math::equal_epsilon_f32(normal.x, plane.normal().x, math::EPSILON_F32_5));
                assert!(math::equal_epsilon_f32(normal.y, plane.normal().y, math::EPSILON_F32_5));
                assert!(math::equal_epsilon_f32(normal.z, plane.normal().z, math::EPSILON_F32_5));
            }
        }
    }

    #[test]
    fn test_intersect_ray_renderable_with_plane_point_away_with_depth_success() {
        let plane = Plane::new(vec3::Vec3::new(0.0, 1.0, 0.0), 0.2);

        let ray_origin = vec3::Vec3::new(0.0, -4.0, 1.0);
        let mut ray_direction = vec3::Vec3::new(0.0, 1.0, 1.0);
        ray_direction = vec3::Vec3::normalize(&ray_direction).unwrap();
        let ray = ray::Ray::new(ray_origin, ray_direction);

        assert!(vec3::Vec3::dot(ray.direction(), plane.normal()) > 0.0);

        let mut surface_info = renderable::SurfaceInfo::Hit {
            ray_time: f32::MAX,
            position: vec3::Vec3::from(0.0),
            normal: vec3::Vec3::from(1.0),
        };
        plane.intersect_ray(&ray, &mut surface_info);
        match surface_info {
            renderable::SurfaceInfo::None => {assert!(false);}
            renderable::SurfaceInfo::Hit {ray_time: _, position, normal} => {
                let plane_func = vec3::Vec3::dot(&position, plane.normal()) + plane.distance();
                assert!(math::equal_epsilon_f32(plane_func, 0.0, math::EPSILON_F32_5));

                assert!(math::equal_epsilon_f32(normal.x, -plane.normal().x, math::EPSILON_F32_5));
                assert!(math::equal_epsilon_f32(normal.y, -plane.normal().y, math::EPSILON_F32_5));
                assert!(math::equal_epsilon_f32(normal.z, -plane.normal().z, math::EPSILON_F32_5));
            }
        }
    }

    #[test]
    fn test_intersect_ray_renderable_with_plane_point_toward_with_depth_success() {
        let plane = Plane::new(vec3::Vec3::new(0.0, 1.0, 0.0), 0.2);

        let ray_origin = vec3::Vec3::new(1.0, 1.0, 1.0);
        let mut ray_direction = vec3::Vec3::from(0.0) - ray_origin;
        ray_direction = vec3::Vec3::normalize(&ray_direction).unwrap();
        let ray = ray::Ray::new(ray_origin, ray_direction);

        assert!(vec3::Vec3::dot(ray.direction(), plane.normal()) < 0.0);

        let mut surface_info = renderable::SurfaceInfo::Hit {
            ray_time: f32::MAX,
            position: vec3::Vec3::from(0.0),
            normal: vec3::Vec3::from(1.0),
        };
        plane.intersect_ray(&ray, &mut surface_info);
        match surface_info {
            renderable::SurfaceInfo::None => {assert!(false);}
            renderable::SurfaceInfo::Hit {ray_time: _, position, normal} => {
                let plane_func = vec3::Vec3::dot(&position, plane.normal()) + plane.distance();
                assert!(math::equal_epsilon_f32(plane_func, 0.0, math::EPSILON_F32_5));

                assert!(math::equal_epsilon_f32(normal.x, plane.normal().x, math::EPSILON_F32_5));
                assert!(math::equal_epsilon_f32(normal.y, plane.normal().y, math::EPSILON_F32_5));
                assert!(math::equal_epsilon_f32(normal.z, plane.normal().z, math::EPSILON_F32_5));
            }
        }
    }

    #[test]
    fn test_intersect_ray_renderable_with_depth_fail() {
        let plane = Plane::new(vec3::Vec3::new(0.0, 1.0, 0.0), 0.2);

        let ray_origin = vec3::Vec3::new(0.0, -4.0, 1.0);
        let mut ray_direction = vec3::Vec3::new(0.0, 1.0, 1.0);
        ray_direction = vec3::Vec3::normalize(&ray_direction).unwrap();
        let ray = ray::Ray::new(ray_origin, ray_direction);

        assert!(vec3::Vec3::dot(ray.direction(), plane.normal()) > 0.0);

        let mut surface_info = renderable::SurfaceInfo::Hit {
            ray_time: f32::MIN,
            position: vec3::Vec3::from(0.0),
            normal: vec3::Vec3::from(1.0),
        };
        plane.intersect_ray(&ray, &mut surface_info);
        match surface_info {
            renderable::SurfaceInfo::None => {assert!(false);}
            renderable::SurfaceInfo::Hit {ray_time, position: _, normal: _} => {
                assert!(math::equal_epsilon_f32(ray_time, f32::MIN, math::EPSILON_F32_5));
            }
        }
    }
}
