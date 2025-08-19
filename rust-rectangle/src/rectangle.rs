pub struct Rectangle {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

impl Rectangle {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Rectangle {
            x,
            y,
            width,
            height,
        }
    }

    pub fn area(&self) -> f32 {
        self.width * self.height
    }

    pub fn perimeter(&self) -> f32 {
        (self.width * 2.0) + (self.height * 2.0)
    }

    pub fn contains_point(&self, x: f32, y: f32) -> bool {
        x >= self.x && y >= self.y && x <= (self.x + self.width) && y <= (self.y + self.height)
    }

    pub fn print(&self) -> String {
        format!(
            "Rectangle [{} {} {} {}]",
            self.x, self.y, self.width, self.height
        )
    }
}
