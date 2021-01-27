use crate::scene::light;
use crate::scene::material;
use crate::scene::ray;
use crate::scene::renderable;
use crate::scene::shape;
use std::rc;
use std::slice;

pub struct World {
    renderables: Vec<renderable::Renderable>,
    lights: Vec<Box<dyn light::Light>>,
}

impl World {
    pub fn new() -> World {
        return World {
            renderables: Vec::<renderable::Renderable>::new(),
            lights: Vec::<Box<dyn light::Light>>::new(),
        };
    }

    pub fn add_shape(
        &mut self,
        shape: rc::Rc<dyn shape::Shape>,
        material: rc::Rc<dyn material::Material>,
    ) {
        self.renderables
            .push(renderable::Renderable::new(shape, material));
    }

    pub fn add_light(&mut self, light: Box<dyn light::Light>) {
        self.lights.push(light);
    }

    pub fn lights(&self) -> slice::Iter<'_, std::boxed::Box<dyn light::Light>> {
        return self.lights.iter();
    }

    pub fn is_intersect(&self, ray: &ray::Ray, max_distance: f32) -> bool {
        for renderable in self.renderables.iter() {
            if renderable.is_intersect(ray, max_distance) {
                return true;
            }
        }

        return false;
    }

    pub fn intersect_ray(&self, ray: &ray::Ray) -> Option<renderable::RenderableSurface> {
        let mut intersect: Option<renderable::RenderableSurface> = None;
        for renderable in self.renderables.iter() {
            match intersect.as_mut() {
                Some(curr_renderable_surface) => {
                    if let Some(potential_renderable_surface) = renderable.intersect_ray(&ray) {
                        if curr_renderable_surface.shape_surface().ray_time()
                            > potential_renderable_surface.shape_surface().ray_time()
                        {
                            *curr_renderable_surface = potential_renderable_surface;
                        }
                    }
                }
                None => intersect = renderable.intersect_ray(&ray),
            }
        }

        return intersect;
    }
}
