use vec2::Vector2;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct BoundRect<T> {
    pub min: Vector2<T>,
    pub max: Vector2<T>,
}

impl<T> BoundRect<T> {
    pub fn from_bounds(xmin: T, ymin: T, xmax: T, ymax: T) -> BoundRect<T> {
        BoundRect {
            min: Vector2::new(xmin, ymin),
            max: Vector2::new(xmax, ymax),
        }
    }
}

impl<T> BoundRect<T> {
    pub fn from_points(points: &[Vector2<T>]) -> Option<BoundRect<T>>
    where
        T: Copy + PartialOrd,
    {
        if let Some(&pf) = points.first() {
            let mut bound_box = BoundRect { min: pf, max: pf };

            for &p in points[1..].iter() {
                if p.x < bound_box.min.x {
                    bound_box.min.x = p.x;
                }
                if p.y < bound_box.min.y {
                    bound_box.min.y = p.y;
                }
                if p.x > bound_box.max.x {
                    bound_box.max.x = p.x;
                }
                if p.y > bound_box.max.y {
                    bound_box.max.y = p.y;
                }
            }

            Some(bound_box)
        } else {
            None
        }
    }

    pub fn intersection(self, rhs: BoundRect<T>) -> BoundRect<T>
    where
        T: PartialOrd,
    {
        let xmin = if self.min.x < rhs.min.x {
            rhs.min.x
        } else {
            self.min.x
        };

        let ymin = if self.min.y < rhs.min.y {
            rhs.min.y
        } else {
            self.min.y
        };

        let xmax = if self.max.x > rhs.max.x {
            rhs.max.x
        } else {
            self.max.x
        };

        let ymax = if self.max.y > rhs.max.y {
            rhs.max.y
        } else {
            self.max.y
        };

        BoundRect::from_bounds(xmin, ymin, xmax, ymax)
    }

    pub fn is_empty(&self) -> bool
    where
        T: PartialOrd,
    {
        !(self.min.x < self.max.x && self.min.y < self.max.y)
    }
}
