#[macro_export]
macro_rules! p {
    ($x:expr, $y:expr) => {
        Point { x:$x, y:$y }
    };
    ($x:literal, $y:literal) => {
        Point { x:$x, y:$y }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq,)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}

impl Point {
    /// interpolates all values between 2 points
    pub fn interp(&self, other: Point) -> Vec<f32> {
        let mut result = vec![];
        if self.x == other.x {
            result.push(self.y as f32);
            return result;
        }

        let a = (other.y as f32 - self.y as f32) / (other.x as f32 - self.x as f32);
        let mut y = self.y as f32;
        for _ in self.x..other.x {
            result.push(y);
            y += a;
        }

        result
    }

    /// Returns a new point with self's x and y swapped
    pub fn invert(&self) -> Self {
        Self { x:self.y, y:self.x }
    }

    /// Swaps x and y in place
    pub fn invert_in_place(&mut self) {
        std::mem::swap(&mut self.x, &mut self.y)
    }
}

impl From<[u32; 2]> for Point {
    fn from(value: [u32; 2]) -> Self {
        Self { x:value[0], y:value[1] }
    }
}

impl From<(u32, u32)> for Point {
    fn from(value: (u32, u32)) -> Self {
        Self { x:value.0, y:value.1 }
    }
}