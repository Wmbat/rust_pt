use std::ops;

#[derive(Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct Vector {
    data: [f64; 3],
}

impl Vector {
    pub fn new(x: f64, y: f64, z: f64) -> Vector {
        Vector {
            data: [x,y,z]
        }
    }

    pub fn x(&self) -> f64 {
        self.data[0]
    }

    pub fn y(&self) -> f64 {
        self.data[1]
    }

    pub fn z(&self) -> f64 {
        self.data[2]
    }

    pub fn length_squared(&self) -> f64 {
        self.x() * self.x() + self.y() * self.y() + self.z() * self.z()
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }
}

pub fn dot(lhs: &Vector, rhs: &Vector) -> f64 {
    lhs.x() * rhs.x() + lhs.y() * rhs.y() + lhs.z() * rhs.z()
}

pub fn cross(lhs: &Vector, rhs: &Vector) -> Vector {
    let x: f64 = lhs.y() * rhs.z() - lhs.z() * rhs.y();
    let y: f64 = lhs.z() * rhs.x() - lhs.x() * rhs.z();
    let z: f64 = lhs.x() * rhs.y() - lhs.y() * rhs.x();

    Vector::new(x, y, z)
}

pub fn normalise(vector: &Vector) -> Vector {
    let l: f64 = vector.length();

    Vector::new(vector.x() / l, vector.y() / l, vector.z() / l)
}

impl ops::Add<Vector> for Vector {
    type Output = Vector;

    fn add(self, rhs: Vector) -> Self::Output {
        Vector::new(self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z())
    }
}

impl ops::Sub<Vector> for Vector {
    type Output = Vector;

    fn sub(self, rhs: Vector) -> Self::Output {
        Vector::new(self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z())
    }
}

impl ops::Mul<Vector> for Vector {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Self::Output {
        Vector::new(self.x() * rhs.x(), self.y() * rhs.y(), self.z() * rhs.z())
    }
}

impl ops::Mul<f64> for Vector {
    type Output = Vector;

    fn mul(self, rhs: f64) -> Self::Output {
        Vector::new(self.x() * rhs, self.y() * rhs, self.z() * rhs)
    }
}

impl ops::Div<f64> for Vector {
    type Output = Vector;

    fn div(self, rhs: f64) -> Self::Output {
        let reciprocal: f64 = 1.0 / rhs;
    
        self * reciprocal
    }
}

impl ops::Neg for Vector {
    type Output = Vector; 

    fn neg(self) -> Self::Output {
        Vector::new(-self.x(), -self.y(), -self.z())
    }
}

impl std::fmt::Display for Vector {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} {} {}", self.x(), self.y(), self.z())
    }
}
