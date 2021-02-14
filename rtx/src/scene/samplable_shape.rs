pub struct SampleSurface {
    pub pdf: f32,
    pub surface_point: vec3::Vec3,
    pub surface_normal: vec3::Vec3
}

impl SampleSurface {
    pub fn new(pdf: f32, surface_point: vec3::Vec3, surface_normal: vec3::Vec3) -> SampleSurface {
        return SampleSurface {
            pdf,
            surface_point,
            surface_normal
        };
    }
}

pub trait SamplableShape {
    fn pdf(
        &self,
        sample: &vec2::Vec2,
        surface_point_ref: &vec3::Vec3,
        surface_normal_ref: &vec3::Vec3,
    ) -> Option<SampleSurface>;
}