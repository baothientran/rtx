use crate::core::vec3;
use crate::scene::fresnel;

pub struct Conductor {

}

impl Conductor {

}

impl fresnel::Fresnel for Conductor {
    fn evaluate(&self, dot_normal_wo: f32) -> vec3::Vec3 {
        todo!()
    }
}