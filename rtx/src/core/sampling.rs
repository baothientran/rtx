use crate::core::math;
use crate::core::vec2;
use crate::core::vec3;

pub struct SphericalSample {
    pub theta: f32,
    pub phi: f32,
    pub position: vec3::Vec3,
    pub pdf: f32,
}

pub struct DiskSample {
    pub radius: f32,
    pub theta: f32,
    pub position: vec3::Vec3,
    pub pdf: f32,
}

pub fn sample_uniform_unit_sphere(sample: &vec2::Vec2) -> SphericalSample {
    let theta = f32::acos(sample.x);
    let phi = 2.0 * math::PI_F32 * sample.y;
    let position = vec3::Vec3::new(
        f32::cos(2.0 * math::PI_F32 * sample.y) * 2.0 * f32::sqrt(sample.x * (1.0 - sample.x)),
        f32::sin(2.0 * math::PI_F32 * sample.y) * 2.0 * f32::sqrt(sample.x * (1.0 - sample.x)),
        1.0 - 2.0 * sample.x,
    );
    let pdf = 1.0 / (4.0 * math::PI_F32);
    return SphericalSample {
        theta,
        phi,
        position,
        pdf,
    };
}

pub fn sample_uniform_unit_hemisphere(sample: &vec2::Vec2) -> SphericalSample {
    let theta = f32::acos(sample.x);
    let phi = 2.0 * math::PI_F32 * sample.y;
    let position = vec3::Vec3::new(
        f32::cos(2.0 * math::PI_F32 * sample.y) * f32::sqrt(1.0 - sample.x * sample.x),
        f32::sin(2.0 * math::PI_F32 * sample.y) * f32::sqrt(1.0 - sample.x * sample.x),
        sample.x,
    );
    let pdf = 1.0 / (2.0 * math::PI_F32);
    return SphericalSample {
        theta,
        phi,
        position,
        pdf,
    };
}

pub fn sample_uniform_unit_disk(sample: &vec2::Vec2) -> DiskSample {
    let radius = f32::sqrt(sample.x);
    let theta = 2.0 * math::PI_F32 * sample.y;
    let position = vec3::Vec3::new(radius * f32::cos(theta), radius * f32::sin(theta), 0.0);
    let pdf = 1.0 / math::PI_F32;
    return DiskSample {
        radius,
        theta,
        position,
        pdf,
    };
}

pub fn sample_concentric_unit_disk(sample: &vec2::Vec2) -> DiskSample {
    let radius;
    let theta;
    let offset = 2.0 * sample - vec2::Vec2::from(1.0);
    if offset.x == 0.0 || offset.y == 0.0 {
        radius = 0.0;
        theta = 0.0;
    } else if offset.x * offset.x > offset.y * offset.y {
        radius = offset.x;
        theta = math::PI_F32 * 0.25 * offset.y / offset.x;
    } else {
        radius = offset.y;
        theta = math::PI_F32 * 0.5 - math::PI_F32 * 0.25 * offset.x / offset.y;
    }

    let position = vec3::Vec3::new(radius * f32::cos(theta), radius * f32::sin(theta), 0.0);
    let pdf = 1.0 / math::PI_F32;
    return DiskSample {
        radius,
        theta,
        position,
        pdf
    };
}
