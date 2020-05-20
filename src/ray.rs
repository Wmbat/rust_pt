use crate::Vector;

pub struct Ray {
    orig: Vector,
    dir: Vector,
}

impl Ray {
    pub fn new(origin: &Vector, direction: &Vector) -> Self {
        Ray {
            orig: origin.clone(),
            dir: direction.clone()
        }
    }

    pub fn origin(&self) -> &Vector {
        &self.orig
    }

    pub fn direction(&self) -> &Vector {
        &self.dir
    }

    pub fn position_along(&self, t: f64) -> Vector {
        *self.origin() + *self.direction() * t
    }
}
