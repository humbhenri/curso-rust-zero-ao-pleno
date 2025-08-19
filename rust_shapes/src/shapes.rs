use std::f32::consts::PI;
pub trait Shape {
    fn area(&self) -> f32;
    fn perimeter(&self) -> f32;
    fn draw(&self);
}

#[derive(Debug)]
pub struct Circle {
    pub diameter: f32,
}

impl Shape for Circle {
    fn area(&self) -> f32 {
        let r = self.diameter/2.0;
        PI * r * r
    }
    fn perimeter(&self) -> f32 {
        PI * self.diameter
    }
    fn draw(&self) {
        println!("{:?}", self);
    }
}
