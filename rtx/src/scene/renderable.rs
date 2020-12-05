use crate::scene::shape;

pub struct Renderable {
    shape: Box<dyn shape::Shape>,
}

impl Renderable {
    pub fn new(shape: Box<dyn shape::Shape>) -> Renderable {
        return Renderable { shape };
    }

    pub fn shape(&self) -> &dyn shape::Shape {
        return self.shape.as_ref();
    }
}
