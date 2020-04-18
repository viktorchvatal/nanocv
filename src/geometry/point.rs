use std::{fmt::{Formatter, Debug, Error}, ops::{Add, Sub, Mul, Div, Neg}};

/// General purpose two dimensional vector
#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct Point<T> {
    pub x: T,
    pub y: T
}

impl<T: Debug> Debug for Point<T> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "({:?}, {:?})", self.x, self.y)
    }
}

impl<T: Copy> Point<T> {
    /// Create a new Point instance
    /// # Examples
    ///
    /// ```
    /// use nanocv::Point;
    /// let point = Point::<usize>::new(1, 2);
    /// assert_eq!(point.x, 1);
    /// assert_eq!(point.y, 2);
    /// ```
    /// Add two points
    /// ```
    /// # use nanocv::Point;
    /// assert_eq!(Point::new(1, 2) + Point::new(2, 4), Point::new(3, 6));
    /// ```
    /// Subtract two points
    /// ```
    /// # use nanocv::Point;
    /// assert_eq!(Point::new(1, 2) - Point::new(2, 4), Point::new(-1, -2));
    /// ```
    /// Dot product
    /// ```
    /// # use nanocv::Point;
    /// assert_eq!(Point::new(1, 2)*Point::new(2, 4), 10);
    /// ```
    /// Add a scalar value
    /// ```
    /// # use nanocv::Point;
    /// assert_eq!(Point::new(1, 2) + 1, Point::new(2, 3));
    /// ```
    /// Subtract a scalar value
    /// ```
    /// # use nanocv::Point;
    /// assert_eq!(Point::new(1, 2) - 1, Point::new(0, 1));
    /// ```
    /// Multiply by a scalar value
    /// ```
    /// # use nanocv::Point;
    /// assert_eq!(Point::new(1, 2)*2, Point::new(2, 4));
    /// ```
    /// Divide by a scalar value
    /// ```
    /// # use nanocv::Point;
    /// assert_eq!(Point::new(2, 4)/2, Point::new(1, 2));
    /// ```
    pub fn new(x: T, y: T) -> Point<T> { 
        Point { x, y } 
    }
}

impl<T: Mul<T, Output=T>> Point<T> {
    /// Product of vector elements
    /// ```
    /// use nanocv::Point;
    /// assert_eq!(Point::new(3, 4).product(), 12);
    /// ```    
    pub fn product(self) -> T {
        self.x*self.y
    }
}

impl<T: Default> Default for Point<T> {
    fn default() -> Self {
        Self { x: T::default(), y: T::default() }
    }
}

impl<T: Add<T, Output=T>> Add for Point<T> {
    type Output = Point<T>;

    fn add(self, other: Point<T>) -> Point<T> {
        Point {x: self.x + other.x, y: self.y + other.y}
    }
}

impl<T: Sub<T, Output=T>> Sub for Point<T> {
    type Output = Point<T>;

    fn sub(self, other: Point<T>) -> Point<T> {
        Point {x: self.x - other.x, y: self.y - other.y}
    }
}

impl<T: Mul<T, Output=T> + Add<T, Output=T>> Mul<Point<T>> for Point<T> {
    type Output = T;

    fn mul(self, other: Point<T>) -> T {
        self.x*other.x + self.y*other.y
    }
}

impl<T: Neg<Output=T> + Copy> Neg for Point<T> {
    type Output = Point<T>;

    fn neg(self) -> Point<T> {
        Point {x: -(self.x), y: -(self.y) }
    }
}

impl<T: Add<T, Output=T> + Copy> Add<T> for Point<T> {
    type Output = Point<T>;

    fn add(self, scalar: T) -> Point<T> {
        Point {x: self.x + scalar, y: self.y + scalar}
    }
}

impl<T: Sub<T, Output=T> + Copy> Sub<T> for Point<T> {
    type Output = Point<T>;

    fn sub(self, scalar: T) -> Point<T> {
        Point {x: self.x - scalar, y: self.y - scalar}
    }
}

impl<T: Mul<T, Output=T> + Copy> Mul<T> for Point<T> {
    type Output = Point<T>;

    fn mul(self, scalar: T) -> Point<T> {
        Point {x: self.x*scalar, y: self.y*scalar}
    }
}

impl<T: Div<T, Output=T> + Copy> Div<T> for Point<T> {
    type Output = Point<T>;

    fn div(self, scalar: T) -> Point<T> {
        Point {x: self.x/scalar, y: self.y/scalar}
    }
}