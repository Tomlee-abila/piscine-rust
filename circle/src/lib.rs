pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

use std::f64::consts::PI;

#[derive(Debug, Clone, Copy)]
pub struct Circle {
	pub center: Point,
	pub radius: f64
}

impl Circle {
    pub fn new(p1: f64, p2: f64, radius: f64)-> Circle{
        Circle{
            center: Point(p1, p2),
            radius
        }
    }

    pub fn diameter(&self) -> f64{
        self.radius*2.0
    }
    
    pub fn area(&self) -> f64{
         PI * self.radius.powi(2)
        
    }
    
    pub fn intersect(&self, c: Circle) -> bool{
        (self.center.0 - c.center.0).abs() < self.radius || (self.center.1 - c.center.1).abs() < self.radius
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Point(f64, f64);

impl Point {
    pub fn distance(&self, p: Point) -> f64{
        ((self.0 - p.0).powi(2) + (self.1 - p.1).powi(2)).sqrt()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
