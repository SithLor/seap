
struct Point {
    x: f64,
    y: f64,
}
trait Distance {
    fn distance(&self, other: &Self) -> f64;
}
trait Getter {
    fn get_x(&self) -> f64;
    fn get_y(&self) -> f64;
}
trait Setter {
    fn set_x(&mut self, x: f64);
    fn set_y(&mut self, y: f64);
}
trait Scale {
    fn scale_x_by(&mut self, factor: f64);
    fn scale_y_by(&mut self, factor: f64);
}

impl Setter for Point {
    fn set_x(&mut self, x: f64) {
        self.x = x;
    }
    fn set_y(&mut self, y: f64) {
        self.y = y;
    }
}
impl Distance for Point {
    fn distance(&self, other: &Self) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }
}
impl Getter for Point {
    fn get_x(&self) -> f64 {
        self.x
    }
    fn get_y(&self) -> f64 {
        self.y
    }
}
impl Scale for Point {
    fn scale_x_by(&mut self, factor: f64) {
        self.x *= factor;
    }
    fn scale_y_by(&mut self, factor: f64) {
        self.y *= factor;
    }
}


fn main() {
    let mut p1: Point = Point { x: 1.0, y: 2.0 };
    let p2: Point = Point { x: 4.0, y: 6.0 };
    println!("Distance between p1 and p2: {}", p1.distance(&p2));
    println!("p1.x: {}", p1.get_x());
    println!("p1.y: {}", p1.get_y());
    p1.scale_x_by(2.0);
    p1.scale_y_by(3.0);
    println!("p1.x: {}", p1.get_x());
    println!("p1.y: {}", p1.get_y());
}
