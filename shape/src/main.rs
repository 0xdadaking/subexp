use std::fmt::Display;

pub trait Shape: Display {
    fn perimeter(&self) -> f32;
    fn area(&self) -> f32;
}

struct Rectangle {
    pub width: f32,
    pub height: f32,
}

impl Shape for Rectangle {
    fn perimeter(&self) -> f32 {
        self.width * 2.0 + self.height * 2.0
    }

    fn area(&self) -> f32 {
        self.width * self.height
    }
}

impl Display for Rectangle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Rectangle(w:{}, h:{})", self.width, self.height)
    }
}

struct Triangle {
    pub side: f32,
}

impl Shape for Triangle {
    fn perimeter(&self) -> f32 {
        self.side * 3.0_f32
    }

    fn area(&self) -> f32 {
        self.side * 0.5 * 3.0_f32.sqrt() / 2.0 * self.side
    }
}

impl Display for Triangle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Triangle(side:{})", self.side)
    }
}

struct Circle {
    pub radius: f32,
}

use std::f32::consts::PI;

impl Shape for Circle {
    fn perimeter(&self) -> f32 {
        self.radius * 2.0 * PI
    }

    fn area(&self) -> f32 {
        self.radius * self.radius * PI
    }
}

impl Display for Circle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Circle(r:{})", self.radius)
    }
}

fn print_shapes(shapes: Vec<&dyn Shape>) {
    for s in shapes {
        println!(
            "shape:{}， perimeter:{}, area:{}",
            s,
            s.perimeter(),
            s.area()
        );
    }
}

fn main() {
    let shapes: Vec<&dyn Shape> = vec![
        &Triangle { side: 23.0 },
        &Circle { radius: 50.0 },
        &Rectangle {
            width: 30.0,
            height: 60.0,
        },
    ];

    print_shapes(shapes);
}
