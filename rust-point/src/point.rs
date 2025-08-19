pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub fn new(_x: f32, _y: f32) -> Self {
        Point { x: _x, y: _y }
    }

    pub fn distance(&self, p: &Self) -> f32 {
        let Point { x: x1, y: y1 } = self;
        let Point { x: x2, y: y2 } = p;
        ((x2 - x1).powf(2.0f32) + (y2 - y1).powf(2.0f32)).sqrt()
    }

    pub fn move_to(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }

    pub fn print(&self) -> String {
        format!("Point [{} {}]", self.x, self.y)
    }
}
