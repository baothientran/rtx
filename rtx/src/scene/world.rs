use crate::scene::light;
use crate::scene::material;
use crate::scene::ray;
use crate::scene::shape;
use std::rc;

pub struct World {
    renderables: Vec<shape::RenderableShape>,
    lights: Vec<Box<dyn light::Light>>,
}

impl World {
    pub fn new() -> World {
        return World {
            renderables: Vec::<shape::RenderableShape>::new(),
            lights: Vec::<Box<dyn light::Light>>::new(),
        };
    }

    pub fn add_shape(
        &mut self,
        shape: rc::Rc<dyn shape::IntersectableShape>,
        material: rc::Rc<dyn material::Material>,
    ) {
        self.renderables
            .push(shape::RenderableShape::new(shape, material));
    }

    pub fn add_light(&mut self, light: Box<dyn light::Light>) {
        self.lights.push(light);
    }

    pub fn lights(&self) -> &Vec<Box<(dyn light::Light + 'static)>> {
        return &self.lights;
    }

    pub fn is_intersect(&self, ray: &ray::Ray, max_distance: f32) -> bool {
        for renderable in self.renderables.iter() {
            if renderable.is_intersect(ray, max_distance) {
                return true;
            }
        }

        return false;
    }

    pub fn intersect_ray(&self, ray: &ray::Ray) -> Option<shape::RenderableShapeSurface> {
        let mut intersect: Option<shape::RenderableShapeSurface> = None;
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
