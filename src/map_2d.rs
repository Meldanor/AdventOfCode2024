pub struct Map2D<T> {
    pub height: usize,
    pub width: usize,
    pub points: Vec<Vec<T>>,
}

impl<T> Map2D<T> {
    pub fn new(width: usize, height: usize) -> Self {
        Map2D {
            height,
            width,
            points: Vec::new(),
        }
    }

    pub fn get_at(&self, x: isize, y: isize) -> Option<&T> {
        if x < 0 || y < 0 || x as usize >= self.width || y as usize >= self.height {
            return None;
        }
        return Some(&self.points[y as usize][x as usize]);
    }

    pub fn set_at(&mut self, x: isize, y: isize, value: T) {
        self.points[y as usize][x as usize] = value;
    }
}
