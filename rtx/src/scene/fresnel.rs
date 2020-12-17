pub mod dielectrics;

pub trait Fresnel {
    fn evaluate(&self, dot_normal_wo: f32) -> f32;
}
