#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector2D {
    pub x: f32,
    pub y: f32,
}

impl Vector2D {
    pub fn new(x: f32, y: f32) -> Vector2D {
        Vector2D { x, y }
    }

    /// Create a new unit vector from an angle in radians
    ///
    /// # Arguments
    ///
    /// * `angle` - The angle in radians
    ///
    /// # Examples
    ///
    /// ```
    /// use math::Vector2D;
    ///
    /// let v = Vector2D::from_angle(std::f32::consts::PI);
    /// assert_eq!(v.x, -1.0);
    /// assert_eq!(v.y, 0.0);
    /// assert_eq!(v.magnitude(), 1.0);
    /// ```
    /// ```
    /// use math::Vector2D;
    ///
    /// let v = Vector2D::from_angle(std::f32::consts::PI / 2.0) * 3.0;
    /// assert_eq!(v.x, 0.0);
    /// assert_eq!(v.y, 3.0);
    /// assert_eq!(v.magnitude(), 3.0);
    /// ```
    pub fn from_angle(angle: f32) -> Vector2D {
        Vector2D {
            x: angle.cos(),
            y: angle.sin(),
        }
    }

    pub fn magnitude(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn distance(&self, other: Vector2D) -> f32 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }

    pub fn angle(&self) -> f32 {
        self.y.atan2(self.x)
    }

    pub fn angle_between(&self, other: Vector2D) -> f32 {
        self.angle() - other.angle()
    }

    pub fn angle_to(&self, other: Vector2D) -> f32 {
        (other - *self).angle()
    }

    pub fn normalize(&mut self) {
        let mag = self.magnitude();
        self.x /= mag;
        self.y /= mag;
    }

    pub fn normalized(&self) -> Vector2D {
        let mag = self.magnitude();
        Vector2D {
            x: self.x / mag,
            y: self.y / mag,
        }
    }

    pub fn limit(&mut self, max: f32) {
        if self.magnitude() > max {
            self.normalize();
            self.x *= max;
            self.y *= max;
        }
    }

    pub fn limited(&self, max: f32) -> Vector2D {
        let mut v = self.normalized();
        if v.magnitude() > max {
            v.x *= max;
            v.y *= max;
        }
        v
    }

    pub fn rotate(&mut self, angle: f32) {
        let new_x = self.x * angle.cos() - self.y * angle.sin();
        let new_y = self.x * angle.sin() + self.y * angle.cos();
        self.x = new_x;
        self.y = new_y;
    }

    pub fn rotated(&self, angle: f32) -> Vector2D {
        Vector2D {
            x: self.x * angle.cos() - self.y * angle.sin(),
            y: self.x * angle.sin() + self.y * angle.cos(),
        }
    }

    pub fn dot(&self, other: Vector2D) -> f32 {
        self.x * other.x + self.y * other.y
    }
}

impl std::ops::Add for Vector2D {
    type Output = Vector2D;

    fn add(self, other: Vector2D) -> Vector2D {
        Vector2D {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl std::ops::Sub for Vector2D {
    type Output = Vector2D;

    fn sub(self, other: Vector2D) -> Vector2D {
        Vector2D {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl std::ops::Mul<f32> for Vector2D {
    type Output = Vector2D;

    fn mul(self, scalar: f32) -> Vector2D {
        Vector2D {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

impl std::ops::Div<f32> for Vector2D {
    type Output = Vector2D;

    fn div(self, scalar: f32) -> Vector2D {
        Vector2D {
            x: self.x / scalar,
            y: self.y / scalar,
        }
    }
}

impl std::ops::AddAssign for Vector2D {
    fn add_assign(&mut self, other: Vector2D) {
        *self = Vector2D {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

impl std::ops::SubAssign for Vector2D {
    fn sub_assign(&mut self, other: Vector2D) {
        *self = Vector2D {
            x: self.x - other.x,
            y: self.y - other.y,
        };
    }
}

impl std::ops::MulAssign<f32> for Vector2D {
    fn mul_assign(&mut self, scalar: f32) {
        *self = Vector2D {
            x: self.x * scalar,
            y: self.y * scalar,
        };
    }
}

impl std::ops::DivAssign<f32> for Vector2D {
    fn div_assign(&mut self, scalar: f32) {
        *self = Vector2D {
            x: self.x / scalar,
            y: self.y / scalar,
        };
    }
}
