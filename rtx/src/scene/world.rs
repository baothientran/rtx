use crate::scene::light;
use crate::scene::ray;
use crate::scene::renderable;
use crate::scene::shape;

pub struct World {
    renderables: Vec<renderable::Renderable>,
    lights: Vec<Box<dyn light::Light>>,
}

impl World {
    pub fn new() -> World {
        World {
            renderables: Vec::<renderable::Renderable>::new(),
            lights: Vec::<Box<dyn light::Light>>::new(),
        }
    }

    pub fn add_shape(&mut self, shape: Box<dyn shape::Shape>) {
        self.renderables.push(renderable::Renderable::new(shape));
    }

    pub fn add_light(&mut self, light: Box<dyn light::Light>) {
        self.lights.push(light);
    }

    pub fn intersect_ray(&self, ray: &ray::Ray) -> Option<renderable::RenderableSurface> {
        let mut intersect: Option<renderable::RenderableSurface> = None;
        for renderable in self.renderables.iter() {
            match intersect.as_mut() {
                Some(surface) => {
                    if let Some(renderable_surface) = renderable.intersect_ray(&ray) {
                        if surface.ray_time() > renderable_surface.ray_time() {
                            *surface = renderable_surface;
                        }
                    }
                }
                None => intersect = renderable.intersect_ray(&ray),
            }
        }

        return intersect;
    }
}
