use std::{fmt::{Formatter, Debug, Error}, ops::{Add, Sub, Mul, Div, Neg}};

/// General purpose two dimensional vector
#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct Vec2d<T> {
    pub x: T,
    pub y: T
}

impl<T: Debug> Debug for Vec2d<T> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "({:?}, {:?})", self.x, self.y)
    }
}

impl<T: Copy> Vec2d<T> {
    /// Create a new Vec2d instance
    /// # Examples
    ///
    /// ```
    /// use nanocv::Vec2d;
    /// let Vec2d = Vec2d::<usize>::new(1, 2);
    /// assert_eq!(Vec2d.x, 1);
    /// assert_eq!(Vec2d.y, 2);
    /// ```
    /// Add two Vec2ds
    /// ```
    /// # use nanocv::Vec2d;
    /// assert_eq!(Vec2d::new(1, 2) + Vec2d::new(2, 4), Vec2d::new(3, 6));
    /// ```
    /// Subtract two Vec2ds
    /// ```
    /// # use nanocv::Vec2d;
    /// assert_eq!(Vec2d::new(1, 2) - Vec2d::new(2, 4), Vec2d::new(-1, -2));
    /// ```
    /// Dot product
    /// ```
    /// # use nanocv::Vec2d;
    /// assert_eq!(Vec2d::new(1, 2)*Vec2d::new(2, 4), 10);
    /// ```
    /// Add a scalar value
    /// ```
    /// # use nanocv::Vec2d;
    /// assert_eq!(Vec2d::new(1, 2) + 1, Vec2d::new(2, 3));
    /// ```
    /// Subtract a scalar value
    /// ```
    /// # use nanocv::Vec2d;
    /// assert_eq!(Vec2d::new(1, 2) - 1, Vec2d::new(0, 1));
    /// ```
    /// Multiply by a scalar value
    /// ```
    /// # use nanocv::Vec2d;
    /// assert_eq!(Vec2d::new(1, 2)*2, Vec2d::new(2, 4));
    /// ```
    /// Divide by a scalar value
    /// ```
    /// # use nanocv::Vec2d;
    /// assert_eq!(Vec2d::new(2, 4)/2, Vec2d::new(1, 2));
    /// ```
    pub fn new(x: T, y: T) -> Vec2d<T> { 
        Vec2d { x, y } 
    }
}

impl<T: Mul<T, Output=T>> Vec2d<T> {
    /// Product of vector elements
    /// ```
    /// use nanocv::Vec2d;
    /// assert_eq!(Vec2d::new(3, 4).product(), 12);
    /// ```    
    pub fn product(self) -> T {
        self.x*self.y
    }
}

impl<T: Default> Default for Vec2d<T> {
    fn default() -> Self {
        Self { x: T::default(), y: T::default() }
    }
}

impl<T: Add<T, Output=T>> Add for Vec2d<T> {
    type Output = Vec2d<T>;

    fn add(self, other: Vec2d<T>) -> Vec2d<T> {
        Vec2d {x: self.x + other.x, y: self.y + other.y}
    }
}

impl<T: Sub<T, Output=T>> Sub for Vec2d<T> {
    type Output = Vec2d<T>;

    fn sub(self, other: Vec2d<T>) -> Vec2d<T> {
        Vec2d {x: self.x - other.x, y: self.y - other.y}
    }
}

impl<T: Mul<T, Output=T> + Add<T, Output=T>> Mul<Vec2d<T>> for Vec2d<T> {
    type Output = T;

    fn mul(self, other: Vec2d<T>) -> T {
        self.x*other.x + self.y*other.y
    }
}

impl<T: Neg<Output=T> + Copy> Neg for Vec2d<T> {
    type Output = Vec2d<T>;

    fn neg(self) -> Vec2d<T> {
        Vec2d {x: -(self.x), y: -(self.y) }
    }
}

impl<T: Add<T, Output=T> + Copy> Add<T> for Vec2d<T> {
    type Output = Vec2d<T>;

    fn add(self, scalar: T) -> Vec2d<T> {
        Vec2d {x: self.x + scalar, y: self.y + scalar}
    }
}

impl<T: Sub<T, Output=T> + Copy> Sub<T> for Vec2d<T> {
    type Output = Vec2d<T>;

    fn sub(self, scalar: T) -> Vec2d<T> {
        Vec2d {x: self.x - scalar, y: self.y - scalar}
    }
}

impl<T: Mul<T, Output=T> + Copy> Mul<T> for Vec2d<T> {
    type Output = Vec2d<T>;

    fn mul(self, scalar: T) -> Vec2d<T> {
        Vec2d {x: self.x*scalar, y: self.y*scalar}
    }
}

impl<T: Div<T, Output=T> + Copy> Div<T> for Vec2d<T> {
    type Output = Vec2d<T>;

    fn div(self, scalar: T) -> Vec2d<T> {
        Vec2d {x: self.x/scalar, y: self.y/scalar}
    }
}