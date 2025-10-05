// Возьмите код из предыдущего занятия.
// И теперь представим, что список фигур для которых мы хотим выполнить вычисления
// неизвестен на этапе компиляции программы.

// Исправьте фунцию perimeter_by_area, так чтобы она принимала параметр Box<dyn Shape>

struct Triangle {
    sides_lens: [f64; 3],
}

struct Rectangle {
    width: f64,
    height: f64,
}

struct Circle {
    radius: f64,
}

trait Shape {
    fn get_area(&self) -> f64;
    fn get_perimeter(&self) -> f64;
}

impl Shape for Triangle {
    fn get_area(&self) -> f64 {
        let p = self.get_perimeter() / 2f64;

        (p * (p - self.sides_lens[0]) * (p - self.sides_lens[1]) * (p - self.sides_lens[2])).sqrt()
    }

    fn get_perimeter(&self) -> f64 {
        self.sides_lens.iter().sum()
    }
}

impl Shape for Rectangle {
    fn get_area(&self) -> f64 {
        self.height * self.width
    }

    fn get_perimeter(&self) -> f64 {
        (self.width + self.height) * 2f64
    }
}

impl Shape for Circle {
    fn get_area(&self) -> f64 {
        std::f64::consts::PI * self.radius.powf(2f64)
    }

    fn get_perimeter(&self) -> f64 {
        2f64 * std::f64::consts::PI * self.radius
    }
}

fn perimeter_by_area(shape: Box<dyn Shape>) -> f64 {
    shape.get_perimeter() / shape.get_area()
}

fn main() {
    let a = Triangle {
        sides_lens: [3.0, 4.0, 5.0],
    };
    let b = Rectangle {
        width: 10.0,
        height: 4.0,
    };
    let c = Circle { radius: 2.0 };

    println!("{}", perimeter_by_area(Box::new(a)));
    println!("{}", perimeter_by_area(Box::new(b)));
    println!("{}", perimeter_by_area(Box::new(c)));
}

#[cfg(test)]
#[allow(unused_must_use)]
mod tests {
    use super::*;
    use approx::relative_eq;

    #[test]
    fn test() {
        relative_eq!(
            perimeter_by_area(Box::new(Triangle {
                sides_lens: [3.0, 4.0, 5.0]
            })),
            2.0
        );
        relative_eq!(perimeter_by_area(Box::new(Circle { radius: 2.0 })), 1.0);
        relative_eq!(
            perimeter_by_area(Box::new(Rectangle {
                width: 2.0,
                height: 3.0,
            })),
            1.6666
        );
    }
}
