use crate::core::math;
use crate::core::vec2;
use crate::core::vec3;

pub fn sample_uniform_unit_disk(sample: &vec2::Vec2) -> vec3::Vec3 {
    let radius = f32::sqrt(sample.x);
    let theta = 2.0 * math::PI_F32 * sample.y;
    let position = vec3::Vec3::new(radius * f32::cos(theta), radius * f32::sin(theta), 0.0);
    return position;
}

pub fn sample_concentric_unit_disk(sample: &vec2::Vec2) -> vec3::Vec3 {
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
    return position;
}

pub fn pdf_uniform_unit_disk() -> f32 {
    return 1.0 / math::PI_F32;
}

pub fn sample_uniform_unit_sphere(sample: &vec2::Vec2) -> vec3::Vec3 {
    let position = vec3::Vec3::new(
        f32::cos(2.0 * math::PI_F32 * sample.y) * 2.0 * f32::sqrt(sample.x * (1.0 - sample.x)),
        f32::sin(2.0 * math::PI_F32 * sample.y) * 2.0 * f32::sqrt(sample.x * (1.0 - sample.x)),
        1.0 - 2.0 * sample.x,
    );
    return position;
}

pub fn pdf_uniform_unit_sphere() -> f32 {
    return 1.0 / (4.0 * math::PI_F32);
}

pub fn sample_uniform_unit_hemisphere(sample: &vec2::Vec2) -> vec3::Vec3 {
    let position = vec3::Vec3::new(
        f32::cos(2.0 * math::PI_F32 * sample.y) * f32::sqrt(1.0 - sample.x * sample.x),
        f32::sin(2.0 * math::PI_F32 * sample.y) * f32::sqrt(1.0 - sample.x * sample.x),
        sample.x,
    );
    return position;
}

pub fn pdf_uniform_unit_hemisphere() -> f32 {
    return 1.0 / (2.0 * math::PI_F32);
}

pub fn sample_cosine_weighted_unit_hemisphere(sample: &vec2::Vec2) -> vec3::Vec3 {
    let disk_sample = sample_concentric_unit_disk(sample);
    let x_sq = disk_sample.x * disk_sample.x;
    let y_sq = disk_sample.y * disk_sample.y;
    let z = f32::sqrt(1.0 - x_sq - y_sq);

    let position = vec3::Vec3::new(disk_sample.x, disk_sample.y, z);
    return position;
}

pub fn pdf_cosine_weighted_unit_hemisphere(cos_theta: f32) -> f32 {
    assert!(cos_theta > 0.0);
    return cos_theta / math::PI_F32;
}